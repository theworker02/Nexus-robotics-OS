# Adapter 1.0

Adapters implement `connect`, `disconnect`, `discover`, `health`, `telemetry`, and priority `stop`. Nexus-originated motion passes from planner to skill to Safety Governor to adapter; an adapter must not bypass this boundary.
