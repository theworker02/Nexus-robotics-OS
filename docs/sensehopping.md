# SenseHopping

SenseHopping lets a skill request information instead of a fixed sensor. The NXR-2 reference model currently offers RGB-D, lidar, and proximity providers. The router ranks healthy providers by confidence, accuracy, range, latency, power cost, and environment conditions; it records each choice and fallback.

```powershell
cargo run -p nexus-cli -- sense list
cargo run -p nexus-cli -- sense plan door-geometry
```

This is deterministic simulation logic. Sensor accuracy values are test data, not physical sensor certification.
