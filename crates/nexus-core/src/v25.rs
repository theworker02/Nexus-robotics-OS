//! NCM 2.5 semantic resources and compatibility constraints.
use super::v2::CapabilityProvenance;
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum CapabilityValue {
    Bool(bool),
    Number(f64),
    Text(String),
}
#[derive(Clone, Debug, PartialEq)]
pub struct SemanticCapability {
    pub id: String,
    pub version: u16,
    pub available: bool,
    pub properties: BTreeMap<String, CapabilityValue>,
    pub quality: BTreeMap<String, f64>,
    pub provenance: CapabilityProvenance,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CapabilityConstraint {
    pub minimum_numbers: BTreeMap<String, f64>,
    pub required_properties: BTreeMap<String, CapabilityValue>,
    pub minimum_quality: BTreeMap<String, f64>,
}
impl CapabilityConstraint {
    pub fn satisfied_by(&self, capability: &SemanticCapability) -> Result<(), ConstraintFailure> {
        if !capability.available {
            return Err(ConstraintFailure::Unavailable(capability.id.clone()));
        }
        for (key, minimum) in &self.minimum_numbers {
            match capability.properties.get(key) {
                Some(CapabilityValue::Number(value)) if value >= minimum => {}
                _ => {
                    return Err(ConstraintFailure::Property {
                        capability: capability.id.clone(),
                        property: key.clone(),
                        required: *minimum,
                    })
                }
            }
        }
        for (key, expected) in &self.required_properties {
            if capability.properties.get(key) != Some(expected) {
                return Err(ConstraintFailure::RequiredProperty {
                    capability: capability.id.clone(),
                    property: key.clone(),
                });
            }
        }
        for (key, minimum) in &self.minimum_quality {
            match capability.quality.get(key) {
                Some(value) if value >= minimum => {}
                _ => {
                    return Err(ConstraintFailure::Quality {
                        capability: capability.id.clone(),
                        metric: key.clone(),
                        required: *minimum,
                    })
                }
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintFailure {
    Unavailable(String),
    Property {
        capability: String,
        property: String,
        required: f64,
    },
    RequiredProperty {
        capability: String,
        property: String,
    },
    Quality {
        capability: String,
        metric: String,
        required: f64,
    },
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::v2::{CapabilityProvenance, CapabilitySourceType};
    #[test]
    fn constraint_requires_capability_quality() {
        let capability = SemanticCapability {
            id: "vision.depth".into(),
            version: 1,
            available: true,
            properties: BTreeMap::new(),
            quality: [("max_range_m".into(), 8.0)].into_iter().collect(),
            provenance: CapabilityProvenance {
                source_type: CapabilitySourceType::Static,
                provider: "test".into(),
                observed_at_ms: None,
            },
        };
        let constraint = CapabilityConstraint {
            minimum_quality: [("max_range_m".into(), 5.0)].into_iter().collect(),
            ..Default::default()
        };
        assert!(constraint.satisfied_by(&capability).is_ok());
    }
}
