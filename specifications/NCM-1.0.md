# NCM 1.0 — Nexus Capability Manifest

NCM describes robot capabilities with normalized dotted paths. A capability is a positive, machine-readable fact such as `vision.rgbd`, `locomotion.biped`, or `manipulators.right.gripper`. Skills declare required paths and Nexus produces either `COMPATIBLE` or a list of missing requirements.

## Minimal manifest

```yaml
robot:
  id: nxr-1
  name: NXR-1
  architecture: x86_64-sim
capabilities:
  - locomotion.biped
  - vision.rgbd
  - manipulators.right.gripper
```

NCM aliases are deliberately semantic: `vision.rgbd` satisfies both `vision.rgb` and `vision.depth`. Production transports should attach a robot public key, hardware identifier, capability hash, and signature to this document.
