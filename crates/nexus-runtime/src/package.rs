//! NRP (Nexus Robotics Package) metadata validation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageType {
    Skill,
    Adapter,
    RobotProfile,
    Simulator,
    Model,
    Integration,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageManifest {
    pub name: String,
    pub version: String,
    pub package_type: PackageType,
    pub nexus_compatibility: String,
    pub components: Vec<String>,
    pub permissions: Vec<String>,
    pub publisher: Option<String>,
    pub signature: Option<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackageInspection {
    pub content_hash: String,
    pub signed: bool,
    pub production_allowed: bool,
    pub warnings: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PackageError {
    MissingField(&'static str),
    UnsupportedType(String),
    IncompatibleNexus(String),
}
impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for PackageError {}
impl PackageManifest {
    /// Parses the compact NRP YAML subset used by the local CLI. Full YAML can be added behind this boundary.
    pub fn parse(yaml: &str) -> Result<Self, PackageError> {
        let value = |key: &str| {
            yaml.lines()
                .find_map(|line| line.trim().strip_prefix(&format!("{key}:")))
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
        };
        let name = value("name").ok_or(PackageError::MissingField("name"))?;
        let version = value("version").ok_or(PackageError::MissingField("version"))?;
        let raw_type = value("type").ok_or(PackageError::MissingField("type"))?;
        let package_type = match raw_type.as_str() {
            "skill" => PackageType::Skill,
            "adapter" => PackageType::Adapter,
            "robot-profile" => PackageType::RobotProfile,
            "simulator" => PackageType::Simulator,
            "model" => PackageType::Model,
            "integration" => PackageType::Integration,
            _ => return Err(PackageError::UnsupportedType(raw_type)),
        };
        let nexus_compatibility = value("nexus").unwrap_or_else(|| ">=2.0".into());
        if !nexus_compatibility.contains('2') {
            return Err(PackageError::IncompatibleNexus(nexus_compatibility));
        }
        Ok(Self {
            name,
            version,
            package_type,
            nexus_compatibility,
            components: yaml
                .lines()
                .filter_map(|line| line.trim().strip_prefix("- ").map(str::to_owned))
                .collect(),
            permissions: vec![],
            publisher: value("publisher"),
            signature: value("signature"),
        })
    }
    pub fn inspect(&self) -> PackageInspection {
        let material = format!(
            "{}:{}:{:?}:{}",
            self.name,
            self.version,
            self.package_type,
            self.components.join(",")
        );
        let hash = material.bytes().fold(0xcbf29ce484222325u64, |hash, byte| {
            (hash ^ byte as u64).wrapping_mul(0x100000001b3)
        });
        let signed = self.signature.is_some();
        PackageInspection {
            content_hash: format!("fnv1a:{hash:016x}"),
            signed,
            production_allowed: signed,
            warnings: if signed {
                vec![]
            } else {
                vec!["unsigned package: allowed for local development only; production mode must reject it".into()]
            },
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsigned_package_is_visible_not_silently_trusted() {
        let manifest =
            PackageManifest::parse("name: nori\nversion: 2.0.0\ntype: integration\nnexus: >=2.0\n")
                .unwrap();
        let inspected = manifest.inspect();
        assert!(!inspected.signed);
        assert!(!inspected.production_allowed);
    }
}
