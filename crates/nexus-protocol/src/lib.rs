//! Transport-neutral VirtualBus used to exercise hardware adapters in CI.

use nexus_core::Health;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VirtualServoFaultState {
    None,
    Overtemperature,
    Timeout,
    Disconnected,
    PositionStuck,
    Delayed,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualServo {
    pub id: u8,
    pub model: String,
    pub position_deg: f32,
    pub velocity_deg_s: f32,
    pub temperature_c: f32,
    pub current_a: f32,
    pub voltage_v: f32,
    pub torque_estimate_nm: f32,
    pub communication_latency_ms: u32,
    pub min_deg: f32,
    pub max_deg: f32,
    pub stalled: bool,
    pub connected: bool,
    pub position_failed: bool,
    pub fault_state: VirtualServoFaultState,
}
impl VirtualServo {
    pub fn health(&self) -> Health {
        if !self.connected || self.position_failed || self.temperature_c >= 85.0 {
            Health::Fault
        } else if self.temperature_c >= 70.0 || self.stalled {
            Health::Degraded
        } else {
            Health::Healthy
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VirtualFault {
    Overtemperature,
    ServoTimeout,
    BusDisconnect,
    PositionFailure,
    ResponseDelayed,
}
/// Faults across the complete simulated robot, rather than a single actuator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VirtualRobotFault {
    CameraOffline,
    LidarNoisy,
    BatteryLow,
    MicrophoneOffline,
    ImuOffline,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VirtualCamera {
    pub id: String,
    pub online: bool,
    pub latency_ms: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualLidar {
    pub online: bool,
    pub latency_ms: u32,
    pub noise_std_dev_m: f32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualImu {
    pub online: bool,
    pub latency_ms: u32,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VirtualMicrophone {
    pub online: bool,
    pub latency_ms: u32,
}
#[derive(Clone, Debug, PartialEq)]
pub enum BusError {
    UnknownServo(u8),
    Disconnected(u8),
    PositionOutOfRange {
        id: u8,
        requested: f32,
        min: f32,
        max: f32,
    },
    Faulted(u8),
}
impl std::fmt::Display for BusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for BusError {}

/// In-memory model of serial, CAN, GPIO and device feeds. It is intentionally not a physical validation claim.
#[derive(Clone, Debug, Default)]
pub struct VirtualBus {
    servos: BTreeMap<u8, VirtualServo>,
    pub camera_streams: usize,
    pub cameras: Vec<VirtualCamera>,
    pub lidar: VirtualLidar,
    pub imu: VirtualImu,
    pub microphone: VirtualMicrophone,
    pub imu_online: bool,
    pub battery_percent: f32,
    pub serial_online: bool,
    pub can_online: bool,
    pub gpio_online: bool,
}
/// Compatibility name for the complete simulated robot electronics surface.
///
/// Existing adapters can continue to accept `VirtualBus`; this alias makes the
/// wider device model explicit without introducing an adapter-only test path.
pub type VirtualRobotBus = VirtualBus;
impl VirtualBus {
    pub fn nxr2() -> Self {
        let mut bus = Self {
            camera_streams: 4,
            cameras: vec![
                VirtualCamera {
                    id: "front-rgb".into(),
                    online: true,
                    latency_ms: 28,
                },
                VirtualCamera {
                    id: "front-depth".into(),
                    online: true,
                    latency_ms: 32,
                },
                VirtualCamera {
                    id: "left-rgb".into(),
                    online: true,
                    latency_ms: 31,
                },
                VirtualCamera {
                    id: "right-rgb".into(),
                    online: true,
                    latency_ms: 31,
                },
            ],
            lidar: VirtualLidar {
                online: true,
                latency_ms: 25,
                noise_std_dev_m: 0.01,
            },
            imu: VirtualImu {
                online: true,
                latency_ms: 8,
            },
            microphone: VirtualMicrophone {
                online: true,
                latency_ms: 12,
            },
            imu_online: true,
            battery_percent: 100.0,
            serial_online: true,
            can_online: true,
            gpio_online: true,
            ..Self::default()
        };
        for id in 1..=14 {
            bus.servos.insert(
                id,
                VirtualServo {
                    id,
                    model: "STS3215-sim".into(),
                    position_deg: 0.0,
                    velocity_deg_s: 0.0,
                    temperature_c: 29.0,
                    current_a: 0.0,
                    voltage_v: 12.0,
                    torque_estimate_nm: 0.0,
                    communication_latency_ms: 4,
                    min_deg: -170.0,
                    max_deg: 170.0,
                    stalled: false,
                    connected: true,
                    position_failed: false,
                    fault_state: VirtualServoFaultState::None,
                },
            );
        }
        bus
    }
    pub fn discover_servos(&self) -> Vec<&VirtualServo> {
        self.servos.values().collect()
    }
    pub fn servo(&self, id: u8) -> Result<&VirtualServo, BusError> {
        self.servos.get(&id).ok_or(BusError::UnknownServo(id))
    }
    pub fn set_position(&mut self, id: u8, angle_deg: f32) -> Result<(), BusError> {
        let servo = self.servos.get_mut(&id).ok_or(BusError::UnknownServo(id))?;
        if !servo.connected {
            return Err(BusError::Disconnected(id));
        }
        if servo.position_failed || servo.temperature_c >= 85.0 {
            return Err(BusError::Faulted(id));
        }
        if angle_deg < servo.min_deg || angle_deg > servo.max_deg {
            return Err(BusError::PositionOutOfRange {
                id,
                requested: angle_deg,
                min: servo.min_deg,
                max: servo.max_deg,
            });
        }
        servo.velocity_deg_s = (angle_deg - servo.position_deg).abs();
        servo.position_deg = angle_deg;
        servo.current_a = 0.4;
        servo.torque_estimate_nm = 0.7;
        Ok(())
    }
    pub fn inject_fault(&mut self, id: u8, fault: VirtualFault) -> Result<(), BusError> {
        let servo = self.servos.get_mut(&id).ok_or(BusError::UnknownServo(id))?;
        match fault {
            VirtualFault::Overtemperature => {
                servo.temperature_c = 90.0;
                servo.fault_state = VirtualServoFaultState::Overtemperature;
            }
            VirtualFault::ServoTimeout => {
                servo.stalled = true;
                servo.fault_state = VirtualServoFaultState::Timeout;
            }
            VirtualFault::BusDisconnect => {
                servo.connected = false;
                servo.fault_state = VirtualServoFaultState::Disconnected;
                self.serial_online = false;
            }
            VirtualFault::PositionFailure => {
                servo.position_failed = true;
                servo.fault_state = VirtualServoFaultState::PositionStuck;
            }
            VirtualFault::ResponseDelayed => {
                servo.communication_latency_ms = 400;
                servo.fault_state = VirtualServoFaultState::Delayed;
            }
        };
        Ok(())
    }
    pub fn inject_robot_fault(&mut self, fault: VirtualRobotFault) {
        match fault {
            VirtualRobotFault::CameraOffline => {
                if let Some(camera) = self
                    .cameras
                    .iter_mut()
                    .find(|camera| camera.id == "front-depth")
                {
                    camera.online = false;
                }
                self.camera_streams = self.cameras.iter().filter(|camera| camera.online).count();
            }
            VirtualRobotFault::LidarNoisy => self.lidar.noise_std_dev_m = 0.35,
            VirtualRobotFault::BatteryLow => self.battery_percent = 8.0,
            VirtualRobotFault::MicrophoneOffline => self.microphone.online = false,
            VirtualRobotFault::ImuOffline => self.imu.online = false,
        }
    }
    pub fn telemetry(&self) -> BTreeMap<String, String> {
        let mut values = BTreeMap::new();
        values.insert("bus.serial".into(), self.serial_online.to_string());
        values.insert(
            "battery.percent".into(),
            format!("{:.1}", self.battery_percent),
        );
        values.insert("cameras.online".into(), self.camera_streams.to_string());
        values.insert("lidar.online".into(), self.lidar.online.to_string());
        values.insert(
            "lidar.noise_std_dev_m".into(),
            format!("{:.2}", self.lidar.noise_std_dev_m),
        );
        values.insert("imu.online".into(), self.imu.online.to_string());
        values.insert(
            "microphone.online".into(),
            self.microphone.online.to_string(),
        );
        values.insert(
            "servos.faulted".into(),
            self.servos
                .values()
                .filter(|s| s.health() == Health::Fault)
                .count()
                .to_string(),
        );
        values
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn virtual_servo_models_safe_range_and_faults() {
        let mut bus = VirtualBus::nxr2();
        assert!(bus.set_position(1, 10.0).is_ok());
        assert!(matches!(
            bus.set_position(1, 300.0),
            Err(BusError::PositionOutOfRange { .. })
        ));
        bus.inject_fault(1, VirtualFault::Overtemperature).unwrap();
        assert!(matches!(
            bus.set_position(1, 10.0),
            Err(BusError::Faulted(1))
        ));
    }
    #[test]
    fn virtual_robot_bus_models_sensor_and_latency_faults() {
        let mut bus: VirtualRobotBus = VirtualBus::nxr2();
        bus.inject_fault(1, VirtualFault::ResponseDelayed).unwrap();
        assert_eq!(bus.servo(1).unwrap().communication_latency_ms, 400);
        bus.inject_robot_fault(VirtualRobotFault::CameraOffline);
        bus.inject_robot_fault(VirtualRobotFault::LidarNoisy);
        assert_eq!(bus.camera_streams, 3);
        assert_eq!(bus.lidar.noise_std_dev_m, 0.35);
    }
}
