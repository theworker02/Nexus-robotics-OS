//! Version 2 contracts for integrations, dynamic capabilities, and task safety.

use super::{Capability, CapabilityManifest, Health, Pose, RobotState, SafetyState};
use std::collections::{BTreeMap, BTreeSet};

/// How Nexus is deployed relative to an existing robot stack.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeploymentMode {
    Native,
    Adapter,
    Compatibility,
}

/// The origin of a capability claim. Consumers can explain disagreements instead of hiding them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilitySourceType {
    Static,
    Discovered,
    Overridden,
    Derived,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityProvenance {
    pub source_type: CapabilitySourceType,
    pub provider: String,
    pub observed_at_ms: Option<u128>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityRecord {
    pub capability: Capability,
    pub enabled: bool,
    pub provenance: CapabilityProvenance,
}

/// NCM 2.0 keeps the v1 manifest for compatibility and adds attributable capability evidence.
#[derive(Clone, Debug)]
pub struct CapabilityManifestV2 {
    pub base: CapabilityManifest,
    pub records: BTreeMap<String, CapabilityRecord>,
    pub capability_hash: Option<String>,
}
impl CapabilityManifestV2 {
    pub fn supports(&self, path: &str) -> bool {
        self.records.get(path).is_some_and(|record| record.enabled) || self.base.supports(path)
    }
    pub fn from_static(base: CapabilityManifest, provider: impl Into<String>) -> Self {
        let provider = provider.into();
        let records = base
            .capabilities
            .iter()
            .map(|capability| {
                (
                    capability.0.clone(),
                    CapabilityRecord {
                        capability: capability.clone(),
                        enabled: true,
                        provenance: CapabilityProvenance {
                            source_type: CapabilitySourceType::Static,
                            provider: provider.clone(),
                            observed_at_ms: None,
                        },
                    },
                )
            })
            .collect();
        Self {
            base,
            records,
            capability_hash: None,
        }
    }
}

/// The most restrictive motion policy always wins. Nexus never relaxes a vendor safety limit.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionPolicy {
    pub max_speed_mps: f32,
    pub joint_limits: BTreeMap<String, (f32, f32)>,
    pub restricted_zones: BTreeSet<String>,
    pub collision_avoidance: bool,
}
impl MotionPolicy {
    pub fn effective_with(&self, vendor: &VendorLimits) -> Self {
        let mut joint_limits = self.joint_limits.clone();
        for (joint, (vendor_min, vendor_max)) in &vendor.joint_limits {
            joint_limits
                .entry(joint.clone())
                .and_modify(|(min, max)| {
                    *min = min.max(*vendor_min);
                    *max = max.min(*vendor_max);
                })
                .or_insert((*vendor_min, *vendor_max));
        }
        Self {
            max_speed_mps: self.max_speed_mps.min(vendor.max_speed_mps),
            joint_limits,
            restricted_zones: self
                .restricted_zones
                .union(&vendor.restricted_zones)
                .cloned()
                .collect(),
            collision_avoidance: self.collision_avoidance || vendor.collision_avoidance,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct VendorLimits {
    pub max_speed_mps: f32,
    pub joint_limits: BTreeMap<String, (f32, f32)>,
    pub restricted_zones: BTreeSet<String>,
    pub collision_avoidance: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TaskPriority {
    Background,
    Normal,
    High,
    Safety,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskNodeV2 {
    Action(String),
    Parallel(Vec<String>),
    Wait {
        reason: String,
        timeout_ms: u64,
    },
    HumanGate(String),
    Recovery {
        retries: u8,
        compensation: Option<String>,
    },
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskGraphV2 {
    pub id: String,
    pub priority: TaskPriority,
    pub nodes: Vec<TaskNodeV2>,
    pub resource_locks: BTreeSet<String>,
}

/// A low-level adapter boundary; callers use capability contracts, not vendor internals.
pub trait RobotAdapter: Send {
    fn name(&self) -> &str;
    fn mode(&self) -> DeploymentMode;
    fn connect(&mut self) -> Result<(), AdapterError>;
    fn disconnect(&mut self) -> Result<(), AdapterError>;
    fn discover(&mut self) -> Result<CapabilityManifestV2, AdapterError>;
    fn health(&self) -> Health;
    fn telemetry(&self) -> Result<BTreeMap<String, String>, AdapterError>;
    fn stop(&mut self) -> Result<(), AdapterError>;
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterError {
    Unavailable(String),
    AuthenticationFailed,
    InvalidManifest(String),
    OperationDenied(String),
}
impl std::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for AdapterError {}

/// A physics backend is swappable; v2's deterministic backend remains valid for CI.
pub trait PhysicsBackend: Send {
    fn name(&self) -> &str;
    fn reset(&mut self) -> Result<(), String>;
    fn step(&mut self, milliseconds: u64) -> Result<(), String>;
}

/// A digital twin mirrors the safety-relevant, inspectable robot shape—not proprietary vendor assets.
#[derive(Clone, Debug)]
pub struct DigitalTwin {
    pub id: String,
    pub geometry: Vec<String>,
    pub joints: BTreeMap<String, (f32, f32)>,
    pub sensors: Vec<String>,
    pub actuators: Vec<String>,
    pub mass_kg: f32,
    pub collisions_enabled: bool,
    pub manifest: CapabilityManifestV2,
}

#[derive(Clone, Debug)]
pub struct ShadowAssessment {
    pub predicted_pose: Pose,
    pub safety: SafetyState,
    pub advisory: String,
}
pub fn shadow_assess(
    state: &RobotState,
    requested_speed: f32,
    policy: &MotionPolicy,
) -> ShadowAssessment {
    let safety = if requested_speed.abs() > policy.max_speed_mps {
        SafetyState::Limited
    } else {
        state.safety_state
    };
    ShadowAssessment {
        predicted_pose: Pose {
            x: state.pose.x + requested_speed,
            ..state.pose
        },
        safety,
        advisory: if safety == SafetyState::Limited {
            "command exceeds effective motion policy".into()
        } else {
            "prediction is advisory; emergency stop remains direct".into()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vendor_limits_can_only_tighten() {
        let local = MotionPolicy {
            max_speed_mps: 1.0,
            joint_limits: [("arm".into(), (-90.0, 120.0))].into_iter().collect(),
            restricted_zones: BTreeSet::new(),
            collision_avoidance: false,
        };
        let vendor = VendorLimits {
            max_speed_mps: 0.6,
            joint_limits: [("arm".into(), (-45.0, 80.0))].into_iter().collect(),
            restricted_zones: ["no-entry".into()].into_iter().collect(),
            collision_avoidance: true,
        };
        let effective = local.effective_with(&vendor);
        assert_eq!(effective.max_speed_mps, 0.6);
        assert_eq!(effective.joint_limits["arm"], (-45.0, 80.0));
        assert!(effective.collision_avoidance);
    }
}
