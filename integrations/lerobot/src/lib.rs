//! Loss-aware bridge for LeRobot-like datasets and episodes.
use std::collections::BTreeMap;
#[derive(Clone, Debug, PartialEq)]
pub struct Observation {
    pub timestamp_ms: u64,
    pub state: Vec<f32>,
    pub cameras: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Action {
    pub timestamp_ms: u64,
    pub values: Vec<f32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Episode {
    pub id: String,
    pub observations: Vec<Observation>,
    pub actions: Vec<Action>,
    pub metadata: BTreeMap<String, String>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConversionReport {
    pub lossless: bool,
    pub warnings: Vec<String>,
}
pub fn import_episode(episode: Episode) -> Result<(Episode, ConversionReport), String> {
    if episode.observations.iter().any(|o| o.state.is_empty()) {
        return Err("observation state vectors cannot be empty".into());
    }
    Ok((
        episode,
        ConversionReport {
            lossless: true,
            warnings: vec![],
        },
    ))
}
pub fn export_episode(episode: Episode) -> (Episode, ConversionReport) {
    let mut warnings = vec![];
    if episode.observations.iter().any(|o| o.cameras.is_empty()) {
        warnings.push("one or more observations have no camera references".into());
    }
    (
        episode,
        ConversionReport {
            lossless: warnings.is_empty(),
            warnings,
        },
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bridge_preserves_action_observation_timestamps() {
        let episode = Episode {
            id: "e1".into(),
            observations: vec![Observation {
                timestamp_ms: 10,
                state: vec![1.0],
                cameras: BTreeMap::new(),
                metadata: BTreeMap::new(),
            }],
            actions: vec![Action {
                timestamp_ms: 12,
                values: vec![0.5],
            }],
            metadata: BTreeMap::new(),
        };
        let (converted, report) = import_episode(episode).unwrap();
        assert!(report.lossless);
        assert_eq!(converted.actions[0].timestamp_ms, 12);
    }
}
