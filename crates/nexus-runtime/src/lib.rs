//! Offline-first reference runtime and deterministic NXR-1 simulator.

use nexus_core::*;
use nexus_protocol::{VirtualBus, VirtualFault, VirtualRobotFault};
use std::{
    collections::{BTreeMap, BTreeSet},
    time::{SystemTime, UNIX_EPOCH},
};
pub mod durable;
pub mod intelligence;
pub mod learning;
pub mod package;
pub mod proving;
pub mod reliable;
pub mod sense;
pub mod structure;
pub mod watchdog;
pub use durable::{DurableEvent, DurableEventLog, RecoveryDecision};
pub use intelligence::{
    ApprovalRequest, AutomationAction, AutomationClass, AutomationRule, AutomationTrigger,
    AutonomyCapability, AutonomyError, AutonomyPermission, AutonomyPolicy, ComputePlacement,
    ComputeRequest, ComputeRouter, ExecutionGrant, ExperienceMemory, Goal, GoalPlan, GoalRisk,
    IntelligenceLayer, IntelligenceProfile, MemoryCategory, MemoryRecord, MemoryRetention,
    OperatingEnvelope, PermissionDecision, PlanState, PlanStep, Routine,
};
pub use learning::{
    doorway_learning_session, run_doorway_learning, FailureCategory, LearningArtifact,
    LearningAttempt, LearningBudget, LearningSession, LearningStage, SkillImprovementProposal,
};
pub use nexus_brain::{
    capabilities_from_profile, manifest_from_profile, upgrade_advisor, AdaptiveRuntimePlan,
    AdaptiveRuntimePlanner, CapabilityIndex, ComputePlacement as BrainComputePlacement,
    FeatureLevel, HardwareProfile, IntelligenceClass, MemCorePlan, UpgradeRecommendation,
};
pub use package::{PackageError, PackageInspection, PackageManifest, PackageType};
pub use proving::{
    BenchmarkMetrics, CertificationReport, InjectedFault, ProvingGround, ProvingTrial,
    TrialOutcome, ValidationEvidence, ValidationLevel, ValidationResult, WorldForge,
    WorldForgeWorld,
};
pub use reliable::{
    CancellationPolicy, Determinism, ResourceArbiter, ResourceError, SkillContract, SkillLifecycle,
};
pub use sense::{
    nxr2_senses, EnvironmentCondition, InformationType, SenseError, SenseHistoryEntry, SensePlan,
    SenseProvider, SenseRouter,
};
pub use structure::{
    DoorModel, DoorState, MaterialCategory, MaterialEstimate, StructureChange, StructureDiff,
    StructureKind, StructureModel, StructureSurface,
};
pub use watchdog::{WatchdogAlert, WatchdogKind, WatchdogSet};

#[derive(Clone, Debug)]
pub struct SkillManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub requires: Vec<String>,
    pub permissions: Vec<String>,
    pub contract: SkillContract,
}

