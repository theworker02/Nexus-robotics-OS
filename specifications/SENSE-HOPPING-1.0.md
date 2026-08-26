# SenseHopping 1.0

SenseHopping routes semantic information requirements to healthy available `SenseProvider` records. Providers state information types, accuracy, latency, range, confidence, cost, availability, and health. A `SensePlan` records primary, secondary, and fused providers with the selection rationale.

The router may substitute or fuse providers after confidence degradation, health changes, or relevant environmental conditions. Every selection and fallback must be included in replay evidence. Skills request information such as `obstacle_distance` or `door_geometry`, not a vendor-specific sensor name.
