//! Nexus Intelligence Layer (NIL): policy-governed goals above skills and safety.

use crate::reliable::SkillLifecycle;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IntelligenceProfile {
    Manual,
    Assisted,
    Supervised,
    Autonomous,
    Custom,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AutonomyCapability {
    Navigation,
    Manipulation,
    Perception,
    Learning,
    Exploration,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExecutionGrant {
    Denied,
    Manual,
    Supervised,
    Autonomous,
    SimulationOnly,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum AutonomyPermission {
    OpenDoors,
    MoveObjects,
    LeaveZone,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PermissionDecision {
    Denied,
    Ask,
    Allowed,
}
#[derive(Clone, Debug)]
pub struct AutonomyPolicy {
    pub profile: IntelligenceProfile,
    pub grants: BTreeMap<AutonomyCapability, ExecutionGrant>,
    pub permissions: BTreeMap<AutonomyPermission, PermissionDecision>,
    pub allowed_zones: BTreeSet<String>,
    pub speed_scale: f32,
    pub minimum_skill_validation: SkillLifecycle,
}
impl AutonomyPolicy {
    pub fn for_profile(profile: IntelligenceProfile) -> Self {
        let baseline = match profile {
            IntelligenceProfile::Manual => ExecutionGrant::Manual,
            IntelligenceProfile::Assisted => ExecutionGrant::Supervised,
            IntelligenceProfile::Supervised => ExecutionGrant::Supervised,
            IntelligenceProfile::Autonomous => ExecutionGrant::Autonomous,
            IntelligenceProfile::Custom => ExecutionGrant::Denied,
        };
        let mut grants = BTreeMap::new();
        for capability in [
            AutonomyCapability::Navigation,
            AutonomyCapability::Manipulation,
            AutonomyCapability::Perception,
            AutonomyCapability::Exploration,
        ] {
            grants.insert(capability, baseline);
        }
        grants.insert(AutonomyCapability::Learning, ExecutionGrant::SimulationOnly);
        let permissions = [
            (AutonomyPermission::OpenDoors, PermissionDecision::Ask),
            (
                AutonomyPermission::MoveObjects,
                if profile == IntelligenceProfile::Autonomous {
                    PermissionDecision::Allowed
                } else {
                    PermissionDecision::Ask
                },
            ),
            (AutonomyPermission::LeaveZone, PermissionDecision::Denied),
        ]
        .into_iter()
        .collect();
        Self {
            profile,
            grants,
            permissions,
            allowed_zones: ["home".into(), "warehouse".into(), "workspace".into()]
                .into_iter()
                .collect(),
            speed_scale: if profile == IntelligenceProfile::Manual {
                0.5
            } else {
                1.0
            },
            minimum_skill_validation: SkillLifecycle::SimulationValidated,
        }
    }
    pub fn grant(&self, capability: AutonomyCapability) -> ExecutionGrant {
        self.grants
            .get(&capability)
            .copied()
            .unwrap_or(ExecutionGrant::Denied)
    }
    pub fn permission(&self, permission: AutonomyPermission) -> PermissionDecision {
        self.permissions
            .get(&permission)
            .copied()
            .unwrap_or(PermissionDecision::Denied)
    }
    pub fn operating_envelope(&self) -> OperatingEnvelope {
        let mut allowed = vec![];
        let mut approval_required = vec![];
        let mut prohibited = vec![];
        for (capability, label) in [
            (AutonomyCapability::Navigation, "Navigate designated zones"),
            (AutonomyCapability::Perception, "Use permitted sensors"),
            (AutonomyCapability::Exploration, "Explore allowed zones"),
        ] {
            match self.grant(capability) {
                ExecutionGrant::Autonomous => allowed.push(label.into()),
                ExecutionGrant::Supervised | ExecutionGrant::Manual => {
                    approval_required.push(label.into())
                }
                _ => prohibited.push(label.into()),
            }
        }
        for (permission, label) in [
            (AutonomyPermission::MoveObjects, "Move objects"),
            (AutonomyPermission::OpenDoors, "Open doors"),
            (AutonomyPermission::LeaveZone, "Leave allowed zones"),
        ] {
            match self.permission(permission) {
                PermissionDecision::Allowed => allowed.push(label.into()),
                PermissionDecision::Ask => approval_required.push(label.into()),
                PermissionDecision::Denied => prohibited.push(label.into()),
            }
        }
        OperatingEnvelope {
            allowed,
            approval_required,
            prohibited,
        }
    }
}
#[derive(Clone, Debug)]
pub struct OperatingEnvelope {
    pub allowed: Vec<String>,
    pub approval_required: Vec<String>,
    pub prohibited: Vec<String>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GoalRisk {
    Low,
    Medium,
    High,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanState {
    Preview,
    AwaitingApproval,
    Approved,
    Executing,
    Completed,
    Rejected,
}
#[derive(Clone, Debug)]
pub struct Goal {
    pub id: String,
    pub request: String,
    pub objective: String,
}
#[derive(Clone, Debug)]
pub struct PlanStep {
    pub skill: String,
    pub target: Option<String>,
    pub capability: AutonomyCapability,
    pub permission: Option<AutonomyPermission>,
    pub rationale: String,
}
#[derive(Clone, Debug)]
pub struct GoalPlan {
    pub goal: Goal,
    pub steps: Vec<PlanStep>,
    pub required_capabilities: Vec<String>,
    pub expected_duration_s: u64,
    pub risk: GoalRisk,
    pub state: PlanState,
}
#[derive(Clone, Debug)]
pub struct ApprovalRequest {
    pub id: String,
    pub goal_id: String,
    pub reason: String,
    pub steps: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AutonomyError {
    ApprovalRequired(String),
    Denied(String),
    InvalidApproval(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum MemoryCategory {
    World,
    Task,
    Skill,
    Failure,
    Routine,
    Operator,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryRetention {
    Persistent,
    SessionOnly,
    Expiring,
    Disabled,
}
#[derive(Clone, Debug)]
pub struct MemoryRecord {
    pub id: String,
    pub category: MemoryCategory,
    pub subject: String,
    pub detail: String,
    pub confidence: f32,
    pub timestamp_ms: u128,
    pub retention: MemoryRetention,
}
#[derive(Clone, Debug)]
pub struct ExperienceMemory {
    records: Vec<MemoryRecord>,
    policies: BTreeMap<MemoryCategory, MemoryRetention>,
}
impl Default for ExperienceMemory {
    fn default() -> Self {
        let policies = [
            (MemoryCategory::World, MemoryRetention::Persistent),
            (MemoryCategory::Task, MemoryRetention::Persistent),
            (MemoryCategory::Skill, MemoryRetention::Persistent),
            (MemoryCategory::Failure, MemoryRetention::Persistent),
            (MemoryCategory::Routine, MemoryRetention::Persistent),
            (MemoryCategory::Operator, MemoryRetention::Disabled),
        ]
        .into_iter()
        .collect();
        Self {
            records: vec![],
            policies,
        }
    }
}
impl ExperienceMemory {
    pub fn remember(
        &mut self,
        category: MemoryCategory,
        subject: impl Into<String>,
        detail: impl Into<String>,
        confidence: f32,
        timestamp_ms: u128,
    ) {
        let retention = self
            .policies
            .get(&category)
            .copied()
            .unwrap_or(MemoryRetention::Disabled);
        if retention == MemoryRetention::Disabled {
            return;
        }
        self.records.push(MemoryRecord {
            id: format!("mem-{}", self.records.len() + 1),
            category,
            subject: subject.into(),
            detail: detail.into(),
            confidence,
            timestamp_ms,
            retention,
        });
    }
    pub fn records(&self) -> &[MemoryRecord] {
        &self.records
    }
    pub fn forget(&mut self, id: &str) -> bool {
        let before = self.records.len();
        self.records.retain(|record| record.id != id);
        before != self.records.len()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputePlacement {
    OnRobot,
    EdgeServer,
    Cloud,
}
#[derive(Clone, Debug)]
pub struct ComputeRequest {
    pub requires_private_data: bool,
    pub network_available: bool,
    pub latency_sensitive: bool,
    pub estimated_cost: f32,
}
#[derive(Clone, Debug)]
pub struct ComputeRouter {
    pub cloud_allowed: bool,
    pub cost_limit: f32,
}
impl Default for ComputeRouter {
    fn default() -> Self {
        Self {
            cloud_allowed: false,
            cost_limit: 0.0,
        }
    }
}
impl ComputeRouter {
    pub fn choose(&self, request: &ComputeRequest) -> ComputePlacement {
        if request.requires_private_data || request.latency_sensitive || !request.network_available
        {
            ComputePlacement::OnRobot
        } else if self.cloud_allowed && request.estimated_cost <= self.cost_limit {
            ComputePlacement::Cloud
        } else {
            ComputePlacement::EdgeServer
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AutomationClass {
    Informational,
    System,
    Motion,
    Manipulation,
    SafetyCritical,
}
#[derive(Clone, Debug)]
pub enum AutomationTrigger {
    BatteryBelow(f32),
    SensorDegraded(String),
    DoorStateChanged(String),
}
#[derive(Clone, Debug)]
pub enum AutomationAction {
    RunSkill(String),
    RebuildSensePlan,
    NotifyOperator(String),
}
#[derive(Clone, Debug)]
pub struct AutomationRule {
    pub id: String,
    pub class: AutomationClass,
    pub trigger: AutomationTrigger,
    pub actions: Vec<AutomationAction>,
    pub enabled: bool,
}
#[derive(Clone, Debug)]
pub struct Routine {
    pub id: String,
    pub name: String,
    pub steps: Vec<PlanStep>,
    pub enabled: bool,
}

#[derive(Clone, Debug)]
pub struct IntelligenceLayer {
    pub policy: AutonomyPolicy,
    pub memory: ExperienceMemory,
    pub compute: ComputeRouter,
    pub routines: BTreeMap<String, Routine>,
    pub rules: Vec<AutomationRule>,
    plans: BTreeMap<String, GoalPlan>,
    approvals: BTreeMap<String, ApprovalRequest>,
    next_goal: u64,
}
impl IntelligenceLayer {
    pub fn supervised() -> Self {
        Self::new(IntelligenceProfile::Supervised)
    }
    pub fn new(profile: IntelligenceProfile) -> Self {
        Self {
            policy: AutonomyPolicy::for_profile(profile),
            memory: ExperienceMemory::default(),
            compute: ComputeRouter::default(),
            routines: BTreeMap::new(),
            rules: vec![],
            plans: BTreeMap::new(),
            approvals: BTreeMap::new(),
            next_goal: 1,
        }
    }
    pub fn compile_goal(&mut self, request: &str) -> GoalPlan {
        let request_lower = request.to_ascii_lowercase();
        let id = format!("goal-{}", self.next_goal);
        self.next_goal += 1;
        let (objective, steps, risk) = if request_lower.contains("blue")
            || request_lower.contains("bring")
            || request_lower.contains("container")
        {
            (
                "Locate, retrieve, and deliver the requested object".into(),
                vec![
                    step(
                        "inspect_object",
                        Some("blue_container"),
                        AutonomyCapability::Perception,
                        None,
                        "confirm target identity",
                    ),
                    step(
                        "walk_to",
                        Some("table"),
                        AutonomyCapability::Navigation,
                        None,
                        "approach known target area",
                    ),
                    step(
                        "pick_up",
                        Some("blue_container"),
                        AutonomyCapability::Manipulation,
                        Some(AutonomyPermission::MoveObjects),
                        "acquire target",
                    ),
                    step(
                        "return_home",
                        None,
                        AutonomyCapability::Navigation,
                        None,
                        "return to home location",
                    ),
                    step(
                        "place_object",
                        Some("blue_container"),
                        AutonomyCapability::Manipulation,
                        Some(AutonomyPermission::MoveObjects),
                        "deliver target",
                    ),
                ],
                GoalRisk::Medium,
            )
        } else if request_lower.contains("explore")
            || request_lower.contains("clear")
            || request_lower.contains("inspect")
        {
            (
                "Inspect the permitted environment and report observations".into(),
                vec![
                    step(
                        "scan_area",
                        Some("workspace"),
                        AutonomyCapability::Perception,
                        None,
                        "observe permitted area",
                    ),
                    step(
                        "structure_scan",
                        None,
                        AutonomyCapability::Perception,
                        None,
                        "update structural context",
                    ),
                    step(
                        "return_home",
                        None,
                        AutonomyCapability::Navigation,
                        None,
                        "return to safe home",
                    ),
                ],
                GoalRisk::Low,
            )
        } else {
            (
                "Execute a safe, capability-compatible inspection".into(),
                vec![step(
                    "self_check",
                    None,
                    AutonomyCapability::Perception,
                    None,
                    "verify robot readiness",
                )],
                GoalRisk::Low,
            )
        };
        let plan = GoalPlan {
            goal: Goal {
                id: id.clone(),
                request: request.into(),
                objective,
            },
            required_capabilities: vec!["vision".into(), "mobility".into()],
            expected_duration_s: 120,
            risk,
            state: PlanState::Preview,
            steps,
        };
        self.plans.insert(id, plan.clone());
        plan
    }
    pub fn require_approval(
        &mut self,
        plan: &mut GoalPlan,
    ) -> Result<Option<ApprovalRequest>, AutonomyError> {
        let mut approval_steps = vec![];
        for step in &plan.steps {
            match self.policy.grant(step.capability) {
                ExecutionGrant::Denied => {
                    return Err(AutonomyError::Denied(format!(
                        "{} is denied by autonomy policy",
                        step.skill
                    )))
                }
                ExecutionGrant::Manual | ExecutionGrant::Supervised => {
                    approval_steps.push(step.skill.clone())
                }
                _ => {}
            }
            if let Some(permission) = step.permission {
                match self.policy.permission(permission) {
                    PermissionDecision::Denied => {
                        return Err(AutonomyError::Denied(format!(
                            "permission denied for {}",
                            step.skill
                        )))
                    }
                    PermissionDecision::Ask => approval_steps.push(step.skill.clone()),
                    PermissionDecision::Allowed => {}
                }
            }
        }
        if approval_steps.is_empty() {
            plan.state = PlanState::Approved;
            self.plans.insert(plan.goal.id.clone(), plan.clone());
            return Ok(None);
        }
        let request = ApprovalRequest {
            id: format!("approval-{}", plan.goal.id),
            goal_id: plan.goal.id.clone(),
            reason: "Plan includes supervised autonomy or an approval-gated permission.".into(),
            steps: approval_steps,
        };
        plan.state = PlanState::AwaitingApproval;
        self.plans.insert(plan.goal.id.clone(), plan.clone());
        self.approvals.insert(request.id.clone(), request.clone());
        Ok(Some(request))
    }
    pub fn approve(&mut self, approval_id: &str) -> Result<GoalPlan, AutonomyError> {
        let request = self
            .approvals
            .remove(approval_id)
            .ok_or_else(|| AutonomyError::InvalidApproval(approval_id.into()))?;
        let plan = self
            .plans
            .get_mut(&request.goal_id)
            .ok_or_else(|| AutonomyError::InvalidApproval(request.goal_id.clone()))?;
        plan.state = PlanState::Approved;
        Ok(plan.clone())
    }
    pub fn plan(&self, goal_id: &str) -> Option<&GoalPlan> {
        self.plans.get(goal_id)
    }
}
fn step(
    skill: &str,
    target: Option<&str>,
    capability: AutonomyCapability,
    permission: Option<AutonomyPermission>,
    rationale: &str,
) -> PlanStep {
    PlanStep {
        skill: skill.into(),
        target: target.map(str::to_string),
        capability,
        permission,
        rationale: rationale.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn supervised_manipulation_requests_approval() {
        let mut nil = IntelligenceLayer::supervised();
        let mut plan = nil.compile_goal("Bring the blue container here");
        assert!(nil.require_approval(&mut plan).unwrap().is_some());
    }
    #[test]
    fn autonomous_policy_never_allows_leaving_zone() {
        let policy = AutonomyPolicy::for_profile(IntelligenceProfile::Autonomous);
        assert_eq!(
            policy.permission(AutonomyPermission::LeaveZone),
            PermissionDecision::Denied
        );
    }
    #[test]
    fn compute_router_keeps_private_work_local() {
        let router = ComputeRouter {
            cloud_allowed: true,
            cost_limit: 2.0,
        };
        assert_eq!(
            router.choose(&ComputeRequest {
                requires_private_data: true,
                network_available: true,
                latency_sensitive: false,
                estimated_cost: 1.0
            }),
            ComputePlacement::OnRobot
        );
    }
}
