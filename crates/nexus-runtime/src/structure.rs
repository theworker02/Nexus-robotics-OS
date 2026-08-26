//! StructureScan models visible and instrument-accessible building structure only.
use nexus_core::Pose;
use std::collections::BTreeMap;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StructureKind {
    Wall,
    Door,
    Window,
    Floor,
    Ceiling,
    Column,
    Panel,
    Cabinet,
    Opening,
    UnknownSurface,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MaterialCategory {
    WoodLike,
    Glass,
    Metal,
    GypsumDrywallLike,
    ConcreteMasonryLike,
    Plastic,
    Composite,
    Unknown,
}
#[derive(Clone, Debug)]
pub struct MaterialEstimate {
    pub category: MaterialCategory,
    pub confidence: f32,
    pub evidence: Vec<String>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoorState {
    Open,
    Closed,
    Obstructed,
    Unknown,
}
#[derive(Clone, Debug)]
pub struct DoorModel {
    pub id: String,
    pub state: DoorState,
    pub width_m: f32,
    pub height_m: f32,
    pub hinge_side: Option<String>,
    pub hinge_confidence: f32,
    pub handle_type: Option<String>,
    pub handle_position: Option<Pose>,
    pub material: MaterialEstimate,
}
#[derive(Clone, Debug)]
pub struct StructureSurface {
    pub id: String,
    pub kind: StructureKind,
    pub pose: Pose,
    pub dimensions_m: (f32, f32),
    pub confidence: f32,
    pub material: MaterialEstimate,
    pub last_seen_ms: u128,
}
#[derive(Clone, Debug, Default)]
pub struct StructureModel {
    pub revision: u64,
    pub surfaces: BTreeMap<String, StructureSurface>,
    pub doors: BTreeMap<String, DoorModel>,
    pub rooms: BTreeMap<String, Vec<String>>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StructureChange {
    DoorState {
        id: String,
        previous: DoorState,
        current: DoorState,
    },
    Added(String),
    Removed(String),
}
#[derive(Clone, Debug)]
pub struct StructureDiff {
    pub from_revision: u64,
    pub to_revision: u64,
    pub changes: Vec<StructureChange>,
}
impl StructureModel {
    pub fn doorway_lab() -> Self {
        let material = MaterialEstimate {
            category: MaterialCategory::GypsumDrywallLike,
            confidence: 0.82,
            evidence: vec!["RGB texture".into(), "depth plane".into()],
        };
        let door = DoorModel {
            id: "D-118".into(),
            state: DoorState::Closed,
            width_m: 0.91,
            height_m: 2.03,
            hinge_side: Some("left".into()),
            hinge_confidence: 0.94,
            handle_type: Some("lever".into()),
            handle_position: Some(Pose {
                x: 0.82,
                y: 1.01,
                yaw: 0.0,
            }),
            material: MaterialEstimate {
                category: MaterialCategory::WoodLike,
                confidence: 0.71,
                evidence: vec!["RGB color".into(), "lidar geometry".into()],
            },
        };
        Self {
            revision: 18,
            surfaces: [(
                "S-204".into(),
                StructureSurface {
                    id: "S-204".into(),
                    kind: StructureKind::Wall,
                    pose: Pose {
                        x: 2.0,
                        y: 0.0,
                        yaw: 1.57,
                    },
                    dimensions_m: (3.1, 2.4),
                    confidence: 0.88,
                    material,
                    last_seen_ms: 0,
                },
            )]
            .into_iter()
            .collect(),
            doors: [(door.id.clone(), door)].into_iter().collect(),
            rooms: [("Room A".into(), vec!["S-204".into(), "D-118".into()])]
                .into_iter()
                .collect(),
        }
    }
    pub fn mutate_door(&mut self, id: &str, state: DoorState) -> Option<StructureDiff> {
        let door = self.doors.get_mut(id)?;
        let previous = door.state;
        door.state = state;
        self.revision += 1;
        Some(StructureDiff {
            from_revision: self.revision - 1,
            to_revision: self.revision,
            changes: vec![StructureChange::DoorState {
                id: id.into(),
                previous,
                current: state,
            }],
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn door_scan_model_is_versioned_and_diffable() {
        let mut model = StructureModel::doorway_lab();
        let diff = model.mutate_door("D-118", DoorState::Open).unwrap();
        assert_eq!(diff.changes.len(), 1);
        assert_eq!(model.revision, 19);
    }
}
