# Nexus Proving Ground 1.0

Nexus Proving Ground is the local certification subsystem for demonstrating what a capability has actually passed before it is connected to a robot.

## Validation levels

| Level | Claim | Required evidence |
| --- | --- | --- |
| L0 | Software Verified | Unit, schema, state-machine, and contract checks pass. |
| L1 | Virtual Hardware Verified | The adapter-facing VirtualRobotBus completes relevant device and fault flows. |
| L2 | Physics Verified | The behavior is repeatedly executed in a recorded physics backend such as Gazebo Harmonic. |
| L3 | Adversarially Verified | L2 evidence plus deterministic fault, latency, sensor, and environment variation evidence. |
| L4 | HIL Verified | At least one physical sensor, controller, or robot component is included in the recorded run. |
| L5 | Robot Verified | The documented behavior is demonstrated on the physical robot configuration. |

Levels are cumulative: L3 cannot be claimed without L2 evidence. A higher level never erases the limitations of lower-fidelity simulation.

## Report invariants

- Each report identifies skill version, robot profile, WorldForge seed, trial count, outcomes, safety violations, and evidence per level.
- A level is `NOT RUN` unless its matching executor recorded it during that run.
- A virtual safe abort is not reported as task success.
- No level authorizes bypassing manufacturer limits, hardware emergency stops, guarding, operator procedures, or physical validation.
- `production-ready` is prohibited unless a project-specific release gate has reviewed earned evidence; compiling alone is never certification.

## WorldForge

WorldForge derives door geometry, handle height, lighting, floor friction, camera/lidar noise, latency, and start pose from a seed. Replaying the seed must reproduce the test world inputs.

## Physical exclusions

No L0-L3 evidence proves actual motor torque, backlash, calibration, thermal characteristics, battery runtime, camera behavior, cable faults, wheel slip, collision forces, manufacturing variation, emergency-stop hardware, or human/robot interaction.
