# Runtime recovery

Significant task events can be recorded in the local append-only durable event log. Recovery never auto-resumes physical motion: a persisted motion task requires operator review after robot-state reconciliation.
