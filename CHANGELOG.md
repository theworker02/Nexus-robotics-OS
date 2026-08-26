# Changelog

All notable changes are documented here. Dates are included only for releases prepared in this repository.

## [4.2.0-rc.2] — 2026-08-26

### Added

- Public release-candidate identity, compatibility policy, privacy policy, release policy, and author attribution guidance.
- GitHub Sponsors funding configuration for `theworker02`.
- Explicit release-readiness, safety, validation, and known-limitation language across public surfaces.

### Changed

- Workspace and citation metadata now identify `4.2.0-rc.2`.
- README, roadmap, security policy, and website metadata use the Nexus Robotics OS 4.2 release-candidate identity.
- Compatibility and validation claims are documented as evidence-based and retain `NOT RUN`/unvalidated states where applicable.

### Security

- Public security policy now documents model, MCP, wireless, integration, and robot-control trust boundaries.

### Known limitations

- This is a release candidate. HIL, physical-robot, vendor-verified, live remote-provider, and live transport claims remain dependent on corresponding evidence.

## [2.6.0] — 2026-08-26

### Added

- Nexus Proving Ground: reproducible validation evidence, six explicit validation levels, certification reports, and `nexus prove` commands.
- WorldForge deterministic, seeded scenario parameters and adversarial virtual-hardware trials.
- VirtualRobotBus device coverage for camera, lidar, IMU, microphone, battery, servo torque estimates, communication latency, and injected faults.
- Gazebo Harmonic / ROS 2 / `ros_gz_bridge` container scaffold for future L2 physics evidence.

### Security

- Virtual, physics, HIL, and robot evidence are distinct. Proving Ground reports never award L2–L5 without corresponding recorded execution.
- Recorded a successful finite Gazebo Harmonic L2 backend smoke run; it does not elevate any individual skill above L1.
- Added the NXR-2 Gazebo model, ROS 2 `cmd_vel` / odometry transport, and a measured `move_forward@2.6.0` L2 physics assertion (0.650 m observed displacement).

## [2.5.0] — 2026-08-26

### Added

- NCM 2.5 semantic capability constraints and quality contracts.
- Reliable skill lifecycle contracts, resource arbitration, watchdogs, durable event records, restart policy, and fleet scheduling.
- 33 simulation-validated built-in skills and reliability specifications.
- SenseHopping, StructureScan, DoorScan, Active Learning, and the Unfamiliar Door Challenge simulator scenario.

### Security

- Dead-man command watchdog and no-auto-resume policy for persisted physical motion.

## [2.0.0] — 2026-08-26

### Added

- NCM 2.0 provenance contracts, adapter interfaces, motion policies, task v2 contracts, and digital-twin/shadow-mode foundations.
- NXR-2 warehouse simulator, VirtualBus, virtual servo faults, Nori community compatibility, LeRobot bridge, NRP package validation, Docker test stack, and product website.

### Security

- Unsigned NRP packages are visibly development-only; vendor limits can only tighten motion policy; containers run non-root with no added capabilities.

## [0.5.0] — 2026-08-26

- Established the offline-first Phase I runtime foundation.
- Added NXR-1, 15 built-in skills, deterministic fetch-cube demonstration, safety governor, capability engine, CLI, tests, and documentation.
