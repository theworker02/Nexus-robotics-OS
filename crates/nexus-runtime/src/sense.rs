//! SenseHopping: health-aware semantic information routing and fusion.
use nexus_core::Health;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum InformationType {
    ObstacleDistance,
    SpatialGeometry,
    ObjectIdentity,
    NearFieldClearance,
    DoorGeometry,
}
#[derive(Clone, Debug)]
pub struct SenseProvider {
    pub id: String,
    pub provides: Vec<InformationType>,
    pub accuracy: f32,
    pub latency_ms: u32,
    pub range_m: f32,
    pub confidence: f32,
    pub health: Health,
    pub cost: f32,
    pub available: bool,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EnvironmentCondition {
    Normal,
    LowLight,
    ReflectiveSurfaces,
    HighAcousticNoise,
}
#[derive(Clone, Debug)]
pub struct SensePlan {
    pub requirement: InformationType,
    pub primary: String,
    pub secondary: Option<String>,
    pub fused: Vec<String>,
    pub reason: String,
}
#[derive(Clone, Debug)]
pub struct SenseHistoryEntry {
    pub requirement: InformationType,
    pub selected: String,
    pub confidence: f32,
    pub reason: String,
}
#[derive(Default)]
pub struct SenseRouter {
    providers: Vec<SenseProvider>,
    pub history: Vec<SenseHistoryEntry>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SenseError {
    NoProvider(InformationType),
}
impl SenseRouter {
    pub fn with_providers(providers: Vec<SenseProvider>) -> Self {
        Self {
            providers,
            history: vec![],
        }
    }
    pub fn providers(&self) -> &[SenseProvider] {
        &self.providers
    }
    pub fn route(
        &mut self,
        requirement: InformationType,
        condition: EnvironmentCondition,
    ) -> Result<SensePlan, SenseError> {
        let mut candidates: Vec<_> = self
            .providers
            .iter()
            .filter(|provider| {
                provider.available
                    && provider.health == Health::Healthy
                    && provider.provides.contains(&requirement)
            })
            .collect();
        candidates
            .sort_by(|left, right| score(right, condition).total_cmp(&score(left, condition)));
        let primary = candidates
            .first()
            .ok_or(SenseError::NoProvider(requirement))?;
        let secondary = candidates.get(1);
        let should_fuse = matches!(condition, EnvironmentCondition::ReflectiveSurfaces)
            && secondary.is_some()
            || primary.confidence < 0.85 && secondary.is_some();
        let plan = SensePlan {
            requirement,
            primary: primary.id.clone(),
            secondary: secondary.map(|provider| provider.id.clone()),
            fused: if should_fuse {
                vec![primary.id.clone(), secondary.expect("checked").id.clone()]
            } else {
                vec![primary.id.clone()]
            },
            reason: format!(
                "health={:?}, confidence={:.2}, latency={}ms",
                primary.health, primary.confidence, primary.latency_ms
            ),
        };
        self.history.push(SenseHistoryEntry {
            requirement,
            selected: plan.primary.clone(),
            confidence: primary.confidence,
            reason: plan.reason.clone(),
        });
        Ok(plan)
    }
    pub fn mark_unavailable(&mut self, id: &str) {
        if let Some(provider) = self.providers.iter_mut().find(|provider| provider.id == id) {
            provider.available = false;
        }
    }
}
fn score(provider: &SenseProvider, condition: EnvironmentCondition) -> f32 {
    let environment_penalty = match condition {
        EnvironmentCondition::LowLight if provider.id.contains("rgb") => 0.35,
        EnvironmentCondition::HighAcousticNoise if provider.id.contains("audio") => 0.35,
        _ => 1.0,
    };
    provider.accuracy * provider.confidence * environment_penalty + provider.range_m / 100.0
        - provider.latency_ms as f32 / 10_000.0
        - provider.cost / 100.0
}
pub fn nxr2_senses() -> SenseRouter {
    SenseRouter::with_providers(vec![
        SenseProvider {
            id: "front-rgbd".into(),
            provides: vec![
                InformationType::ObstacleDistance,
                InformationType::SpatialGeometry,
                InformationType::ObjectIdentity,
                InformationType::DoorGeometry,
            ],
            accuracy: 0.91,
            latency_ms: 30,
            range_m: 6.0,
            confidence: 0.93,
            health: Health::Healthy,
            cost: 0.4,
            available: true,
        },
        SenseProvider {
            id: "front-lidar".into(),
            provides: vec![
                InformationType::ObstacleDistance,
                InformationType::SpatialGeometry,
                InformationType::DoorGeometry,
            ],
            accuracy: 0.96,
            latency_ms: 25,
            range_m: 12.0,
            confidence: 0.96,
            health: Health::Healthy,
            cost: 0.3,
            available: true,
        },
        SenseProvider {
            id: "proximity-ring".into(),
            provides: vec![
                InformationType::NearFieldClearance,
                InformationType::ObstacleDistance,
            ],
            accuracy: 0.88,
            latency_ms: 8,
            range_m: 0.4,
            confidence: 0.98,
            health: Health::Healthy,
            cost: 0.1,
            available: true,
        },
    ])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn router_hops_when_preferred_provider_fails() {
        let mut router = nxr2_senses();
        assert_eq!(
            router
                .route(
                    InformationType::SpatialGeometry,
                    EnvironmentCondition::Normal
                )
                .unwrap()
                .primary,
            "front-lidar"
        );
        router.mark_unavailable("front-lidar");
        assert_eq!(
            router
                .route(
                    InformationType::SpatialGeometry,
                    EnvironmentCondition::Normal
                )
                .unwrap()
                .primary,
            "front-rgbd"
        );
        assert_eq!(router.history.len(), 2);
    }
}