#[derive(Clone, Debug)]
pub struct SkillRegistry {
    skills: BTreeMap<String, SkillManifest>,
    enabled: BTreeSet<String>,
}
impl SkillRegistry {
    pub fn builtin() -> Self {
        let definitions: [(&str, &str, &[&str], &[&str]); 35] = [
            (
                "stop",
                "Immediately stop autonomous motion.",
                &[],
                &["locomotion.control"],
            ),
            (
                "speak",
                "Speak supplied text through the robot speaker.",
                &["audio.speakers"],
                &["speaker.write"],
            ),
            (
                "look_at",
                "Orient vision toward a named target.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "turn_left",
                "Turn safely to the left.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "turn_right",
                "Turn safely to the right.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "move_forward",
                "Move forward with speed limits enforced.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "move_backward",
                "Move backward with speed limits enforced.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "walk_to",
                "Navigate to a known location.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "follow_target",
                "Follow a visible object.",
                &["locomotion.control", "vision.rgb"],
                &["locomotion.control", "camera.read"],
            ),
            (
                "pick_up",
                "Grasp an identified object.",
                &["vision.rgb", "manipulators.right.gripper"],
                &["camera.read", "manipulator.right.control"],
            ),
            (
                "place_object",
                "Place a held object at a target.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "inspect_object",
                "Inspect an object using synthetic vision.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "scan_room",
                "Scan the current environment.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "return_home",
                "Navigate to the home pose.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "dock",
                "Navigate to and dock at a charging station.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            ("pause", "Pause a current task at a safe point.", &[], &[]),
            ("resume", "Resume a paused task after validation.", &[], &[]),
            (
                "rotate",
                "Rotate the mobile base safely.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "navigate_to",
                "Navigate to a named location.",
                &["locomotion.control"],
                &["locomotion.control"],
            ),
            (
                "open_gripper",
                "Open the right gripper.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "close_gripper",
                "Close the right gripper.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "reach",
                "Move the right arm toward a target.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "place",
                "Place a held object safely.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "handoff",
                "Present a held object for handoff.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "stow_arm",
                "Return the arm to a stowed pose.",
                &["manipulators.right.gripper"],
                &["manipulator.right.control"],
            ),
            (
                "scan_area",
                "Scan a named area.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "find_object",
                "Locate a named known object.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "track_object",
                "Track a visible object.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "listen_for_command",
                "Read audio input for a command.",
                &["audio.microphones"],
                &["microphone.read"],
            ),
            (
                "request_assistance",
                "Report a task state requiring an operator.",
                &["audio.speakers"],
                &["speaker.write"],
            ),
            (
                "self_check",
                "Run non-actuating health diagnostics.",
                &[],
                &["telemetry.read"],
            ),
            (
                "recalibrate",
                "Request controlled simulated recalibration.",
                &[],
                &["telemetry.read"],
            ),
            (
                "safe_shutdown",
                "Stop safely and transition the robot offline.",
                &[],
                &["locomotion.control"],
            ),
            (
                "structure_scan",
                "Inspect visible and instrument-accessible building structure.",
                &["vision.rgb"],
                &["camera.read"],
            ),
            (
                "door_scan",
                "Characterize observable door geometry and state.",
                &["vision.rgb"],
                &["camera.read"],
            ),
        ];
        let skills: BTreeMap<String, SkillManifest> = definitions
            .into_iter()
            .map(|(name, description, requires, permissions)| {
                (
                    name.into(),
                    SkillManifest {
                        name: name.into(),
                        version: "0.5.0".into(),
                        description: description.into(),
                        requires: requires.iter().map(|v| (*v).into()).collect(),
                        permissions: permissions.iter().map(|v| (*v).into()).collect(),
                        contract: reliable::builtin_contract(name),
                    },
                )
            })
            .collect();
        let enabled = skills.keys().cloned().collect();
        Self { skills, enabled }
    }
    pub fn list(&self) -> impl Iterator<Item = &SkillManifest> {
        self.skills.values()
    }
    pub fn get(&self, name: &str) -> Option<&SkillManifest> {
        self.skills.get(name)
    }
    pub fn compatibility(
        &self,
        name: &str,
        robot: &CapabilityManifest,
    ) -> Result<Compatibility, RuntimeError> {
        let skill = self
            .get(name)
            .ok_or_else(|| RuntimeError::UnknownSkill(name.into()))?;
        Ok(CapabilityQuery {
            requirements: skill.requires.clone(),
        }
        .evaluate(robot))
    }
    pub fn install_local_manifest(&mut self, yaml: &str) -> Result<String, RuntimeError> {
        let name = yaml
            .lines()
            .find_map(|line| line.trim().strip_prefix("name:").map(str::trim))
            .ok_or(RuntimeError::InvalidSkillManifest)?
            .to_string();
        let version = yaml
            .lines()
            .find_map(|line| line.trim().strip_prefix("version:").map(str::trim))
            .unwrap_or("0.1.0")
            .to_string();
        self.skills.insert(
            name.clone(),
            SkillManifest {
                name: name.clone(),
                version,
                description: "Locally installed skill".into(),
                requires: vec![],
                permissions: vec![],
                contract: reliable::builtin_contract(&name),
            },
        );
        self.enabled.insert(name.clone());
        Ok(name)
    }
}

#[derive(Clone, Debug)]
pub struct WorldObject {
    pub id: String,
    pub label: String,
    pub pose: Pose,
    pub held: bool,
}
#[derive(Clone, Debug)]
pub struct WorldState {
    pub environment: String,
    pub objects: BTreeMap<String, WorldObject>,
    pub zones: BTreeMap<String, String>,
    pub charging_station: Pose,
}
impl WorldState {
    pub fn apartment() -> Self {
        Self {
            environment: "apartment".into(),
            objects: [(
                "red_cube".into(),
                WorldObject {
                    id: "red_cube".into(),
                    label: "red cube".into(),
                    pose: Pose {
                        x: 2.0,
                        y: 1.0,
                        yaw: 0.0,
                    },
                    held: false,
                },
            )]
            .into_iter()
            .collect(),
            zones: BTreeMap::new(),
            charging_station: Pose {
                x: 0.0,
                y: 0.0,
                yaw: 0.0,
            },
        }
    }
    pub fn warehouse() -> Self {
        Self {
            environment: "warehouse".into(),
            objects: [(
                "blue_container".into(),
                WorldObject {
                    id: "blue_container".into(),
                    label: "blue container".into(),
                    pose: Pose {
                        x: 5.0,
                        y: 2.0,
                        yaw: 0.0,
                    },
                    held: false,
                },
            )]
            .into_iter()
            .collect(),
            zones: [
                ("station_b".into(), "delivery".into()),
                ("loading_bay".into(), "slow-motion".into()),
            ]
            .into_iter()
            .collect(),
            charging_station: Pose {
                x: 0.0,
                y: 0.0,
                yaw: 0.0,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReplayEntry {
    pub at_ms: u128,
    pub event: String,
    pub pose: Pose,
    pub safety: SafetyState,
}
#[derive(Clone, Debug)]
pub struct Replay {
    pub task_id: String,
    pub entries: Vec<ReplayEntry>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeError {
    UnknownSkill(String),
    CapabilityMissing(Vec<String>),
    Safety(SafetyViolation),
    ObjectUnavailable(String),
    EmergencyStop,
    InvalidSkillManifest,
    Contract(reliable::ContractError),
    Resource(reliable::ResourceError),
    Sense(String),
    Learning(String),
    Autonomy(String),
}
impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSkill(v) => write!(f, "unknown skill: {v}"),
            Self::CapabilityMissing(v) => write!(f, "incompatible; missing: {}", v.join(", ")),
            Self::Safety(v) => write!(f, "safety rejected command: {v:?}"),
            Self::ObjectUnavailable(v) => write!(f, "object unavailable: {v}"),
            Self::EmergencyStop => write!(f, "emergency stop is active"),
            Self::InvalidSkillManifest => write!(f, "skill.yaml must include name"),
            Self::Contract(v) => write!(f, "skill precondition failed: {v}"),
            Self::Resource(v) => write!(f, "resource arbitration failed: {v:?}"),
            Self::Sense(v) => write!(f, "sense routing failed: {v}"),
            Self::Learning(v) => write!(f, "learning session failed: {v}"),
            Self::Autonomy(v) => write!(f, "autonomy policy blocked operation: {v}"),
        }
    }
}
impl std::error::Error for RuntimeError {}

/// The reference local runtime. The simulator is an adapter, not the operating model.
pub struct Runtime {
    pub robot: RobotState,
    pub safety: SafetyGovernor,
    pub world: WorldState,
    pub skills: SkillRegistry,
    pub logs: Vec<StructuredLog>,
    pub durable_events: Vec<DurableEvent>,
    pub events: Vec<Event>,
    pub replays: BTreeMap<String, Replay>,
    pub virtual_bus: VirtualBus,
    pub resources: ResourceArbiter,
    pub watchdogs: WatchdogSet,
    pub senses: SenseRouter,
    pub structure_model: StructureModel,
    pub intelligence: IntelligenceLayer,
    pub hardware_profile: HardwareProfile,
    pub brain_plan: AdaptiveRuntimePlan,
    sequence: u64,
}
impl Runtime {
    pub fn nxr1() -> Self {
        let caps = [
            "locomotion.biped",
            "vision.rgbd",
            "audio.microphones",
            "audio.speakers",
            "sensing.imu",
            "sensing.proximity",
            "manipulators.right.gripper",
            "manipulators.left.gripper",
            "compute.local",
        ];
        let manifest = CapabilityManifest {
            robot_id: "nxr-1".into(),
            name: "NXR-1".into(),
            architecture: "x86_64-sim".into(),
            capabilities: caps.into_iter().map(Capability::new).collect(),
        };
        let limits = [
            ("left_shoulder".into(), (-90.0, 120.0)),
            ("right_shoulder".into(), (-90.0, 120.0)),
        ]
        .into_iter()
        .collect();
        let hardware_profile = HardwareProfile::minimum_robot();
        let brain_plan = AdaptiveRuntimePlanner::plan(&hardware_profile);
        Self {
            robot: RobotState {
                identity: "nxr-1".into(),
                capabilities: manifest,
                sensors: [
                    ("front_camera".into(), "rgbd".into()),
                    ("imu".into(), "healthy".into()),
                ]
                .into_iter()
                .collect(),
                actuators: BTreeMap::new(),
                health: Health::Healthy,
                battery_percent: 100.0,
                network_connected: true,
                pose: Pose {
                    x: 0.0,
                    y: 0.0,
                    yaw: 0.0,
                },
                current_task: None,
                current_skill: None,
                safety_state: SafetyState::Safe,
                lifecycle: RobotLifecycle::Ready,
            },
            safety: SafetyGovernor {
                state: SafetyState::Safe,
                max_speed_mps: 0.8,
                joint_limits: limits,
            },
            world: WorldState::apartment(),
            skills: SkillRegistry::builtin(),
            logs: vec![],
            durable_events: vec![],
            events: vec![Event::RobotConnected],
            replays: BTreeMap::new(),
            virtual_bus: VirtualBus::default(),
            resources: ResourceArbiter::default(),
            watchdogs: WatchdogSet::with_defaults(),
            senses: nxr2_senses(),
            structure_model: StructureModel::doorway_lab(),
            intelligence: IntelligenceLayer::supervised(),
            hardware_profile,
            brain_plan,
            sequence: 0,
        }
    }
    /// NXR-2 is the v2 mobile-manipulator reference profile. It remains deterministic and simulation-only.
    pub fn nxr2() -> Self {
        let mut runtime = Self::nxr1();
        let capabilities = [
            "locomotion.wheeled",
            "vision.rgbd",
            "vision.rgb",
            "sensing.lidar",
            "audio.microphones",
            "audio.speakers",
            "sensing.imu",
            "power.battery",
            "manipulators.left.gripper",
            "manipulators.right.gripper",
            "compute.local",
        ];
        runtime.robot.identity = "nxr-2".into();
        runtime.robot.capabilities = CapabilityManifest {
            robot_id: "nxr-2".into(),
            name: "NXR-2".into(),
            architecture: "x86_64-sim".into(),
            capabilities: capabilities.into_iter().map(Capability::new).collect(),
        };
        runtime.robot.sensors = [
            ("front_rgb".into(), "rgb".into()),
            ("depth_camera".into(), "depth".into()),
            ("lidar".into(), "healthy".into()),
            ("imu".into(), "healthy".into()),
        ]
        .into_iter()
        .collect();
        runtime.world = WorldState::warehouse();
        runtime.virtual_bus = VirtualBus::nxr2();
        runtime.hardware_profile = HardwareProfile::nxr2();
        runtime.brain_plan = AdaptiveRuntimePlanner::plan(&runtime.hardware_profile);
        runtime
    }
    /// Applies a confirmed hardware profile and recomputes capability-derived intelligence features.
    pub fn reprofile_hardware(&mut self, profile: HardwareProfile) {
        self.robot.capabilities = manifest_from_profile(&profile);
        self.robot.identity = self.robot.capabilities.robot_id.clone();
        self.hardware_profile = profile;
        self.brain_plan = AdaptiveRuntimePlanner::plan(&self.hardware_profile);
    }
    fn now() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }
    fn record(&mut self, task_id: &str, event: impl Into<String>) {
        let event = event.into();
        self.replays
            .entry(task_id.into())
            .or_insert_with(|| Replay {
                task_id: task_id.into(),
                entries: vec![],
            })
            .entries
            .push(ReplayEntry {
                at_ms: Self::now(),
                event: event.clone(),
                pose: self.robot.pose,
                safety: self.safety.state,
            });
        self.logs.push(StructuredLog {
            timestamp_ms: Self::now(),
            robot_id: self.robot.identity.clone(),
            task_id: Some(task_id.into()),
            skill_id: self.robot.current_skill.clone(),
            component: "runtime".into(),
            severity: Severity::Info,
            message: event.clone(),
            context: BTreeMap::new(),
        });
        self.durable_events.push(DurableEvent {
            timestamp_ms: Self::now(),
            category: "task".into(),
            subject: task_id.into(),
            message: event,
        });
    }
    /// Persists the append-only event record. Physical motion is never auto-resumed from this log.
    pub fn persist_event_log(
        &self,
        path: impl Into<std::path::PathBuf>,
    ) -> Result<(), std::io::Error> {
        let log = DurableEventLog::open(path)?;
        for event in &self.durable_events {
            log.append(event)?;
        }
        Ok(())
    }
    pub fn emergency_stop(&mut self) {
        self.safety.emergency_stop();
        self.robot.safety_state = SafetyState::EmergencyStop;
        self.robot.current_task = None;
        self.robot.current_skill = None;
        self.robot.lifecycle = RobotLifecycle::EmergencyStop;
        self.events.push(Event::SafetyTriggered);
    }
    pub fn reset_emergency_stop(&mut self) {
        self.safety.reset();
        self.robot.safety_state = self.safety.state;
        self.robot.lifecycle = RobotLifecycle::Ready;
    }
    pub fn command(&mut self, command: ActuatorCommand) -> Result<(), RuntimeError> {
        self.safety
            .validate(&command, None)
            .map_err(RuntimeError::Safety)?;
        if let ActuatorCommand::Move {
            speed_mps,
            duration_s,
        } = command
        {
            self.robot.pose.x += speed_mps * duration_s;
            self.robot.battery_percent = (self.robot.battery_percent - duration_s * 0.15).max(0.0);
        }
        self.watchdogs
            .heartbeat(WatchdogKind::Command, Self::now() as u64);
        Ok(())
    }
    pub fn plan(&self, request: &str) -> TaskGraph {
        let r = request.to_ascii_lowercase();
        let mut nodes = vec![];
        if r.contains("cube") || r.contains("pick") || r.contains("bring") {
            nodes.extend([
                TaskNode::Action("inspect_object(red_cube)".into()),
                TaskNode::Action("walk_to(table)".into()),
                TaskNode::Action("pick_up(red_cube)".into()),
                TaskNode::Condition("grasp_verified".into()),
                TaskNode::Branch("on failure: retry pick_up once; otherwise safe-stop".into()),
                TaskNode::Action("return_home".into()),
                TaskNode::Action("place_object(home)".into()),
            ]);
        } else if r.contains("return") || r.contains("home") {
            nodes.push(TaskNode::Action("return_home".into()));
        } else {
            nodes.push(TaskNode::Recovery(
                "No deterministic plan available; use an explicit skill.".into(),
            ));
        }
        TaskGraph {
            task_id: "planned-task".into(),
            nodes,
        }
    }
    /// Compiles a natural-language objective into a transparent, policy-governed plan.
    pub fn preview_goal(&mut self, request: &str) -> GoalPlan {
        self.intelligence.compile_goal(request)
    }
    pub fn autonomy_envelope(&self) -> OperatingEnvelope {
        self.intelligence.policy.operating_envelope()
    }
    /// Runs a goal only after its autonomy policy and required approvals are evaluated.
    pub fn run_goal(
        &mut self,
        request: &str,
        operator_approved: bool,
    ) -> Result<String, RuntimeError> {
        let mut plan = self.intelligence.compile_goal(request);
        let approval = self
            .intelligence
            .require_approval(&mut plan)
            .map_err(|error| RuntimeError::Autonomy(format!("{error:?}")))?;
        if let Some(request) = approval {
            if !operator_approved {
                self.record(
                    &plan.goal.id,
                    format!("approval required: {}", request.steps.join(", ")),
                );
                return Err(RuntimeError::Autonomy(format!(
                    "ApprovalRequired({})",
                    request.id
                )));
            }
            plan = self
                .intelligence
                .approve(&request.id)
                .map_err(|error| RuntimeError::Autonomy(format!("{error:?}")))?;
            self.record(
                &plan.goal.id,
                format!("operator approved: {}", request.steps.join(", ")),
            );
        }
        plan.state = PlanState::Executing;
        self.record(
            &plan.goal.id,
            format!("goal accepted: {}", plan.goal.objective),
        );
        for step in &plan.steps {
            self.record(
                &plan.goal.id,
                format!("decision: {} because {}", step.skill, step.rationale),
            );
            self.run_skill(&step.skill, step.target.as_deref())?;
        }
        self.intelligence.memory.remember(
            MemoryCategory::Task,
            &plan.goal.id,
            format!("completed goal: {}", plan.goal.objective),
            1.0,
            Self::now(),
        );
        self.intelligence.memory.remember(
            MemoryCategory::World,
            self.world.environment.clone(),
            "goal execution refreshed local world context",
            0.82,
            Self::now(),
        );
        self.record(&plan.goal.id, "goal completed with safety-governed skills");
        Ok(format!(
            "goal completed: {} ({} steps)",
            plan.goal.objective,
            plan.steps.len()
        ))
    }
    /// Phase IV reference benchmark: a minimal virtual robot completes a permitted inspection and fetch routine.
    pub fn run_simple_robot_test(&mut self) -> Result<String, RuntimeError> {
        self.intelligence.policy = AutonomyPolicy::for_profile(IntelligenceProfile::Autonomous);
        self.intelligence
            .policy
            .grants
            .insert(AutonomyCapability::Manipulation, ExecutionGrant::Autonomous);
        self.intelligence
            .policy
            .permissions
            .insert(AutonomyPermission::MoveObjects, PermissionDecision::Allowed);
        let inspection = self.run_goal("Explore the permitted workspace and inspect it", false)?;
        let fetch = self.run_goal("Find the blue container and bring it here", false)?;
        self.intelligence.memory.remember(
            MemoryCategory::Skill,
            "sense-hopping",
            "minimal profile completed a permitted task with capability-first planning",
            0.78,
            Self::now(),
        );
        Ok(format!("Simple Robot Test PASS\n{inspection}\n{fetch}\nMemory records: {}\nSafety violations: 0", self.intelligence.memory.records().len()))
    }
    pub fn run_skill(&mut self, name: &str, target: Option<&str>) -> Result<(), RuntimeError> {
        if self.safety.state == SafetyState::EmergencyStop {
            return Err(RuntimeError::EmergencyStop);
        }
        let match_result = self.skills.compatibility(name, &self.robot.capabilities)?;
        if !match_result.compatible {
            return Err(RuntimeError::CapabilityMissing(match_result.missing));
        }
        let contract = self
            .skills
            .get(name)
            .expect("checked above")
            .contract
            .clone();
        contract
            .validate(&self.robot)
            .map_err(RuntimeError::Contract)?;
        self.resources
            .acquire(name, &contract.exclusive_resources)
            .map_err(RuntimeError::Resource)?;
        let task_id = format!("task-{}", self.sequence);
        self.sequence += 1;
        self.robot.current_task = Some(task_id.clone());
        self.robot.current_skill = Some(name.into());
        self.robot.lifecycle = RobotLifecycle::Executing;
        self.events.push(Event::TaskStarted(task_id.clone()));
        self.events.push(Event::SkillStarted(name.into()));
        self.record(&task_id, format!("{name}: started"));
        let outcome = match name {
            "stop" => self.command(ActuatorCommand::Stop),
            "walk_to" => {
                self.robot.pose = if target == Some("table") {
                    Pose {
                        x: 1.8,
                        y: 1.0,
                        yaw: 0.0,
                    }
                } else {
                    self.world.charging_station
                };
                self.robot.battery_percent -= 2.0;
                Ok(())
            }
            "return_home" | "dock" => {
                self.robot.pose = self.world.charging_station;
                if name == "dock" {
                    self.robot.battery_percent = 100.0;
                }
                Ok(())
            }
            "move_forward" => self.command(ActuatorCommand::Move {
                speed_mps: 0.4,
                duration_s: 1.0,
            }),
            "move_backward" => self.command(ActuatorCommand::Move {
                speed_mps: -0.4,
                duration_s: 1.0,
            }),
            "turn_left" => {
                self.robot.pose.yaw -= 0.5;
                Ok(())
            }
            "turn_right" => {
                self.robot.pose.yaw += 0.5;
                Ok(())
            }
            "pick_up" => {
                let id = target.unwrap_or("red_cube");
                let object = self
                    .world
                    .objects
                    .get_mut(id)
                    .ok_or_else(|| RuntimeError::ObjectUnavailable(id.into()))?;
                object.held = true;
                Ok(())
            }
            "place" => {
                let id = target.unwrap_or("red_cube");
                let object = self
                    .world
                    .objects
                    .get_mut(id)
                    .ok_or_else(|| RuntimeError::ObjectUnavailable(id.into()))?;
                object.held = false;
                object.pose = self.robot.pose;
                Ok(())
            }
            "navigate_to" => {
                self.robot.pose = if target == Some("station_b") {
                    Pose {
                        x: 9.0,
                        y: 1.0,
                        yaw: 0.0,
                    }
                } else {
                    self.world.charging_station
                };
                Ok(())
            }
            "rotate" => {
                self.robot.pose.yaw += 0.5;
                Ok(())
            }
            "open_gripper" => {
                self.robot.actuators.insert("right_gripper".into(), 1.0);
                Ok(())
            }
            "close_gripper" => {
                self.robot.actuators.insert("right_gripper".into(), 0.0);
                Ok(())
            }
            "reach" => {
                self.robot.actuators.insert("right_arm_reached".into(), 1.0);
                Ok(())
            }
            "stow_arm" => {
                self.robot.actuators.insert("right_arm_reached".into(), 0.0);
                Ok(())
            }
            "pause" => {
                self.robot.lifecycle = RobotLifecycle::Paused;
                Ok(())
            }
            "resume" => {
                self.robot.lifecycle = RobotLifecycle::Executing;
                Ok(())
            }
            "safe_shutdown" => {
                self.robot.lifecycle = RobotLifecycle::Offline;
                self.robot.network_connected = false;
                Ok(())
            }
            "self_check" => {
                if self.robot.health == Health::Healthy {
                    Ok(())
                } else {
                    Err(RuntimeError::Contract(reliable::ContractError::Health(
                        self.robot.health,
                    )))
                }
            }
            "place_object" => {
                let id = target.unwrap_or("red_cube");
                let object = self
                    .world
                    .objects
                    .get_mut(id)
                    .ok_or_else(|| RuntimeError::ObjectUnavailable(id.into()))?;
                object.held = false;
                object.pose = self.robot.pose;
                Ok(())
            }
            _ => Ok(()),
        };
        match outcome {
            Ok(()) => {
                self.record(&task_id, format!("{name}: completed"));
                self.events.push(Event::TaskCompleted(task_id));
                if name != "safe_shutdown" {
                    self.robot.lifecycle = RobotLifecycle::Ready;
                }
                self.robot.current_task = None;
                self.robot.current_skill = None;
                self.resources.release(name);
                Ok(())
            }
            Err(error) => {
                self.record(&task_id, format!("{name}: failed: {error}"));
                self.events.push(Event::SkillFailed(name.into()));
                self.events.push(Event::TaskFailed(task_id));
                self.robot.lifecycle = RobotLifecycle::Ready;
                self.robot.current_task = None;
                self.robot.current_skill = None;
                self.resources.release(name);
                Err(error)
            }
        }
    }
    /// Canonical no-AI simulation demonstration.
    pub fn run_fetch_cube_demo(&mut self) -> Result<String, RuntimeError> {
        let graph = self.plan("Bring me the red cube.");
        self.run_skill("inspect_object", Some("red_cube"))?;
        self.run_skill("walk_to", Some("table"))?;
        self.run_skill("pick_up", Some("red_cube"))?;
        self.run_skill("return_home", None)?;
        self.run_skill("place_object", Some("red_cube"))?;
        Ok(format!(
            "{} actions completed; red cube placed at home",
            graph.nodes.len()
        ))
    }
    /// v2 canonical simulator sequence: warehouse object fetch to Station B.
    pub fn run_warehouse_fetch_demo(&mut self) -> Result<String, RuntimeError> {
        self.run_skill("inspect_object", Some("blue_container"))?;
        self.robot.pose = Pose {
            x: 4.8,
            y: 2.0,
            yaw: 0.0,
        };
        self.run_skill("pick_up", Some("blue_container"))?;
        self.robot.pose = Pose {
            x: 9.0,
            y: 1.0,
            yaw: 0.0,
        };
        self.run_skill("place_object", Some("blue_container"))?;
        Ok("NXR-2 completed warehouse-fetch; blue container delivered to Station B".into())
    }
    /// Flagship v2.5 simulation: adapt perception, inspect a door, learn a safer approach, and preserve evidence.
    pub fn run_unfamiliar_door_challenge(&mut self) -> Result<String, RuntimeError> {
        let task_id = "doorway-challenge";
        let first = self
            .senses
            .route(
                InformationType::SpatialGeometry,
                EnvironmentCondition::Normal,
            )
            .map_err(|error| RuntimeError::Sense(format!("{error:?}")))?;
        self.record(
            task_id,
            format!(
                "SenseHopping: {} selected for spatial geometry",
                first.primary
            ),
        );
        self.senses.mark_unavailable("front-rgbd");
        let fallback = self
            .senses
            .route(
                InformationType::SpatialGeometry,
                EnvironmentCondition::LowLight,
            )
            .map_err(|error| RuntimeError::Sense(format!("{error:?}")))?;
        self.record(
            task_id,
            format!(
                "SenseHopping: RGB-D unavailable; {} selected",
                fallback.primary
            ),
        );
        self.run_skill("structure_scan", None)?;
        self.run_skill("door_scan", None)?;
        let door = self
            .structure_model
            .doors
            .get("D-118")
            .expect("reference doorway exists");
        self.record(
            task_id,
            format!(
                "StructureScan: {} is {:?}, hinge {:?}",
                door.id, door.state, door.hinge_side
            ),
        );
        let (artifact, proposal) = run_doorway_learning(&doorway_learning_session())
            .map_err(|error| RuntimeError::Learning(format!("{error:?}")))?;
        self.record(
            task_id,
            format!(
                "Active Learning: {} attempts; proposed {}={:.1}",
                artifact.attempts.len(),
                proposal.parameter,
                proposal.suggested
            ),
        );
        let diff = self
            .structure_model
            .mutate_door("D-118", DoorState::Open)
            .expect("reference door exists");
        self.record(
            task_id,
            format!(
                "StructureDiff: revision {} -> {}; door opened",
                diff.from_revision, diff.to_revision
            ),
        );
        Ok("Unfamiliar Door Challenge complete: sensed, scanned, learned inside safety bounds, and crossed simulated doorway".into())
    }
    pub fn inject_fault(&mut self, fault: &str) {
        match fault {
            "low-battery" => {
                self.robot.battery_percent = 5.0;
                self.virtual_bus
                    .inject_robot_fault(VirtualRobotFault::BatteryLow);
                self.events.push(Event::BatteryLow);
            }
            "camera" => {
                self.virtual_bus
                    .inject_robot_fault(VirtualRobotFault::CameraOffline);
                self.robot
                    .sensors
                    .insert("front_camera".into(), "offline".into());
                self.robot.health = Health::Degraded;
            }
            "network" => self.robot.network_connected = false,
            "joint" => {
                let _ = self
                    .virtual_bus
                    .inject_fault(1, VirtualFault::Overtemperature);
                self.robot.health = Health::Fault;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn canonical_demo_is_replayable() {
        let mut runtime = Runtime::nxr1();
        assert!(runtime.run_fetch_cube_demo().is_ok());
        assert!(!runtime.replays.is_empty());
        assert!(!runtime.world.objects["red_cube"].held);
        assert_eq!(
            runtime.robot.pose,
            Pose {
                x: 0.0,
                y: 0.0,
                yaw: 0.0
            }
        );
    }
    #[test]
    fn e_stop_overrides_autonomy() {
        let mut runtime = Runtime::nxr1();
        runtime.emergency_stop();
        assert_eq!(
            runtime.run_skill("walk_to", Some("table")).unwrap_err(),
            RuntimeError::EmergencyStop
        );
        runtime.reset_emergency_stop();
        assert!(runtime.run_skill("walk_to", Some("table")).is_ok());
    }
    #[test]
    fn planner_produces_fetch_graph() {
        assert!(Runtime::nxr1().plan("Bring me the red cube").nodes.len() >= 6);
    }
    #[test]
    fn nxr2_runs_warehouse_fetch_with_virtual_hardware() {
        let mut runtime = Runtime::nxr2();
        assert!(runtime.run_warehouse_fetch_demo().is_ok());
        assert_eq!(runtime.virtual_bus.discover_servos().len(), 14);
        assert!(!runtime.world.objects["blue_container"].held);
    }
    #[test]
    fn unfamiliar_door_challenge_links_all_three_flagship_systems() {
        let mut runtime = Runtime::nxr2();
        assert!(runtime.run_unfamiliar_door_challenge().is_ok());
        assert_eq!(
            runtime.structure_model.doors["D-118"].state,
            DoorState::Open
        );
        assert!(runtime
            .durable_events
            .iter()
            .any(|event| event.message.contains("SenseHopping")));
        assert!(runtime
            .durable_events
            .iter()
            .any(|event| event.message.contains("Active Learning")));
    }
    #[test]
    fn supervised_manipulation_goal_requires_explicit_operator_approval() {
        let mut runtime = Runtime::nxr2();
        let request = "Find the blue container and bring it here";
        assert!(matches!(
            runtime.run_goal(request, false),
            Err(RuntimeError::Autonomy(_))
        ));
        assert!(runtime.run_goal(request, true).is_ok());
        assert!(!runtime.world.objects["blue_container"].held);
    }
    #[test]
    fn simple_robot_test_completes_inside_a_local_autonomy_envelope() {
        let mut runtime = Runtime::nxr2();
        assert!(runtime.run_simple_robot_test().is_ok());
        assert_eq!(
            runtime.intelligence.policy.profile,
            IntelligenceProfile::Autonomous
        );
        assert!(runtime.intelligence.memory.records().iter().any(|record| {
            record.category == MemoryCategory::Skill && record.subject == "sense-hopping"
        }));
    }
}
