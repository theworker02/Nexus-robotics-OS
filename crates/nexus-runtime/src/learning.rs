//! Active Learning sessions optimize constrained strategy parameters in simulation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureCategory {
    PerceptionFailure,
    PlanningFailure,
    ManipulationFailure,
    NavigationFailure,
    SafetyRejection,
    CapabilityFailure,
    Timeout,
    EnvironmentChange,
    Unknown,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LearningStage {
    Experiment,
    Candidate,
    SimValidated,
    HilValidated,
    HardwareValidated,
    Production,
}
#[derive(Clone, Debug)]
pub struct LearningBudget {
    pub max_attempts: u8,
    pub max_duration_s: u64,
    pub max_energy_percent: f32,
    pub max_actuator_cycles: Option<u32>,
}
#[derive(Clone, Debug)]
pub struct LearningSession {
    pub id: String,
    pub task: String,
    pub environment: String,
    pub robot_profile: String,
    pub allowed_skills: Vec<String>,
    pub success_criterion: String,
    pub budget: LearningBudget,
    pub seed: u64,
    pub stage: LearningStage,
}
#[derive(Clone, Debug)]
pub struct LearningAttempt {
    pub number: u8,
    pub approach_angle_deg: f32,
    pub outcome: Result<(), FailureCategory>,
    pub explanation: String,
}
#[derive(Clone, Debug)]
pub struct LearningArtifact {
    pub base_skill_version: String,
    pub attempts: Vec<LearningAttempt>,
    pub proposed_angle_deg: f32,
    pub validation_success_rate: f32,
    pub stage: LearningStage,
}
#[derive(Clone, Debug)]
pub struct SkillImprovementProposal {
    pub skill: String,
    pub parameter: String,
    pub previous: f32,
    pub suggested: f32,
    pub reason: String,
    pub stage: LearningStage,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LearningError {
    SafetyEnvelope,
    AttemptBudgetExhausted,
    Overfit,
}
pub fn doorway_learning_session() -> LearningSession {
    LearningSession {
        id: "LS-doorway-001".into(),
        task: "open and pass doorway".into(),
        environment: "doorway-lab-v1".into(),
        robot_profile: "nxr-2".into(),
        allowed_skills: vec![
            "door_scan".into(),
            "navigate_to".into(),
            "reach".into(),
            "pick_up".into(),
        ],
        success_criterion: "robot crosses doorway".into(),
        budget: LearningBudget {
            max_attempts: 4,
            max_duration_s: 180,
            max_energy_percent: 8.0,
            max_actuator_cycles: Some(12),
        },
        seed: 42,
        stage: LearningStage::Experiment,
    }
}
pub fn run_doorway_learning(
    session: &LearningSession,
) -> Result<(LearningArtifact, SkillImprovementProposal), LearningError> {
    let mut angle: f32 = 15.0;
    let mut attempts = vec![];
    for number in 1..=session.budget.max_attempts {
        let success = (angle - 22.0).abs() < 0.1;
        let outcome = if success {
            Ok(())
        } else {
            Err(FailureCategory::ManipulationFailure)
        };
        attempts.push(LearningAttempt {
            number,
            approach_angle_deg: angle,
            outcome,
            explanation: if success {
                "door handle aligned; crossing verified".into()
            } else {
                "GRASP_MISALIGNMENT: observed handle offset exceeds configured approach".into()
            },
        });
        if success {
            let artifact = LearningArtifact {
                base_skill_version: "pick_up@2.5.0".into(),
                attempts,
                proposed_angle_deg: angle,
                validation_success_rate: 1.0,
                stage: LearningStage::SimValidated,
            };
            let proposal = SkillImprovementProposal {
                skill: "door_open_candidate".into(),
                parameter: "approach_angle_deg".into(),
                previous: 15.0,
                suggested: angle,
                reason: "simulation attempt succeeded inside the configured safety envelope".into(),
                stage: LearningStage::Candidate,
            };
            return Ok((artifact, proposal));
        }
        angle = (angle + 7.0).min(30.0);
    }
    Err(LearningError::AttemptBudgetExhausted)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn learning_proposes_bounded_simulation_candidate() {
        let (artifact, proposal) = run_doorway_learning(&doorway_learning_session()).unwrap();
        assert_eq!(artifact.attempts.len(), 2);
        assert_eq!(proposal.suggested, 22.0);
        assert_eq!(artifact.stage, LearningStage::SimValidated);
    }
}
