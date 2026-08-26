# Automation and routines

NIL defines typed domain models for automation rules and routines so scheduled and event-triggered behavior can use the same policy boundary as interactive goals. An automation rule has a class, trigger, conditions, actions, and an enabled state. A routine is an ordered sequence with a declared policy and schedule.

In v3.0 these are in-memory domain contracts, not a scheduler or Behavior Studio UI. No schedule is persisted, activated automatically, or dispatched to a robot. Future execution must compile actions into ordinary `GoalPlan` records and require any approval that the operating envelope demands.

That design prevents a routine from becoming an unreviewed bypass around Safety Governor, adapter checks, validation lifecycle, or operator authorization.
