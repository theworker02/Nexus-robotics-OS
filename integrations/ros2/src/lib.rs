//! ROS 2 capability discovery contract. Transport binding is intentionally separate.
use nexus_core::{v2::*, Capability, CapabilityManifest, Health};
use std::collections::BTreeMap;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RosEndpoint {
    pub name: String,
    pub type_name: String,
    pub kind: RosEndpointKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RosEndpointKind {
    Topic,
    Service,
    Action,
}
pub fn map_endpoint(endpoint: &RosEndpoint) -> Option<&'static str> {
    match endpoint.type_name.as_str() {
        "sensor_msgs/msg/Image" => Some("vision.rgb"),
        "sensor_msgs/msg/CameraInfo" => Some("vision.camera_calibration"),
        "sensor_msgs/msg/Imu" => Some("sensing.imu"),
        "sensor_msgs/msg/BatteryState" => Some("power.battery"),
        "sensor_msgs/msg/PointCloud2" | "sensor_msgs/msg/LaserScan" => Some("sensing.lidar"),
        "control_msgs/action/FollowJointTrajectory" => Some("manipulators.joint_trajectory"),
        "geometry_msgs/msg/Twist" => Some("locomotion.control"),
        _ => None,
    }
}
pub struct Ros2Adapter {
    endpoints: Vec<RosEndpoint>,
    connected: bool,
    stop_requested: bool,
}
impl Ros2Adapter {
    pub fn new(endpoints: Vec<RosEndpoint>) -> Self {
        Self {
            endpoints,
            connected: false,
            stop_requested: false,
        }
    }

    /// Reports whether the adapter has received the deterministic stop request.
    /// This is an observable contract signal, not a live ROS command publisher.
    pub fn stop_requested(&self) -> bool {
        self.stop_requested
    }
}
impl RobotAdapter for Ros2Adapter {
    fn name(&self) -> &str {
        "ros2"
    }
    fn mode(&self) -> DeploymentMode {
        DeploymentMode::Adapter
    }
    fn connect(&mut self) -> Result<(), AdapterError> {
        self.connected = true;
        self.stop_requested = false;
        Ok(())
    }
    fn disconnect(&mut self) -> Result<(), AdapterError> {
        self.connected = false;
        Ok(())
    }
    fn discover(&mut self) -> Result<CapabilityManifestV2, AdapterError> {
        if !self.connected {
            return Err(AdapterError::Unavailable(
                "ROS 2 graph not connected".into(),
            ));
        }
        let capabilities = self
            .endpoints
            .iter()
            .filter_map(map_endpoint)
            .map(Capability::new)
            .collect();
        let base = CapabilityManifest {
            robot_id: "ros2-discovered".into(),
            name: "ROS 2 discovered robot".into(),
            architecture: "adapter".into(),
            capabilities,
        };
        let mut manifest = CapabilityManifestV2::from_static(base, "ros2-adapter");
        for record in manifest.records.values_mut() {
            record.provenance.source_type = CapabilitySourceType::Discovered;
        }
        Ok(manifest)
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
            ("ros2.endpoints".into(), self.endpoints.len().to_string()),
            ("ros2.connected".into(), self.connected.to_string()),
            (
                "ros2.stop_requested".into(),
                self.stop_requested.to_string(),
            ),
        ]
        .into_iter()
        .collect())
    }
    fn stop(&mut self) -> Result<(), AdapterError> {
        self.stop_requested = true;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adapter_requires_connection_and_reports_lifecycle() {
        let mut adapter = Ros2Adapter::new(vec![]);
        assert_eq!(adapter.health(), Health::Offline);
        assert!(matches!(
            adapter.discover(),
            Err(AdapterError::Unavailable(_))
        ));
        adapter.connect().unwrap();
        assert_eq!(adapter.health(), Health::Healthy);
        adapter.stop().unwrap();
        assert!(adapter.stop_requested());
        let telemetry = adapter.telemetry().unwrap();
        assert_eq!(
            telemetry.get("ros2.stop_requested").map(String::as_str),
            Some("true")
        );
        adapter.disconnect().unwrap();
        assert_eq!(adapter.health(), Health::Offline);
    }

    #[test]
    fn unknown_ros_types_do_not_become_capabilities() {
        let mut adapter = Ros2Adapter::new(vec![RosEndpoint {
            name: "/private/control".into(),
            type_name: "vendor_msgs/msg/UnsafeCommand".into(),
            kind: RosEndpointKind::Topic,
        }]);
        adapter.connect().unwrap();
        let manifest = adapter.discover().unwrap();
        assert!(manifest.records.is_empty());
    }

    #[test]
    fn common_ros_interfaces_map_to_ncm() {
        let mut adapter = Ros2Adapter::new(vec![
            RosEndpoint {
                name: "/cam".into(),
                type_name: "sensor_msgs/msg/Image".into(),
                kind: RosEndpointKind::Topic,
            },
            RosEndpoint {
                name: "/base".into(),
                type_name: "geometry_msgs/msg/Twist".into(),
                kind: RosEndpointKind::Topic,
            },
        ]);
        adapter.connect().unwrap();
        let manifest = adapter.discover().unwrap();
        assert!(manifest.supports("vision.rgb"));
        assert!(manifest.supports("locomotion.control"));
    }
}
