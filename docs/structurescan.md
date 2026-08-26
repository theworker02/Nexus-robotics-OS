# StructureScan and DoorScan

StructureScan builds a versioned representation of observable surfaces, rooms, openings, and doors. DoorScan captures the observable state, geometry, hinge-side estimate, handle information, and a probabilistic material estimate. StructureDiff reports a later observable change without silently applying it to a canonical digital twin.

```powershell
cargo run -p nexus-cli -- structure scan
cargo run -p nexus-cli -- structure diff
```

The current doorway model is a deterministic simulator fixture. It does not infer hidden occupants or perform through-wall surveillance.
