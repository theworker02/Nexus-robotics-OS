//! Reliability contracts and resource arbitration for production skill execution.
use nexus_core::{Health, RobotLifecycle, RobotState, SafetyState};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkillLifecycle {
    Draft,
    UnitTested,
    SimulationValidated,
    HilValidated,
    HardwareValidated,
    Production,
    Deprecated,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationPolicy {
    Immediate,
    SafePoint,
    NotCancellable,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Determinism {
    Deterministic,
    Probabilistic,
    ModelDriven,
}
#[derive(Clone, Debug)]
pub struct SkillContract {
    pub lifecycle: SkillLifecycle,
    pub cancellation: CancellationPolicy,
    pub determinism: Determinism,
    pub max_runtime_s: u64,
    pub min_battery_percent: f32,
    pub requires_safe_state: bool,
    pub exclusive_resources: BTreeSet<String>,
    pub shared_resources: BTreeSet<String>,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub recoverable_failures: Vec<String>,
    pub non_recoverable_failures: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    BatteryLow { available: u8, minimum: u8 },
    SafetyState(SafetyState),
    RobotNotReady(RobotLifecycle),
    Health(Health),
}
impl std::fmt::Display for ContractError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for ContractError {}
impl SkillContract {
    pub fn validate(&self, state: &RobotState) -> Result<(), ContractError> {
        if state.battery_percent < self.min_battery_percent {
            return Err(ContractError::BatteryLow {
                available: state.battery_percent as u8,
                minimum: self.min_battery_percent as u8,
            });
        }
        if self.requires_safe_state && state.safety_state != SafetyState::Safe {
            return Err(ContractError::SafetyState(state.safety_state));
        }
        if state.lifecycle != RobotLifecycle::Ready {
            return Err(ContractError::RobotNotReady(state.lifecycle));
        }
        if state.health != Health::Healthy {
            return Err(ContractError::Health(state.health));
        }
        Ok(())
    }
}
pub fn builtin_contract(name: &str) -> SkillContract {
    let (resources, battery, cancellation) = match name {
        "stop" | "safe_shutdown" => (
            &["base", "left_arm", "right_arm"][..],
            0.0,
            CancellationPolicy::Immediate,
        ),
        "pick_up" | "place" | "place_object" | "reach" | "open_gripper" | "close_gripper"
        | "handoff" | "stow_arm" => (&["right_arm"][..], 25.0, CancellationPolicy::SafePoint),
        "move_forward" | "move_backward" | "walk_to" | "navigate_to" | "return_home" | "dock"
        | "rotate" => (&["base"][..], 15.0, CancellationPolicy::SafePoint),
        "scan_area" | "scan_room" | "find_object" | "inspect_object" | "track_object"
        | "look_at" => (&["front_camera"][..], 10.0, CancellationPolicy::Immediate),
        _ => (&[][..], 5.0, CancellationPolicy::Immediate),
    };
    SkillContract {
        lifecycle: SkillLifecycle::SimulationValidated,
        cancellation,
        determinism: Determinism::Deterministic,
        max_runtime_s: 180,
        min_battery_percent: battery,
        requires_safe_state: name != "stop",
        exclusive_resources: resources.iter().map(|value| (*value).into()).collect(),
        shared_resources: BTreeSet::new(),
        preconditions: vec![
            "robot.state=READY".into(),
            format!("battery.percent>={battery}"),
            "health=HEALTHY".into(),
        ],
        postconditions: vec!["runtime event recorded".into()],
        recoverable_failures: vec!["adapter timeout".into(), "target moved".into()],
        non_recoverable_failures: vec!["emergency stop".into(), "hardware fault".into()],
    }
}
#[derive(Default, Debug)]
pub struct ResourceArbiter {
    owners: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResourceError {
    Conflict { resource: String, owner: String },
}
impl ResourceArbiter {
    pub fn acquire(
        &mut self,
        skill: &str,
        resources: &BTreeSet<String>,
    ) -> Result<(), ResourceError> {
        for resource in resources {
            if let Some(owner) = self.owners.get(resource) {
                if owner != skill {
                    return Err(ResourceError::Conflict {
                        resource: resource.clone(),
                        owner: owner.clone(),
                    });
                }
            }
        }
        for resource in resources {
            self.owners.insert(resource.clone(), skill.into());
        }
        Ok(())
    }
    pub fn release(&mut self, skill: &str) {
        self.owners.retain(|_, owner| owner != skill);
    }
    pub fn owner(&self, resource: &str) -> Option<&str> {
        self.owners.get(resource).map(String::as_str)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn resources_cannot_be_controlled_by_two_skills() {
        let mut arbiter = ResourceArbiter::default();
        arbiter
            .acquire("pick_up", &["right_arm".into()].into_iter().collect())
            .unwrap();
        assert!(matches!(
            arbiter.acquire("stow_arm", &["right_arm".into()].into_iter().collect()),
            Err(ResourceError::Conflict { .. })
        ));
    }
}
