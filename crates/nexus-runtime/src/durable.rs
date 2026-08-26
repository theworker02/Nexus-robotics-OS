//! Minimal append-only event and safe task recovery records for local-first runtime use.
use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableEvent {
    pub timestamp_ms: u128,
    pub category: String,
    pub subject: String,
    pub message: String,
}
pub struct DurableEventLog {
    path: PathBuf,
}
impl DurableEventLog {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, std::io::Error> {
        let path = path.into();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(Self { path })
    }
    pub fn append(&self, event: &DurableEvent) -> Result<(), std::io::Error> {
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            event.timestamp_ms, event.category, event.subject, event.message
        );
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?
            .write_all(line.as_bytes())
    }
    pub fn read(&self) -> Result<Vec<DurableEvent>, std::io::Error> {
        let content = fs::read_to_string(&self.path).unwrap_or_default();
        Ok(content.lines().filter_map(parse).collect())
    }
}
fn parse(line: &str) -> Option<DurableEvent> {
    let fields: Vec<_> = line.splitn(4, '\t').collect();
    Some(DurableEvent {
        timestamp_ms: fields.first()?.parse().ok()?,
        category: fields.get(1)?.to_string(),
        subject: fields.get(2)?.to_string(),
        message: fields.get(3)?.to_string(),
    })
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoveryDecision {
    ResumeNonMotion,
    RestartStep,
    RequireOperator,
    Abort,
}
pub fn recovery_decision(
    was_physical_motion: bool,
    robot_state_reconciled: bool,
) -> RecoveryDecision {
    if was_physical_motion {
        RecoveryDecision::RequireOperator
    } else if robot_state_reconciled {
        RecoveryDecision::ResumeNonMotion
    } else {
        RecoveryDecision::RestartStep
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn persisted_motion_never_auto_resumes() {
        assert_eq!(
            recovery_decision(true, true),
            RecoveryDecision::RequireOperator
        );
    }
}
