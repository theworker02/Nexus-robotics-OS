# Nexus Proving Ground — `move_forward` NXR-2 physics certification

**Skill:** `move_forward@2.6.0`  
**Robot profile:** NXR-2 Gazebo model  
**Validation:** **L2 — PHYSICS VERIFIED**  
**Date:** 2026-08-26

## Executed path

```text
Nexus physics adapter
  → ROS 2 /cmd_vel (geometry_msgs/msg/Twist)
  → ros_gz_bridge
  → Gazebo /cmd_vel (gz.msgs.Twist)
  → NXR-2 DiffDrive system
  → Gazebo /model/nxr2/odometry
  → ros_gz_bridge
  → ROS 2 /nxr2/odometry (nav_msgs/msg/Odometry)
  → measured assertion
```

## Command

```powershell
docker compose -f proving-ground/docker/compose.yml --profile physics-e2e run --build --rm nexus-physics-e2e
```

## Result

The live scenario completed with exit code `0` and recorded:

```text
NEXUS_PHYSICS_ASSERTION PASS: move-forward displacement 0.650 m;
ROS 2 cmd_vel -> Gazebo transport -> NXR-2 base
```

The assertion requires at least 0.20 m of bridged odometry displacement after the command is issued. It is a physics simulation result only; it does not establish HIL or physical-robot performance.
