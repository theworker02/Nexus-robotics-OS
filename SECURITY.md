# Security Policy

## Supported versions

The current release candidate is **Nexus Robotics OS 4.2.0-rc.2**. This project is early-stage and support is best-effort. Release, nightly, and development builds may differ; users should identify the exact commit and version when reporting an issue.

## Safety boundary

Nexus is not certified for physical robot safety. Models and external tools do not receive raw actuator authority. Every adapter that can move hardware must route commands through deterministic capability, permission, operating-envelope, resource, and Safety Governor checks, and provide an independent emergency-stop path.

## Current controls

- Capability and permission checks before skill execution.
- Explicit safety metadata in skill manifests.
- Joint, speed, vendor, and operating-envelope limits with structured rejection.
- Emergency stop cancels autonomous work and requires explicit reset.
- No automatic motion resume after restart or recovery.
- Offline, local simulation by default; telemetry is not uploaded by this repository’s default workflows.

## Threat model

Treat model output, skills, external MCP services, wireless devices, integrations, peer robots, network services, and environment text as untrusted. They must not rewrite safety policy, permissions, secrets, or release evidence. Remote model routing and live physical transports remain configuration- and validation-dependent.

## Reporting

Please report vulnerabilities privately to the repository maintainers through the repository’s configured private security channel when available. Do not publish exploit details until a fix is available. Include the affected version or commit, reproduction steps, impact, and whether hardware or only simulation is affected.
