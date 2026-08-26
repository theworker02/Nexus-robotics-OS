//! SDK for vendor and community adapter authors.
pub use nexus_core::v2::{AdapterError, CapabilityManifestV2, DeploymentMode, RobotAdapter};

/// Emits a conservative adapter package skeleton. No actuator permissions are assumed.
pub fn scaffold(name: &str, transport: &str, capabilities: &[&str]) -> String {
    format!("package:\n  name: {name}\n  version: 0.1.0\n  type: adapter\ncompatibility:\n  nexus: \">=2.0\"\nadapter:\n  transport: {transport}\ncapabilities:\n{}permissions:\n  - telemetry.read\n", capabilities.iter().map(|capability| format!("  - {capability}\n")).collect::<String>())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scaffold_keeps_control_unprivileged() {
        let output = scaffold("demo", "serial", &["vision.rgb"]);
        assert!(output.contains("telemetry.read"));
        assert!(!output.contains("servo.control"));
    }
}
