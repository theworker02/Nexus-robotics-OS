//! Community-built compatibility contracts for publicly described Nori-style software flows.
//! This crate is not affiliated with or endorsed by Nori Robotics.

use nexus_core::{v2::*, Capability, CapabilityManifest, Health};
use nexus_protocol::VirtualBus;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Session {
    pub id: String,
    pub status: String,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionConfig {
    pub name: String,
    pub dataset: Option<String>,
}
/// Stable training-environment boundary. Network clients can implement this without leaking vendor APIs into Nexus Core.
pub trait TrainingEnvironment {
    fn sessions(&self) -> Result<Vec<Session>, AdapterError>;
    fn start_session(&mut self, config: SessionConfig) -> Result<String, AdapterError>;
    fn stop_session(&mut self, id: &str) -> Result<(), AdapterError>;
}
#[derive(Default)]
pub struct NoriLabAdapter {
    connected: bool,
    sessions: BTreeMap<String, Session>,
    counter: u64,
}
impl TrainingEnvironment for NoriLabAdapter {
    fn sessions(&self) -> Result<Vec<Session>, AdapterError> {
        Ok(self.sessions.values().cloned().collect())
    }
    fn start_session(&mut self, config: SessionConfig) -> Result<String, AdapterError> {
        if !self.connected {
            return Err(AdapterError::Unavailable(
                "Nori-Lab adapter is disconnected".into(),
            ));
        }
        self.counter += 1;
        let id = format!("nori-session-{}", self.counter);
        self.sessions.insert(
            id.clone(),
            Session {
                id: id.clone(),
                status: format!("running: {}", config.name),
            },
        );
        Ok(id)
    }
    fn stop_session(&mut self, id: &str) -> Result<(), AdapterError> {
        self.sessions
            .get_mut(id)
            .ok_or_else(|| AdapterError::OperationDenied("unknown session".into()))?
            .status = "stopped".into();
        Ok(())
    }
}
impl RobotAdapter for NoriLabAdapter {
    fn name(&self) -> &str {
        "nori-community"
    }
    fn mode(&self) -> DeploymentMode {
        DeploymentMode::Compatibility
    }
    fn connect(&mut self) -> Result<(), AdapterError> {
        self.connected = true;
        Ok(())
    }
    fn disconnect(&mut self) -> Result<(), AdapterError> {
        self.connected = false;
        Ok(())
    }
    fn discover(&mut self) -> Result<CapabilityManifestV2, AdapterError> {
        if !self.connected {
            return Err(AdapterError::Unavailable("connect before discovery".into()));
        }
        Ok(nori_a3_manifest())
    }
    fn health(&self) -> Health {
        if self.connected {
            Health::Healthy
        } else {
            Health::Offline
        }
    }
    fn telemetry(&self) -> Result<BTreeMap<String, String>, AdapterError> {
        Ok([
            (
                "integration.label".into(),
                "Community Integration — simulated compatibility only".into(),
            ),
            ("training.sessions".into(), self.sessions.len().to_string()),
        ]
        .into_iter()
        .collect())
    }
    fn stop(&mut self) -> Result<(), AdapterError> {
        for session in self.sessions.values_mut() {
            session.status = "stopped".into();
        }
        Ok(())
    }
}

/// Servo diagnostics bridge; commands are intentionally absent, so it cannot bypass Nexus safety.
pub struct FeetechMotorLabAdapter<'a> {
    bus: &'a VirtualBus,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ServoDiagnostic {
    pub id: u8,
    pub model: String,
    pub position_deg: f32,
    pub temperature_c: f32,
    pub voltage_v: f32,
    pub health: Health,
}
impl<'a> FeetechMotorLabAdapter<'a> {
    pub fn new(bus: &'a VirtualBus) -> Self {
        Self { bus }
    }
    pub fn scan(&self) -> Vec<ServoDiagnostic> {
        self.bus
            .discover_servos()
            .into_iter()
            .map(|servo| ServoDiagnostic {
                id: servo.id,
                model: servo.model.clone(),
                position_deg: servo.position_deg,
                temperature_c: servo.temperature_c,
                voltage_v: servo.voltage_v,
                health: servo.health(),
            })
            .collect()
    }
}

pub fn nori_a3_manifest() -> CapabilityManifestV2 {
    let capabilities = [
        "manipulators.left.gripper",
        "manipulators.right.gripper",
        "vision.rgb",
        "sensing.lidar",
        "audio.microphones",
        "audio.speakers",
        "power.battery",
    ];
    let base = CapabilityManifest {
        robot_id: "nori-a3-sim".into(),
        name: "SIMULATED Nori-compatible profile".into(),
        architecture: "simulated".into(),
        capabilities: capabilities.into_iter().map(Capability::new).collect(),
    };
    let mut manifest = CapabilityManifestV2::from_static(base, "nori-community-profile");
    manifest.records.insert(
        "manipulators.left.dof".into(),
        CapabilityRecord {
            capability: Capability::new("manipulators.left.dof"),
            enabled: true,
            provenance: CapabilityProvenance {
                source_type: CapabilitySourceType::Static,
                provider: "public nori-a3 example profile".into(),
                observed_at_ms: None,
            },
        },
    );
    manifest.records.insert(
        "vision.depth".into(),
        CapabilityRecord {
            capability: Capability::new("vision.depth"),
            enabled: false,
            provenance: CapabilityProvenance {
                source_type: CapabilitySourceType::Derived,
                provider: "not inferred from RGB-only public profile".into(),
                observed_at_ms: None,
            },
        },
    );
    manifest
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compatibility_adapter_discovers_without_vendor_api() {
        let mut adapter = NoriLabAdapter::default();
        adapter.connect().unwrap();
        let manifest = adapter.discover().unwrap();
        assert!(manifest.supports("vision.rgb"));
        assert!(!manifest.supports("vision.depth"));
        let id = adapter
            .start_session(SessionConfig {
                name: "evaluation".into(),
                dataset: None,
            })
            .unwrap();
        adapter.stop_session(&id).unwrap();
    }
    #[test]
    fn motorlab_reads_virtual_servo_diagnostics() {
        let bus = VirtualBus::nxr2();
        assert_eq!(FeetechMotorLabAdapter::new(&bus).scan().len(), 14);
    }
}
