//! Software watchdogs and dead-man motion expiry.
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WatchdogKind {
    Command,
    AdapterHeartbeat,
    Telemetry,
    Runtime,
    Skill,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatchdogAlert {
    pub kind: WatchdogKind,
    pub age_ms: u64,
    pub limit_ms: u64,
}
#[derive(Default)]
pub struct WatchdogSet {
    last_seen_ms: BTreeMap<WatchdogKind, u64>,
    limits_ms: BTreeMap<WatchdogKind, u64>,
}
impl WatchdogSet {
    pub fn with_defaults() -> Self {
        let limits_ms = [
            (WatchdogKind::Command, 500),
            (WatchdogKind::AdapterHeartbeat, 2_000),
            (WatchdogKind::Telemetry, 5_000),
            (WatchdogKind::Runtime, 2_000),
            (WatchdogKind::Skill, 10_000),
        ]
        .into_iter()
        .collect();
        Self {
            last_seen_ms: BTreeMap::new(),
            limits_ms,
        }
    }
    pub fn heartbeat(&mut self, kind: WatchdogKind, now_ms: u64) {
        self.last_seen_ms.insert(kind, now_ms);
    }
    pub fn check(&self, now_ms: u64) -> Vec<WatchdogAlert> {
        self.limits_ms
            .iter()
            .filter_map(|(kind, limit)| {
                self.last_seen_ms
                    .get(kind)
                    .map(|seen| (kind, now_ms.saturating_sub(*seen), limit))
            })
            .filter(|(_, age, limit)| age > *limit)
            .map(|(kind, age_ms, limit_ms)| WatchdogAlert {
                kind: *kind,
                age_ms,
                limit_ms: *limit_ms,
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stale_motion_heartbeat_is_detected() {
        let mut watchdogs = WatchdogSet::with_defaults();
        watchdogs.heartbeat(WatchdogKind::Command, 0);
        assert!(watchdogs
            .check(501)
            .iter()
            .any(|alert| alert.kind == WatchdogKind::Command));
    }
}
