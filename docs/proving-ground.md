# Nexus Proving Ground

Proving Ground produces repeatable evidence for a Nexus skill before actuator access is considered. It is local-first and writes plain Markdown reports.

```powershell
cargo run -p nexus-cli -- prove skill fetch-object --trials 100 --seed 483208
cargo run -p nexus-cli -- prove skill sense-hopping --trials 100
cargo run -p nexus-cli -- prove all --trials 100
cargo run -p nexus-cli -- prove skill door-scan --output proving-ground/reports/door-scan.md
```

The default runner covers L0 and L1 with deterministic WorldForge worlds and adversarial virtual faults. It deliberately reports L2-L5 as `NOT RUN` until evidence from those environments is recorded.

## VirtualRobotBus faults

The adapter-facing virtual bus presents NXR-2 servo state—position, velocity, current, temperature, voltage, torque estimate, latency, and fault state—along with camera, lidar, IMU, battery, and microphone devices. Proving Ground injects depth camera loss, darkness, lidar noise, servo disconnect/overheat/400 ms response delay, and low battery.

For safety-critical faults, the expected result is a safe abort, not task success. Sense failures verify semantic fallback; for example, a depth camera failure requires LiDAR to become the primary distance source before the task can proceed.

## Physics backend

`proving-ground/docker/compose.yml` is an optional Gazebo Harmonic + ROS 2 Jazzy scaffold. It uses `ros_gz_bridge` as the ROS 2 / Gazebo Transport boundary, following the supported topic-bridge model in the [Gazebo Harmonic ROS 2 integration documentation](https://gazebosim.org/docs/harmonic/ros2_integration/). Its presence does not count as an L2 result; start it, run a scenario, retain the output, and attach the resulting evidence to earn L2.

## Recorded backend evidence

The self-contained Gazebo world now has an actual finite, headless 100-iteration execution record: [Gazebo Harmonic backend smoke](../proving-ground/reports/gazebo-harmonic-backend-smoke-2026-08-26.md). Repeat it with:

```powershell
docker compose -f proving-ground/docker/compose.yml --profile physics-smoke run --rm gazebo-physics-smoke
```

This is L2 **backend** evidence, not L2 evidence for an individual Nexus skill. Skills stay at L1 until their adapter transport, NXR-2 model, task controller, and pass/fail assertions run through Gazebo. L3, HIL, and robot claims remain distinct and require their own executed evidence.

## Skill-specific physics evidence

`move_forward@2.6.0` is now L2 Physics Verified for the NXR-2 reference model. The live controller sends ROS 2 `cmd_vel`, crosses `ros_gz_bridge` into Gazebo’s DiffDrive system, receives bridged odometry, and requires at least 0.20 m of measured displacement. The recorded execution reached 0.650 m: [move-forward physics certification](../proving-ground/reports/move-forward-nxr2-physics-2026-08-26.md).

```powershell
docker compose -f proving-ground/docker/compose.yml --profile physics-e2e run --build --rm nexus-physics-e2e
```

This does not elevate other skills, and it does not earn L3: those claims require their own repeated, randomized physics trials with fault assertions.
