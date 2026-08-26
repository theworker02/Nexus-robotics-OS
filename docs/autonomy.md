# Adaptive autonomy

Nexus Intelligence Layer (NIL) is the policy-governed planning layer above Nexus skills. It is deliberately not a direct actuator interface. Every accepted plan still executes through skill compatibility checks, contracts, Safety Governor, resource arbitration, the selected adapter, and replay events.

## Profiles and operating envelope

The runtime provides `Manual`, `Assisted`, `Supervised`, `Autonomous`, and `Custom` profiles. A policy independently describes execution grants for navigation, perception, exploration, manipulation, and learning; it also describes discrete permissions for moving objects, opening doors, and leaving an allowed zone.

`nexus autonomy envelope` renders the effective policy as three lists: allowed, approval-required, and prohibited. The shipped policies always deny leaving the allowed zone. Learning is simulation-only. The autonomous reference profile allows moving objects, but door opening still requires approval.

## Goals and approval

`nexus goal plan <objective>` creates an explainable `GoalPlan` with an objective, ordered skill steps, target, capability, optional permission, rationale, risk, and expected duration. The v3.0 compiler supports deterministic reference objectives for object delivery, permitted-area inspection, and readiness checks. It is not a general-purpose language-model planner.

`nexus goal run <objective> --approve` evaluates the plan and dispatches it only when the policy permits every step. The `--approve` flag is an explicit local operator action for this CLI invocation; it is not a durable authorization and it does not enable hardware control.

```powershell
cargo run -p nexus-cli -- goal plan Find the blue container and bring it here --no-ai
cargo run -p nexus-cli -- goal run Find the blue container and bring it here --approve --no-ai
```

## Current boundary

NIL is local, deterministic, and simulation-first in v3.0. It has no remote task dispatch, voice interface, mobile app, persistent profile store, cloud execution client, or direct hardware-autonomy activation. Those additions must preserve the same approval, authentication, Safety Governor, and validation boundaries.

## Reference benchmark

`nexus bench simple-robot --no-ai` runs a permitted environment inspection followed by blue-container delivery on the NXR-2 deterministic profile. It records task and skill experience memory and reports zero safety violations for that controlled run. This is a local functional benchmark only; it does not earn L2, L3, L4, or L5 validation.
