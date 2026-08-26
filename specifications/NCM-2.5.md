# NCM 2.5 — Semantic capability resources

NCM 2.5 represents a capability as a versioned semantic resource with properties, quality metadata, and provenance. Skills may declare required or optional capabilities, alternatives, and numerical constraints.

```yaml
capability:
  id: manipulation.arm.right
  version: 1
  available: true
properties:
  dof: 7
  payload_kg: 1.5
  gripper: parallel
quality: {}
source:
  type: adapter
  integration: nori-community
```

An integration must not infer unsupported properties. A constraint is satisfied only when the capability is available and every required property or quality threshold is met.
