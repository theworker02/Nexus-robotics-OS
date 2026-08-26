# Architecture

Nexus separates the robot operating model from adapter-specific technology. `nexus-core` contains the stable contracts; adapters translate hardware or simulation into those contracts; `nexus-runtime` owns lifecycle, skills, planning, telemetry, replay, and safety dispatch. `nexus-gateway` keeps local safety and telemetry behavior alive when a central connection is unavailable.

```text
Task / skill → capability compatibility → Safety Governor → adapter → robot or simulator
                    ↓                        ↓
              NCM manifest             logs / events / replay
```

The built-in NXR-1 adapter is deliberately deterministic so the canonical scenario is reproducible without an AI model. A future ROS 2 adapter must map topics, actions, and services into the same `RobotState` and NCM contracts, rather than becoming the core architecture.

## Proving Ground

`nexus-runtime` also owns the local Proving Ground evidence model. Skills and adapters execute through the same runtime and VirtualRobotBus used by regular deterministic scenarios; there is no special success-only adapter path for certification. WorldForge makes the test inputs deterministic from a seed, and reports separate software, virtual hardware, physics, adversarial, HIL, and robot evidence. The Gazebo / ROS bridge scaffold is a future L2 executor, not a claim that L2 has run.
