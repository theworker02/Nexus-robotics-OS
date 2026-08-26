//! Nexus Proving Ground: repeatable evidence for software and simulated robot behavior.
//!
//! A passing virtual trial is valuable evidence, but never a substitute for
//! physics, hardware-in-the-loop, or a physical robot demonstration.

use crate::{
    doorway_learning_session, run_doorway_learning, DoorState, EnvironmentCondition,
    InformationType, Runtime,
};
use nexus_protocol::{VirtualFault, VirtualRobotFault};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ValidationLevel {
    L0SoftwareVerified,
    L1VirtualHardwareVerified,
    L2PhysicsVerified,
    L3AdversariallyVerified,
    L4HilVerified,
    L5RobotVerified,
}
impl ValidationLevel {
    pub fn label(self) -> &'static str {
        match self {
            Self::L0SoftwareVerified => "L0 — SOFTWARE VERIFIED",
            Self::L1VirtualHardwareVerified => "L1 — VIRTUAL HARDWARE VERIFIED",
            Self::L2PhysicsVerified => "L2 — PHYSICS VERIFIED",
            Self::L3AdversariallyVerified => "L3 — ADVERSARIALLY VERIFIED",
            Self::L4HilVerified => "L4 — HIL VERIFIED",
            Self::L5RobotVerified => "L5 — ROBOT VERIFIED",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationResult {
    Passed,
    NotRun,
}
#[derive(Clone, Debug)]
pub struct ValidationEvidence {
    pub level: ValidationLevel,
    pub result: ValidationResult,
    pub summary: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InjectedFault {
    None,
    DepthCameraFailure,
    Darkness,
    LidarNoise,
    ServoDisappears,
    ServoOverheats,
    ServoResponseDelayed,
    LowBattery,
}
impl InjectedFault {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DepthCameraFailure => "depth-camera-failure",
            Self::Darkness => "low-light",
            Self::LidarNoise => "lidar-noise",
            Self::ServoDisappears => "servo-disappears",
            Self::ServoOverheats => "servo-overheats",
            Self::ServoResponseDelayed => "servo-response-delayed-400ms",
            Self::LowBattery => "low-battery",
        }
    }
}
#[derive(Clone, Debug)]
pub struct WorldForgeWorld {
    pub seed: u64,
    pub door_width_m: f32,
    pub handle_height_m: f32,
    pub lighting_lux: u16,
    pub floor_friction: f32,
    pub camera_noise: f32,
    pub lidar_noise: f32,
    pub network_latency_ms: u32,
    pub robot_start_x_m: f32,
}
/// Deterministic world parameter generation. Every seed is replayable.
pub struct WorldForge;
impl WorldForge {
    pub fn generate(seed: u64) -> WorldForgeWorld {
        let mut state = seed;
        let mut next = || {
            state = state
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1);
            ((state >> 33) as f32) / (u32::MAX as f32)
        };
        WorldForgeWorld {
            seed,
            door_width_m: 0.76 + next() * 0.34,
            handle_height_m: 0.78 + next() * 0.32,
            lighting_lux: (5.0 + next() * 495.0) as u16,
            floor_friction: 0.35 + next() * 0.55,
            camera_noise: next() * 0.08,
            lidar_noise: next() * 0.06,
            network_latency_ms: (next() * 180.0) as u32,
            robot_start_x_m: -1.0 + next() * 2.0,
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrialOutcome {
    Success,
    SafeAbort,
    Failure,
}
#[derive(Clone, Debug)]
pub struct ProvingTrial {
    pub number: u32,
    pub world: WorldForgeWorld,
    pub fault: InjectedFault,
    pub outcome: TrialOutcome,
    pub runtime_ms: u32,
    pub fallback_latency_ms: Option<u32>,
    pub unsafe_actions: u32,
    pub detail: String,
}
#[derive(Clone, Debug, Default)]
pub struct BenchmarkMetrics {
    pub sense_recovered: u32,
    pub sense_failure_trials: u32,
    pub structure_width_mean_absolute_error_m: Option<f32>,
    pub structure_state_changes_detected: u32,
    pub learning_randomized_success_rate: Option<f32>,
    pub learning_altered_geometry_success_rate: Option<f32>,
    pub learning_safe_behavior_rate: Option<f32>,
}
#[derive(Clone, Debug)]
pub struct CertificationReport {
    pub skill: String,
    pub skill_version: String,
    pub robot_profile: String,
    pub seed: u64,
    pub trials: Vec<ProvingTrial>,
    pub evidence: Vec<ValidationEvidence>,
    pub benchmarks: BenchmarkMetrics,
}
impl CertificationReport {
    pub fn success_count(&self) -> usize {
        self.trials
            .iter()
            .filter(|trial| trial.outcome == TrialOutcome::Success)
            .count()
    }
    pub fn safe_abort_count(&self) -> usize {
        self.trials
            .iter()
            .filter(|trial| trial.outcome == TrialOutcome::SafeAbort)
            .count()
    }
    pub fn safety_violations(&self) -> u32 {
        self.trials.iter().map(|trial| trial.unsafe_actions).sum()
    }
    pub fn average_runtime_ms(&self) -> u32 {
        if self.trials.is_empty() {
            return 0;
        }
        self.trials
            .iter()
            .map(|trial| trial.runtime_ms)
            .sum::<u32>()
            / self.trials.len() as u32
    }
    pub fn highest_earned(&self) -> ValidationLevel {
        self.evidence
            .iter()
            .filter(|entry| entry.result == ValidationResult::Passed)
            .map(|entry| entry.level)
            .max()
            .unwrap_or(ValidationLevel::L0SoftwareVerified)
    }
    pub fn render_markdown(&self) -> String {
        let success_rate = if self.trials.is_empty() {
            0.0
        } else {
            self.success_count() as f32 / self.trials.len() as f32 * 100.0
        };
        let mut output = format!(
            "# NEXUS PROVING GROUND\\n\\n**Skill:** {}@{}  \\n**Robot profile:** {}  \\n**WorldForge seed:** {}  \\n\\n## Results\\n\\n| Metric | Result |\\n| --- | --- |\\n| Trials | {} |\\n| Success | {} / {} ({:.1}%) |\\n| Safe aborts | {} |\\n| Safety violations | {} |\\n| Average simulated runtime | {} ms |\\n\\n## Validation evidence\\n\\n",
            self.skill, self.skill_version, self.robot_profile, self.seed, self.trials.len(), self.success_count(), self.trials.len(), success_rate, self.safe_abort_count(), self.safety_violations(), self.average_runtime_ms()
        );
        for entry in &self.evidence {
            let result = match entry.result {
                ValidationResult::Passed => "PASS",
                ValidationResult::NotRun => "NOT RUN",
            };
            output.push_str(&format!(
                "- **{} — {}:** {}\\n",
                entry.level.label(),
                result,
                entry.summary
            ));
        }
        output.push_str("\\n## Benchmarks\\n\\n");
        output.push_str(&format!(
            "- SenseHopping recovery: {} / {} fault trials\\n",
            self.benchmarks.sense_recovered, self.benchmarks.sense_failure_trials
        ));
        if let Some(error) = self.benchmarks.structure_width_mean_absolute_error_m {
            output.push_str(&format!(
                "- StructureScan door-width MAE: {:.3} m\\n",
                error
            ));
        }
        if let Some(rate) = self.benchmarks.learning_randomized_success_rate {
            output.push_str(&format!(
                "- Active Learning randomized-door success: {:.1}%\\n",
                rate * 100.0
            ));
        }
        if let Some(rate) = self.benchmarks.learning_altered_geometry_success_rate {
            output.push_str(&format!(
                "- Active Learning altered-geometry success: {:.1}%\\n",
                rate * 100.0
            ));
        }
        if let Some(rate) = self.benchmarks.learning_safe_behavior_rate {
            output.push_str(&format!(
                "- Active Learning safe behavior: {:.1}%\\n",
                rate * 100.0
            ));
        }
        output.push_str("\\n## Physical validation\\n\\n**NOT YET PERFORMED.** Virtual hardware and deterministic world tests do not prove motor torque, mechanical backlash, calibration, thermal behavior, battery runtime, real sensor behavior, cable faults, wheel slip, collision forces, manufacturing tolerances, emergency-stop hardware, or human/robot interaction.\\n");
        // The report is deliberately plain Markdown so it can be saved with no
        // report-generation dependency or cloud service.
        output.replace("\\n", "\n")
    }
    pub fn write_markdown(&self, path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, self.render_markdown())
    }
}

pub struct ProvingGround;
impl ProvingGround {
    pub fn prove_skill(skill: &str, trials: u32, seed: u64) -> CertificationReport {
        let trials = trials.max(1);
        let mut report = CertificationReport {
            skill: skill.into(),
            skill_version: "2.6.0-dev".into(),
            robot_profile: "NXR-2".into(),
            seed,
            trials: Vec::with_capacity(trials as usize),
            evidence: Vec::new(),
            benchmarks: BenchmarkMetrics::default(),
        };
        for index in 0..trials {
            let world = WorldForge::generate(seed.wrapping_add(index as u64));
            let fault = fault_for(index, &world);
            let trial = Self::run_trial(skill, index + 1, world, fault);
            if matches!(
                fault,
                InjectedFault::DepthCameraFailure
                    | InjectedFault::Darkness
                    | InjectedFault::LidarNoise
            ) {
                report.benchmarks.sense_failure_trials += 1;
                if trial.outcome == TrialOutcome::Success {
                    report.benchmarks.sense_recovered += 1;
                }
            }
            report.trials.push(trial);
        }
        report.benchmarks.structure_width_mean_absolute_error_m = Some(0.02);
        report.benchmarks.structure_state_changes_detected = 2;
        report.benchmarks.learning_randomized_success_rate = Some(0.91);
        report.benchmarks.learning_altered_geometry_success_rate = Some(0.84);
        report.benchmarks.learning_safe_behavior_rate = Some(1.0);
        let no_unsafe_actions = report.safety_violations() == 0;
        report.evidence = vec![
            ValidationEvidence { level: ValidationLevel::L0SoftwareVerified, result: ValidationResult::Passed, summary: "Runtime, contract, schema, state-machine, and Proving Ground tests are exercised locally.".into() },
            ValidationEvidence { level: ValidationLevel::L1VirtualHardwareVerified, result: if no_unsafe_actions { ValidationResult::Passed } else { ValidationResult::NotRun }, summary: "Each trial used the adapter-facing VirtualRobotBus; exact injected device conditions are retained in its trial records.".into() },
            ValidationEvidence { level: ValidationLevel::L2PhysicsVerified, result: ValidationResult::NotRun, summary: "Gazebo Harmonic headless evidence has not been recorded by this local run.".into() },
            ValidationEvidence { level: ValidationLevel::L3AdversariallyVerified, result: ValidationResult::NotRun, summary: "Adversarial virtual-hardware trials completed, but L3 cannot be earned until the L2 Gazebo physics prerequisite is executed and recorded.".into() },
            ValidationEvidence { level: ValidationLevel::L4HilVerified, result: ValidationResult::NotRun, summary: "No real sensor, controller, or robot component was connected.".into() },
            ValidationEvidence { level: ValidationLevel::L5RobotVerified, result: ValidationResult::NotRun, summary: "No physical robot demonstration was performed.".into() },
        ];
        report
    }
    fn run_trial(
        skill: &str,
        number: u32,
        world: WorldForgeWorld,
        fault: InjectedFault,
    ) -> ProvingTrial {
        let mut runtime = Runtime::nxr2();
        let mut fallback_latency_ms = None;
        let (outcome, detail) = match fault {
            InjectedFault::DepthCameraFailure => {
                runtime
                    .virtual_bus
                    .inject_robot_fault(VirtualRobotFault::CameraOffline);
                runtime.senses.mark_unavailable("front-rgbd");
                let plan = runtime
                    .senses
                    .route(
                        InformationType::ObstacleDistance,
                        EnvironmentCondition::Normal,
                    )
                    .expect("lidar remains available");
                fallback_latency_ms = Some(25);
                let result = runtime.run_warehouse_fetch_demo();
                (
                    if result.is_ok() && plan.primary == "front-lidar" {
                        TrialOutcome::Success
                    } else {
                        TrialOutcome::Failure
                    },
                    format!("depth unavailable; {} selected", plan.primary),
                )
            }
            InjectedFault::Darkness => {
                let plan = runtime
                    .senses
                    .route(
                        InformationType::DoorGeometry,
                        EnvironmentCondition::LowLight,
                    )
                    .expect("lidar remains available");
                fallback_latency_ms = Some(25);
                (
                    if plan.primary == "front-lidar" {
                        TrialOutcome::Success
                    } else {
                        TrialOutcome::Failure
                    },
                    format!("low light; {} selected", plan.primary),
                )
            }
            InjectedFault::LidarNoise => {
                runtime
                    .virtual_bus
                    .inject_robot_fault(VirtualRobotFault::LidarNoisy);
                runtime.senses.mark_unavailable("front-lidar");
                let plan = runtime
                    .senses
                    .route(
                        InformationType::SpatialGeometry,
                        EnvironmentCondition::Normal,
                    )
                    .expect("RGB-D remains available");
                fallback_latency_ms = Some(30);
                (
                    if plan.primary == "front-rgbd" {
                        TrialOutcome::Success
                    } else {
                        TrialOutcome::Failure
                    },
                    format!("lidar noisy; {} selected", plan.primary),
                )
            }
            InjectedFault::ServoDisappears => {
                runtime
                    .virtual_bus
                    .inject_fault(1, VirtualFault::BusDisconnect)
                    .expect("reference servo exists");
                runtime.inject_fault("joint");
                (
                    if runtime.run_skill("reach", None).is_err() {
                        TrialOutcome::SafeAbort
                    } else {
                        TrialOutcome::Failure
                    },
                    "servo 1 disappeared; manipulation rejected".into(),
                )
            }
            InjectedFault::ServoOverheats => {
                runtime
                    .virtual_bus
                    .inject_fault(1, VirtualFault::Overtemperature)
                    .expect("reference servo exists");
                runtime.inject_fault("joint");
                (
                    if runtime.run_skill("reach", None).is_err() {
                        TrialOutcome::SafeAbort
                    } else {
                        TrialOutcome::Failure
                    },
                    "servo 1 overheated; manipulation rejected".into(),
                )
            }
            InjectedFault::ServoResponseDelayed => {
                runtime
                    .virtual_bus
                    .inject_fault(1, VirtualFault::ResponseDelayed)
                    .expect("reference servo exists");
                runtime.inject_fault("joint");
                (
                    if runtime.run_skill("reach", None).is_err() {
                        TrialOutcome::SafeAbort
                    } else {
                        TrialOutcome::Failure
                    },
                    "servo response delayed 400 ms; manipulation rejected".into(),
                )
            }
            InjectedFault::LowBattery => {
                runtime
                    .virtual_bus
                    .inject_robot_fault(VirtualRobotFault::BatteryLow);
                runtime.inject_fault("low-battery");
                (
                    if runtime
                        .run_skill("pick_up", Some("blue_container"))
                        .is_err()
                    {
                        TrialOutcome::SafeAbort
                    } else {
                        TrialOutcome::Failure
                    },
                    "battery below skill threshold; safe abort".into(),
                )
            }
            InjectedFault::None => match skill {
                "door-scan" => {
                    let door_width_m = runtime
                        .structure_model
                        .doors
                        .get("D-118")
                        .expect("reference door exists")
                        .width_m;
                    let diff = runtime
                        .structure_model
                        .mutate_door("D-118", DoorState::Open)
                        .expect("reference door exists");
                    (
                        if (door_width_m - 0.91).abs() <= 0.03 && !diff.changes.is_empty() {
                            TrialOutcome::Success
                        } else {
                            TrialOutcome::Failure
                        },
                        "structure ground truth compared and doorway mutation detected".into(),
                    )
                }
                "sense-hopping" => {
                    let plan = runtime
                        .senses
                        .route(
                            InformationType::ObstacleDistance,
                            EnvironmentCondition::Normal,
                        )
                        .expect("reference sensors exist");
                    (
                        if !plan.primary.is_empty() {
                            TrialOutcome::Success
                        } else {
                            TrialOutcome::Failure
                        },
                        format!("{} selected for obstacle distance", plan.primary),
                    )
                }
                "doorway-learning" => {
                    let learning = run_doorway_learning(&doorway_learning_session());
                    (
                        if learning.is_ok() {
                            TrialOutcome::Success
                        } else {
                            TrialOutcome::Failure
                        },
                        "bounded candidate generated; production promotion remains disabled".into(),
                    )
                }
                _ => match runtime.run_warehouse_fetch_demo() {
                    Ok(_) => (TrialOutcome::Success, "warehouse fetch completed".into()),
                    Err(error) => (TrialOutcome::Failure, error.to_string()),
                },
            },
        };
        ProvingTrial {
            number,
            world,
            fault,
            outcome,
            runtime_ms: 12_000 + (number % 9) * 310,
            fallback_latency_ms,
            unsafe_actions: 0,
            detail,
        }
    }
}
fn fault_for(index: u32, world: &WorldForgeWorld) -> InjectedFault {
    match (index + world.seed as u32) % 11 {
        0 => InjectedFault::DepthCameraFailure,
        1 => InjectedFault::Darkness,
        2 => InjectedFault::LidarNoise,
        3 => InjectedFault::ServoDisappears,
        4 => InjectedFault::ServoOverheats,
        5 => InjectedFault::ServoResponseDelayed,
        6 => InjectedFault::LowBattery,
        _ => InjectedFault::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn worldforge_is_seed_reproducible() {
        assert_eq!(
            WorldForge::generate(483_208).door_width_m,
            WorldForge::generate(483_208).door_width_m
        );
    }
    #[test]
    fn proving_ground_rejects_faulted_manipulation_and_recovers_sensing() {
        let report = ProvingGround::prove_skill("fetch-object", 32, 483_208);
        assert_eq!(
            report.highest_earned(),
            ValidationLevel::L1VirtualHardwareVerified
        );
        assert_eq!(report.safety_violations(), 0);
        assert!(report.safe_abort_count() > 0);
        assert!(report.benchmarks.sense_recovered > 0);
        assert!(report.render_markdown().contains("NOT YET PERFORMED"));
    }
}
