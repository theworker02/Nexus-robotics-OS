//! Stable, hardware-neutral domain contracts for Nexus Robotics OS.

use std::collections::{BTreeMap, BTreeSet};

pub mod v2;
pub mod v25;

/// A machine-readable claim about what a robot can do.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Capability(pub String);

impl Capability {
    /// Builds a normalized dotted capability path.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into().to_ascii_lowercase())
    }
}

/// A capability manifest attached to a robot identity.
#[derive(Clone, Debug)]
pub struct CapabilityManifest {
    pub robot_id: String,
    pub name: String,
    pub architecture: String,
    pub capabilities: BTreeSet<Capability>,
}

impl CapabilityManifest {
    /// Checks an exact capability or an alias understood by Nexus.
    pub fn supports(&self, requested: &str) -> bool {
        let requested = requested.to_ascii_lowercase();
        self.capabilities
            .iter()
            .any(|candidate| candidate.0 == requested)
            || capability_aliases(&requested)
                .iter()
                .any(|alias| self.capabilities.contains(&Capability::new(*alias)))
    }

    /// Returns all capability requirements this manifest cannot satisfy.
    pub fn missing<'a>(&self, requirements: impl IntoIterator<Item = &'a str>) -> Vec<String> {
        requirements
            .into_iter()
            .filter(|item| !self.supports(item))
            .map(str::to_owned)
            .collect()
    }
}

fn capability_aliases(requested: &str) -> &'static [&'static str] {
    match requested {
        "vision.depth" => &["vision.rgbd", "vision.stereo_depth", "sensing.lidar_depth"],
        "vision.rgb" => &["vision.rgbd"],
        "manipulators.right.gripper" => &["manipulators.gripper"],
        "manipulators.left.gripper" => &["manipulators.gripper"],
        "locomotion.control" => &["locomotion.biped", "locomotion.wheeled"],
        _ => &[],
    }
}

/// A reusable, declarative query for capability compatibility.
#[derive(Clone, Debug)]
pub struct CapabilityQuery {
    pub requirements: Vec<String>,
}

