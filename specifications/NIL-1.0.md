# Nexus Intelligence Layer 1.0

## Purpose

NIL provides a policy-governed, explainable layer between a user objective and Nexus skills. It may not directly command a robot, adapter, or actuator.

## Required flow

```text
Objective -> GoalPlan -> policy and approval evaluation -> Skill -> contract -> Safety Governor -> adapter -> robot or simulator
```

An implementation must reject a plan when any required capability is denied, any discrete permission is denied, or a required approval has not been provided. An approval is scoped to one plan instance.

## Policy semantics

Profiles assign `ExecutionGrant` values to autonomy capabilities and `PermissionDecision` values to discrete actions. `Denied` has precedence. Learning must be simulation-only by default. Leaving an allowed zone must be denied by default.

## Evidence and memory

Plan acceptance, approval requirements, decision rationales, and completion must be recorded through the normal Nexus event/replay path. Memory policy must be explicit by category. Operator memory must default to disabled. Compute placement must default private work to local execution and cloud execution to disabled.

## Non-goals of 1.0

NIL 1.0 does not define a general AI planner, remote-control protocol, identity system, persistent memory store, cloud inference integration, or physical-robot certification. These require separate authenticated implementations and validation evidence.
