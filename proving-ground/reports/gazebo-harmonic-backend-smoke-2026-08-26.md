# Nexus Proving Ground — Gazebo Harmonic backend smoke

**Date:** 2026-08-26  
**Status:** PASS  
**Scope:** L2 physics-backend readiness only; this is not a skill certificate.

## Recorded execution

```powershell
docker compose -f proving-ground/docker/compose.yml --profile physics run --rm --no-deps gazebo-harmonic gz sim -s -r --headless-rendering --iterations 100 /opt/nexus/worlds/doorway-lab.sdf
```

The self-contained `doorway-lab.sdf` world completed a finite 100-iteration headless Gazebo run with exit code 0. The image was built locally from `osrf/ros:jazzy-desktop` with `ros-jazzy-ros-gz` installed.

## Bridge evidence

The separately started `ros-gz-bridge` process initialized its `/clock` and `/scan` GZ-to-ROS mappings. The ROS / Gazebo container stack remained up after its read-only runtime directories were supplied through `tmpfs`.

## Certification boundary

This evidence verifies that the Nexus Proving Ground physics backend can execute its reference world. It does **not** show that `fetch-object`, SenseHopping, StructureScan, or Active Learning has been driven through Gazebo. Those individual skills therefore remain L1 until an NXR-2 Gazebo model, adapter transport, scenario controller, and per-skill outcome assertions are connected and executed.
