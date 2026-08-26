//! Fleet primitives and conservative compatibility-aware scheduling.
use nexus_core::{CapabilityManifest, Health};
use std::collections::BTreeMap;
#[derive(Clone, Debug)]
pub struct FleetRobot {
    pub id: String,
    pub group: String,
    pub capabilities: CapabilityManifest,
    pub health: Health,
    pub battery_percent: f32,
    pub workload: u8,
    pub connected: bool,
}
#[derive(Default)]
pub struct Fleet {
    robots: BTreeMap<String, FleetRobot>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FleetError {
    NoCompatibleRobot,
}
impl Fleet {
    pub fn register(&mut self, robot: FleetRobot) {
        self.robots.insert(robot.id.clone(), robot);
    }
    pub fn status(&self) -> impl Iterator<Item = &FleetRobot> {
        self.robots.values()
    }
    pub fn assign(
        &self,
        requirements: &[&str],
        group: Option<&str>,
    ) -> Result<&FleetRobot, FleetError> {
        self.robots
            .values()
            .filter(|robot| {
                robot.connected
                    && robot.health == Health::Healthy
                    && robot.battery_percent >= 25.0
                    && group.is_none_or(|group| robot.group == group)
                    && requirements
                        .iter()
                        .all(|requirement| robot.capabilities.supports(requirement))
            })
            .max_by(|left, right| score(left).total_cmp(&score(right)))
            .ok_or(FleetError::NoCompatibleRobot)
    }
}
fn score(robot: &FleetRobot) -> f32 {
    robot.battery_percent - f32::from(robot.workload) * 10.0
}
#[cfg(test)]
mod tests {
    use super::*;
    use nexus_core::Capability;
    #[test]
    fn scheduler_prefers_healthier_lower_load_robot() {
        let manifest = CapabilityManifest {
            robot_id: "a".into(),
            name: "a".into(),
            architecture: "sim".into(),
            capabilities: [Capability::new("vision.rgb")].into_iter().collect(),
        };
        let mut fleet = Fleet::default();
        fleet.register(FleetRobot {
            id: "low".into(),
            group: "lab".into(),
            capabilities: manifest.clone(),
            health: Health::Healthy,
            battery_percent: 40.0,
            workload: 3,
            connected: true,
        });
        fleet.register(FleetRobot {
            id: "high".into(),
            group: "lab".into(),
            capabilities: manifest,
            health: Health::Healthy,
            battery_percent: 80.0,
            workload: 0,
            connected: true,
        });
        assert_eq!(
            fleet.assign(&["vision.rgb"], Some("lab")).unwrap().id,
            "high"
        );
    }
}
