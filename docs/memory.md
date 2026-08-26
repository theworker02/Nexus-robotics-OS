# Memory and privacy

NIL's `ExperienceMemory` represents local robot knowledge as typed records: world, task, skill, failure, routine, and operator information. A record contains a subject, detail, confidence, timestamp, and retention policy.

The default policies persist world, task, skill, failure, and routine records for the current in-process runtime. Operator memory is disabled by default. This release has no disk persistence, synchronization service, or user identity store, so ending the process clears records despite their logical `Persistent` retention classification.

`ComputeRouter` is policy-only in v3.0. Private requests route to local compute; cloud placement is disabled by default. It does not invoke a cloud provider, transmit data, or retain credentials.

Before adding persistence or remote compute, Nexus must add explicit retention controls, export and delete operations, encryption at rest, authenticated transport, consent, and audit evidence. No personal or sensitive data should be placed in a goal request or memory record unless the operator has explicitly chosen an appropriate storage policy.
