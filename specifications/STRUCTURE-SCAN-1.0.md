# StructureScan 1.0

StructureScan represents visible and instrument-accessible structure: walls, doors, windows, floors, ceilings, columns, panels, cabinets, openings, and unknown surfaces. It must not be used for covert through-wall surveillance or person identification.

Every material or geometry estimate exposes evidence and uncertainty. A `StructureModel` is versioned; `StructureDiff` reports observable changes such as a door state transition, added structure, or removed structure. Persistent digital-twin changes require appropriate confidence and approval; scans do not silently rewrite a canonical twin.
