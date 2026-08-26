# Compatibility

Compatibility is capability- and evidence-based. Nexus does not claim that one binary supports every robot.

| Integration | Discovery | Telemetry | Skills | Simulation | Current evidence |
| --- | --- | --- | --- | --- | --- |
| NXR-1 / NXR-2 | Deterministic profiles | Local runtime | Built-in contracts | Yes | Software and virtual-hardware evidence varies by scenario |
| Nori community | Simulated public profile | Simulated adapter | Compatibility checked | Yes | Unverified physical hardware |
| ROS 2 | Message/action mapper | Contract surface | Capability mapping | Contract-tested | Live graph transport unvalidated |
| LeRobot | Episode/data bridge | Dataset metadata | Bridge foundation | Fixture-tested | Adapter-dependent |
| Custom hardware | Adapter-defined | Adapter-defined | Capability-driven | VirtualBus where supported | Requires adapter and validation |

Statuses must be updated from executable evidence and must not be described as vendor verified, HIL verified, or physical-robot verified without that evidence. See `docs/hardware-validation.md` and `proving-ground/`.
