# Hardware validation boundary

Nexus reports verification levels honestly:

- **Software-tested** — unit or contract tests passed.
- **Simulation-tested** — deterministic or physics simulation passed.
- **HIL-tested** — a hardware-in-the-loop setup passed.
- **Hardware-validated** — the documented physical configuration passed validated tests.

Docker and VirtualBus validate software behavior. They do not validate physical dynamics, electrical behavior, or mechanical safety.
