# NEXUS PROVING GROUND

**Skill:** door-scan@2.6.0-dev  
**Robot profile:** NXR-2  
**WorldForge seed:** 483208  

## Results

| Metric | Result |
| --- | --- |
| Trials | 12 |
| Success | 8 / 12 (66.7%) |
| Safe aborts | 4 |
| Safety violations | 0 |
| Average simulated runtime | 13085 ms |

## Validation evidence

- **L0 — SOFTWARE VERIFIED — PASS:** Runtime, contract, schema, state-machine, and Proving Ground tests are exercised locally.
- **L1 — VIRTUAL HARDWARE VERIFIED — PASS:** Each trial used the adapter-facing VirtualRobotBus; exact injected device conditions are retained in its trial records.
- **L2 — PHYSICS VERIFIED — NOT RUN:** Gazebo Harmonic headless evidence has not been recorded by this local run.
- **L3 — ADVERSARIALLY VERIFIED — NOT RUN:** Adversarial virtual-hardware trials completed, but L3 cannot be earned until the L2 Gazebo physics prerequisite is executed and recorded.
- **L4 — HIL VERIFIED — NOT RUN:** No real sensor, controller, or robot component was connected.
- **L5 — ROBOT VERIFIED — NOT RUN:** No physical robot demonstration was performed.

## Benchmarks

- SenseHopping recovery: 4 / 4 fault trials
- StructureScan door-width MAE: 0.020 m
- Active Learning randomized-door success: 91.0%
- Active Learning altered-geometry success: 84.0%
- Active Learning safe behavior: 100.0%

## Physical validation

**NOT YET PERFORMED.** Virtual hardware and deterministic world tests do not prove motor torque, mechanical backlash, calibration, thermal behavior, battery runtime, real sensor behavior, cable faults, wheel slip, collision forces, manufacturing tolerances, emergency-stop hardware, or human/robot interaction.
