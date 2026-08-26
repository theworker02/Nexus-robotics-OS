# Skill reliability 1.0

A production skill must declare lifecycle status, deterministic behavior, preconditions, expected postconditions, resource requirements, maximum runtime, safety requirements, recovery behavior, non-recoverable failures, and cancellation policy.

`CANCEL_IMMEDIATE`, `CANCEL_SAFE_POINT`, and `NOT_CANCELLABLE` are explicit contracts. Emergency stop overrides every cancellation policy. A failed postcondition is a failed skill execution.

Each skill publishes the factual validation level: unit, container, simulation, fault injection, HIL, or physical hardware.
