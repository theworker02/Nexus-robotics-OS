//! Local robot gateway state machine. Networking transports plug in above this safety-preserving core.
use nexus_core::SafetyState;
use std::collections::VecDeque;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GatewayState {
    Discovered,
    Connecting,
    Authenticating,
    Syncing,
    Ready,
    Degraded,
    Offline,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GatewayError {
    NotReady(GatewayState),
    Offline,
    SafetyStop,
}
impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for GatewayError {}
#[derive(Clone, Debug)]
pub struct Gateway {
    pub state: GatewayState,
    pub safety: SafetyState,
    telemetry_queue: VecDeque<String>,
    pub local_emergency_control: bool,
}
impl Default for Gateway {
    fn default() -> Self {
        Self {
            state: GatewayState::Discovered,
            safety: SafetyState::Safe,
            telemetry_queue: VecDeque::new(),
            local_emergency_control: true,
        }
    }
}
impl Gateway {
    pub fn advance(&mut self) {
        self.state = match self.state {
            GatewayState::Discovered => GatewayState::Connecting,
            GatewayState::Connecting => GatewayState::Authenticating,
            GatewayState::Authenticating => GatewayState::Syncing,
            GatewayState::Syncing => GatewayState::Ready,
            state => state,
        };
    }
    pub fn accept_new_operation(&self) -> Result<(), GatewayError> {
        if self.safety == SafetyState::EmergencyStop {
            Err(GatewayError::SafetyStop)
        } else if self.state == GatewayState::Offline {
            Err(GatewayError::Offline)
        } else if self.state == GatewayState::Ready {
            Ok(())
        } else {
            Err(GatewayError::NotReady(self.state))
        }
    }
    pub fn queue_telemetry(&mut self, event: impl Into<String>) {
        self.telemetry_queue.push_back(event.into());
    }
    pub fn drain_telemetry(&mut self) -> Vec<String> {
        self.telemetry_queue.drain(..).collect()
    }
    pub fn emergency_stop(&mut self) {
        self.safety = SafetyState::EmergencyStop;
        self.queue_telemetry("safety.triggered");
    }
    pub fn central_connection_lost(&mut self) {
        self.state = GatewayState::Offline;
        self.queue_telemetry("gateway.offline: unsafe new operations denied");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gateway_retains_safety_when_central_connection_drops() {
        let mut gateway = Gateway::default();
        for _ in 0..4 {
            gateway.advance();
        }
        assert!(gateway.accept_new_operation().is_ok());
        gateway.central_connection_lost();
        assert_eq!(gateway.accept_new_operation(), Err(GatewayError::Offline));
        assert!(!gateway.drain_telemetry().is_empty());
        gateway.emergency_stop();
        assert_eq!(gateway.safety, SafetyState::EmergencyStop);
    }
}