impl CapabilityQuery {
    pub fn evaluate(&self, manifest: &CapabilityManifest) -> Compatibility {
        let missing = manifest.missing(self.requirements.iter().map(String::as_str));
        Compatibility {
            compatible: missing.is_empty(),
            missing,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Compatibility {
    pub compatible: bool,
    pub missing: Vec<String>,
}

/// Safety is deliberately a state machine, never a boolean flag.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SafetyState {
    Safe,
    Caution,
    Limited,
    EmergencyStop,
    Fault,
}

impl SafetyState {
    pub fn label(self) -> &'static str {
        match self {
            Self::Safe => "SAFE",
            Self::Caution => "CAUTION",
            Self::Limited => "LIMITED",
            Self::EmergencyStop => "EMERGENCY_STOP",
            Self::Fault => "FAULT",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Health {
    Healthy,
    Degraded,
    Maintenance,
    Fault,
    Offline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotLifecycle {
    Provisioning,
    Connecting,
    Ready,
    Executing,
    Paused,
    Degraded,
    EmergencyStop,
    Offline,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskLifecycle {
    Queued,
    Planning,
    Ready,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

/// Pose in the simulator's local coordinate frame (metres, radians).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pose {
    pub x: f32,
    pub y: f32,
    pub yaw: f32,
}

/// Canonical state exposed by all adapters.
#[derive(Clone, Debug)]
pub struct RobotState {
    pub identity: String,
    pub capabilities: CapabilityManifest,
    pub sensors: BTreeMap<String, String>,
    pub actuators: BTreeMap<String, f32>,
    pub health: Health,
    pub battery_percent: f32,
    pub network_connected: bool,
    pub pose: Pose,
    pub current_task: Option<String>,
    pub current_skill: Option<String>,
    pub safety_state: SafetyState,
    pub lifecycle: RobotLifecycle,
}

/// A physical command guarded by the safety governor.
#[derive(Clone, Debug, PartialEq)]
pub enum ActuatorCommand {
    SetJoint { joint: String, angle_deg: f32 },
    Move { speed_mps: f32, duration_s: f32 },
    Stop,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SafetyViolation {
    EmergencyStopActive,
    JointLimit {
        joint: String,
        requested: f32,
        min: f32,
        max: f32,
    },
    SpeedLimit {
        requested: f32,
        max: f32,
    },
    RestrictedZone {
        zone: String,
    },
}

/// Deterministic command guard shared by every adapter.
#[derive(Clone, Debug)]
pub struct SafetyGovernor {
    pub state: SafetyState,
    pub max_speed_mps: f32,
    pub joint_limits: BTreeMap<String, (f32, f32)>,
}

impl SafetyGovernor {
    pub fn validate(
        &self,
        command: &ActuatorCommand,
        zone: Option<&str>,
    ) -> Result<(), SafetyViolation> {
        if self.state == SafetyState::EmergencyStop && !matches!(command, ActuatorCommand::Stop) {
            return Err(SafetyViolation::EmergencyStopActive);
        }
        if let Some(zone) = zone {
            if zone == "no-entry" {
                return Err(SafetyViolation::RestrictedZone { zone: zone.into() });
            }
        }
        match command {
            ActuatorCommand::SetJoint { joint, angle_deg } => {
                if let Some((min, max)) = self.joint_limits.get(joint) {
                    if angle_deg < min || angle_deg > max {
                        return Err(SafetyViolation::JointLimit {
                            joint: joint.clone(),
                            requested: *angle_deg,
                            min: *min,
                            max: *max,
                        });
                    }
                }
            }
            ActuatorCommand::Move { speed_mps, .. } if speed_mps.abs() > self.max_speed_mps => {
                return Err(SafetyViolation::SpeedLimit {
                    requested: *speed_mps,
                    max: self.max_speed_mps,
                });
            }
            _ => {}
        }
        Ok(())
    }
    pub fn emergency_stop(&mut self) {
        self.state = SafetyState::EmergencyStop;
    }
    pub fn reset(&mut self) {
        if self.state == SafetyState::EmergencyStop {
            self.state = SafetyState::Safe;
        }
    }
}

/// A node in an inspectable deterministic task graph.
#[derive(Clone, Debug, PartialEq)]
pub enum TaskNode {
    Action(String),
    Condition(String),
    Wait(String),
    Branch(String),
    Recovery(String),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaskGraph {
    pub task_id: String,
    pub nodes: Vec<TaskNode>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructuredLog {
    pub timestamp_ms: u128,
    pub robot_id: String,
    pub task_id: Option<String>,
    pub skill_id: Option<String>,
    pub component: String,
    pub severity: Severity,
    pub message: String,
    pub context: BTreeMap<String, String>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Severity {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    RobotConnected,
    TaskStarted(String),
    TaskCompleted(String),
    TaskFailed(String),
    SkillStarted(String),
    SkillFailed(String),
    SafetyTriggered,
    BatteryLow,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rgbd_satisfies_depth_and_rgb() {
        let m = CapabilityManifest {
            robot_id: "x".into(),
            name: "x".into(),
            architecture: "x".into(),
            capabilities: [Capability::new("vision.rgbd")].into_iter().collect(),
        };
        assert!(m.supports("vision.depth"));
        assert!(m.supports("vision.rgb"));
    }
    #[test]
    fn safety_rejects_out_of_range_joint() {
        let governor = SafetyGovernor {
            state: SafetyState::Safe,
            max_speed_mps: 1.0,
            joint_limits: [("shoulder".into(), (-90.0, 120.0))].into_iter().collect(),
        };
        assert!(matches!(
            governor.validate(
                &ActuatorCommand::SetJoint {
                    joint: "shoulder".into(),
                    angle_deg: 155.0
                },
                None
            ),
            Err(SafetyViolation::JointLimit { .. })
        ));
    }
}
