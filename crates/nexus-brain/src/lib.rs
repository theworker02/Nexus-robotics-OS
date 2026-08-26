//! Nexus Brain is the hardware-aware coordination layer for memory, planning,
//! learning placement, perception level, skills, automation, and interaction.
//! It performs no direct actuator control and defaults all private workloads to
//! local execution.

use nexus_core::{Capability, CapabilityManifest};
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntelligenceClass {
    N0Control,
    N1Lite,
    N2Adaptive,
    N3Intelligent,
    N4Advanced,
}

impl IntelligenceClass {
    pub fn label(self) -> &'static str {
        match self {
            Self::N0Control => "N0 Control",
            Self::N1Lite => "N1 Lite",
            Self::N2Adaptive => "N2 Adaptive",
            Self::N3Intelligent => "N3 Intelligent",
            Self::N4Advanced => "N4 Advanced",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FeatureLevel {
    Unavailable,
    Basic,
    Enhanced,
    Advanced,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComputePlacement {
    LocalRequired,
    LocalPreferred,
    RemoteAllowed,
    RemotePreferred,
    Disabled,
}

#[derive(Clone, Debug)]
pub struct HardwareProfile {
    pub name: String,
    pub robot_type: String,
    pub architecture: String,
    pub cpu_cores: u16,
    pub ram_mb: u32,
    pub storage_gb: u32,
    pub has_gpu: bool,
    pub has_npu: bool,
    pub network_connected: bool,
    pub battery_percent: Option<u8>,
    pub rgb_cameras: u8,
    pub depth_cameras: u8,
    pub lidar: bool,
    pub imu: bool,
    pub range_sensors: u8,
    pub motors: u8,
    pub servos: u8,
    pub arms: u8,
    pub grippers: u8,
    pub speakers: u8,
    pub screen: bool,
    pub lights: bool,
}

impl HardwareProfile {
    pub fn minimum_robot() -> Self {
        Self {
            name: "Nexus Minimum Robot".into(),
            robot_type: "mobile-robot".into(),
            architecture: "aarch64".into(),
            cpu_cores: 2,
            ram_mb: 1024,
            storage_gb: 16,
            has_gpu: false,
            has_npu: false,
            network_connected: true,
            battery_percent: Some(80),
            rgb_cameras: 1,
            depth_cameras: 0,
            lidar: false,
            imu: true,
            range_sensors: 3,
            motors: 2,
            servos: 0,
            arms: 0,
            grippers: 0,
            speakers: 0,
            screen: false,
            lights: true,
        }
    }

    pub fn nxr2() -> Self {
        Self {
            name: "NXR-2 Reference Simulator".into(),
            robot_type: "mobile-manipulator".into(),
            architecture: std::env::consts::ARCH.into(),
            cpu_cores: 8,
            ram_mb: 8192,
            storage_gb: 64,
            has_gpu: false,
            has_npu: false,
            network_connected: true,
            battery_percent: Some(100),
            rgb_cameras: 2,
            depth_cameras: 1,
            lidar: true,
            imu: true,
            range_sensors: 4,
            motors: 2,
            servos: 14,
            arms: 2,
            grippers: 2,
            speakers: 1,
            screen: true,
            lights: true,
        }
    }

    pub fn discovered_host() -> Self {
        let mut profile = Self::minimum_robot();
        profile.name = "Local Nexus Host (detected compute only)".into();
        profile.robot_type = "custom".into();
        profile.architecture = std::env::consts::ARCH.into();
        profile.cpu_cores = std::thread::available_parallelism()
            .map(|count| count.get().min(u16::MAX as usize) as u16)
            .unwrap_or(1);
        profile
    }

    pub fn validation_warnings(&self) -> Vec<String> {
        let mut warnings = vec![];
        if self.cpu_cores == 0 {
            warnings.push("CPU core count must be greater than zero".into());
        }
        if self.ram_mb < 256 {
            warnings.push("at least 256 MB RAM is required for a Nexus runtime profile".into());
        }
        if self.storage_gb < 1 {
            warnings.push("persistent storage must be configured".into());
        }
        if self.motors > 0 && self.range_sensors == 0 && !self.lidar && self.depth_cameras == 0 {
            warnings.push("mobile movement has no declared range, depth, or lidar sensing".into());
        }
        if self.arms > 0 && self.servos == 0 {
            warnings.push("arm declared without servo count or joint-limit mapping".into());
        }
        if self.grippers > 0 && self.arms == 0 {
            warnings.push("gripper declared without an arm mapping".into());
        }
        warnings
    }

    pub fn to_manifest(&self) -> String {
        format!(
            "[robot]\nname = \"{}\"\ntype = \"{}\"\n\n[compute]\narchitecture = \"{}\"\ncores = {}\nram_mb = {}\nstorage_gb = {}\ngpu = {}\nnpu = {}\n\n[vision]\nrgb_cameras = {}\ndepth_cameras = {}\n\n[sensing]\nimu = {}\nlidar = {}\nproximity = {}\n\n[mobility]\nmotors = {}\nservos = {}\n\n[manipulation]\narms = {}\ngrippers = {}\n\n[connectivity]\nnetwork = {}\n",
            self.name, self.robot_type, self.architecture, self.cpu_cores, self.ram_mb,
            self.storage_gb, self.has_gpu, self.has_npu, self.rgb_cameras, self.depth_cameras,
            self.imu, self.lidar, self.range_sensors, self.motors, self.servos, self.arms,
            self.grippers, self.network_connected
        )
    }

    pub fn from_manifest(input: &str) -> Result<Self, String> {
        let mut profile = Self::minimum_robot();
        let mut section = String::new();
        for raw in input.lines() {
            let line = raw.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                section = line[1..line.len() - 1].to_string();
                continue;
            }
            let Some((key, value)) = line.split_once('=') else {
                return Err(format!("invalid manifest line: {line}"));
            };
            let key = key.trim();
            let value = value.trim().trim_matches('"');
            match (section.as_str(), key) {
                ("robot", "name") => profile.name = value.into(),
                ("robot", "type") => profile.robot_type = value.into(),
                ("compute", "architecture") => profile.architecture = value.into(),
                ("compute", "cores") => profile.cpu_cores = parse(value, key)?,
                ("compute", "ram_mb") => profile.ram_mb = parse(value, key)?,
                ("compute", "storage_gb") => profile.storage_gb = parse(value, key)?,
                ("compute", "gpu") => profile.has_gpu = parse(value, key)?,
                ("compute", "npu") => profile.has_npu = parse(value, key)?,
                ("vision", "rgb_cameras") => profile.rgb_cameras = parse(value, key)?,
                ("vision", "depth_cameras") => profile.depth_cameras = parse(value, key)?,
                ("sensing", "imu") => profile.imu = parse(value, key)?,
                ("sensing", "lidar") => profile.lidar = parse(value, key)?,
                ("sensing", "proximity") => profile.range_sensors = parse(value, key)?,
                ("mobility", "motors") => profile.motors = parse(value, key)?,
                ("mobility", "servos") => profile.servos = parse(value, key)?,
                ("manipulation", "arms") => profile.arms = parse(value, key)?,
                ("manipulation", "grippers") => profile.grippers = parse(value, key)?,
                ("connectivity", "network") => profile.network_connected = parse(value, key)?,
                _ => {}
            }
        }
        if profile.name.is_empty() {
            return Err("[robot].name is required".into());
        }
        Ok(profile)
    }
}

fn parse<T: std::str::FromStr>(value: &str, key: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("invalid value for {key}: {value}"))
}

#[derive(Clone, Debug)]
pub struct CapabilityIndex {
    pub compute: u8,
    pub memory: u8,
    pub perception: u8,
    pub mobility: u8,
    pub manipulation: u8,
    pub connectivity: u8,
    pub acceleration: u8,
    pub recommended: IntelligenceClass,
}

impl CapabilityIndex {
    pub fn evaluate(profile: &HardwareProfile) -> Self {
        let compute = ((profile.cpu_cores as u32 * 10)
            + if profile.has_gpu || profile.has_npu {
                20
            } else {
                0
            })
        .min(100) as u8;
        let memory = (profile.ram_mb / 80).min(100) as u8;
        let perception = ((profile.rgb_cameras as u32 * 15)
            + (profile.depth_cameras as u32 * 25)
            + if profile.lidar { 30 } else { 0 }
            + if profile.imu { 10 } else { 0 }
            + profile.range_sensors as u32 * 5)
            .min(100) as u8;
        let mobility = ((profile.motors as u32 * 25) + (profile.servos as u32 * 3)).min(100) as u8;
        let manipulation = ((profile.arms as u32 * 35)
            + (profile.grippers as u32 * 25)
            + (profile.servos as u32 * 2))
            .min(100) as u8;
        let connectivity = if profile.network_connected { 80 } else { 10 };
        let acceleration = if profile.has_gpu && profile.has_npu {
            100
        } else if profile.has_gpu || profile.has_npu {
            70
        } else {
            0
        };
        let recommended = if profile.ram_mb < 512 || profile.cpu_cores < 2 {
            IntelligenceClass::N0Control
        } else if profile.ram_mb < 4096 || profile.cpu_cores < 4 {
            IntelligenceClass::N1Lite
        } else if profile.ram_mb < 8192 || profile.cpu_cores < 8 {
            IntelligenceClass::N2Adaptive
        } else if profile.ram_mb < 16384 && !profile.has_gpu && !profile.has_npu {
            IntelligenceClass::N3Intelligent
        } else {
            IntelligenceClass::N4Advanced
        };
        Self {
            compute,
            memory,
            perception,
            mobility,
            manipulation,
            connectivity,
            acceleration,
            recommended,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MemCorePlan {
    pub runtime_mb: u32,
    pub perception_mb: u32,
    pub world_state_mb: u32,
    pub planner_mb: u32,
    pub model_mb: u32,
    pub reserve_mb: u32,
}

impl MemCorePlan {
    pub fn allocate(profile: &HardwareProfile, class: IntelligenceClass) -> Self {
        let total = profile.ram_mb.max(256);
        let runtime_mb = (total / 10).clamp(96, 512);
        let perception_mb = match class {
            IntelligenceClass::N0Control => 0,
            IntelligenceClass::N1Lite => total / 8,
            _ => total / 6,
        };
        let world_state_mb = match class {
            IntelligenceClass::N0Control => 32,
            IntelligenceClass::N1Lite => total / 12,
            _ => total / 10,
        };
        let planner_mb = match class {
            IntelligenceClass::N0Control => 0,
            IntelligenceClass::N1Lite => total / 12,
            _ => total / 10,
        };
        let model_mb = match class {
            IntelligenceClass::N0Control | IntelligenceClass::N1Lite => 0,
            IntelligenceClass::N2Adaptive => total / 6,
            IntelligenceClass::N3Intelligent => total / 4,
            IntelligenceClass::N4Advanced => total / 3,
        };
        let allocated = runtime_mb + perception_mb + world_state_mb + planner_mb + model_mb;
        let reserve_mb = total.saturating_sub(allocated).max(total / 5);
        Self {
            runtime_mb,
            perception_mb,
            world_state_mb,
            planner_mb,
            model_mb,
            reserve_mb,
        }
    }

    pub fn pressure_response(&self, available_mb: u32) -> &'static str {
        if available_mb < self.reserve_mb / 2 {
            "unload optional model; preserve safety and control; use deterministic fallback"
        } else if available_mb < self.reserve_mb {
            "reduce planning context and compress old world state"
        } else {
            "normal"
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdaptiveRuntimePlan {
    pub class: IntelligenceClass,
    pub memory: MemCorePlan,
    pub local_workloads: Vec<String>,
    pub edge_eligible: Vec<String>,
    pub disabled: Vec<String>,
    pub sense_hopping: FeatureLevel,
    pub structure_scan: FeatureLevel,
    pub interaction: FeatureLevel,
}

pub struct AdaptiveRuntimePlanner;

impl AdaptiveRuntimePlanner {
    pub fn plan(profile: &HardwareProfile) -> AdaptiveRuntimePlan {
        let index = CapabilityIndex::evaluate(profile);
        let class = index.recommended;
        let sense_hopping = if profile.lidar && profile.depth_cameras > 0 {
            FeatureLevel::Advanced
        } else if profile.rgb_cameras > 0 && (profile.range_sensors > 0 || profile.imu) {
            FeatureLevel::Enhanced
        } else if profile.rgb_cameras > 0 || profile.range_sensors > 0 {
            FeatureLevel::Basic
        } else {
            FeatureLevel::Unavailable
        };
        let structure_scan = if profile.lidar && profile.depth_cameras > 0 {
            FeatureLevel::Advanced
        } else if profile.depth_cameras > 0 {
            FeatureLevel::Enhanced
        } else if profile.rgb_cameras > 0 {
            FeatureLevel::Basic
        } else {
            FeatureLevel::Unavailable
        };
        let interaction = if profile.screen && profile.speakers > 0 {
            FeatureLevel::Advanced
        } else if profile.screen || profile.speakers > 0 || profile.lights {
            FeatureLevel::Basic
        } else {
            FeatureLevel::Unavailable
        };
        let mut local_workloads = vec![
            "Safety and actuator control".into(),
            "Capability resolution".into(),
            "Telemetry".into(),
            "Deterministic skills".into(),
        ];
        if class >= IntelligenceClass::N1Lite {
            local_workloads.push("Basic planning and short-term memory".into());
        }
        if class >= IntelligenceClass::N2Adaptive {
            local_workloads.push("Local perception and StructureScan context".into());
        }
        if class >= IntelligenceClass::N3Intelligent {
            local_workloads.push("Local model runtime eligible".into());
        }
        let edge_eligible = if profile.network_connected {
            vec![
                "Large vision reasoning".into(),
                "Long-horizon planning".into(),
                "Learning analysis".into(),
            ]
        } else {
            vec![]
        };
        let mut disabled = vec![];
        if class < IntelligenceClass::N2Adaptive {
            disabled.push("Local model runtime".into());
        }
        if !profile.network_connected {
            disabled.push("Edge Brain workloads".into());
        }
        AdaptiveRuntimePlan {
            class,
            memory: MemCorePlan::allocate(profile, class),
            local_workloads,
            edge_eligible,
            disabled,
            sense_hopping,
            structure_scan,
            interaction,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UpgradeRecommendation {
    pub priority: u8,
    pub title: String,
    pub benefit: String,
    pub indicative_cost_usd: u32,
}

pub fn upgrade_advisor(
    profile: &HardwareProfile,
    budget_usd: Option<u32>,
) -> Vec<UpgradeRecommendation> {
    let mut recommendations = vec![];
    if profile.ram_mb < 8192 {
        recommendations.push(UpgradeRecommendation {
            priority: 1,
            title: "Increase RAM to 8 GB".into(),
            benefit: "Unlocks richer local memory and adaptive local perception eligibility."
                .into(),
            indicative_cost_usd: 40,
        });
    }
    if profile.depth_cameras == 0 {
        recommendations.push(UpgradeRecommendation {
            priority: 2,
            title: "Add a depth camera".into(),
            benefit: "Upgrades StructureScan and obstacle modelling when an adapter is available."
                .into(),
            indicative_cost_usd: 35,
        });
    }
    if !profile.lidar {
        recommendations.push(UpgradeRecommendation {
            priority: 3,
            title: "Add a lidar or range sensor array".into(),
            benefit: "Improves SenseHopping redundancy and navigation sensing.".into(),
            indicative_cost_usd: 45,
        });
    }
    recommendations
        .into_iter()
        .filter(|item| {
            budget_usd
                .map(|budget| item.indicative_cost_usd <= budget)
                .unwrap_or(true)
        })
        .collect()
}

pub fn capabilities_from_profile(profile: &HardwareProfile) -> BTreeSet<Capability> {
    let mut capabilities = BTreeSet::new();
    if profile.rgb_cameras > 0 {
        capabilities.insert(Capability::new("vision.rgb"));
    }
    if profile.depth_cameras > 0 {
        capabilities.insert(Capability::new("vision.depth"));
    }
    if profile.lidar {
        capabilities.insert(Capability::new("sensing.lidar"));
    }
    if profile.imu {
        capabilities.insert(Capability::new("sensing.imu"));
    }
    if profile.motors > 0 {
        capabilities.insert(Capability::new("locomotion.wheeled"));
    }
    if profile.grippers > 0 {
        capabilities.insert(Capability::new("manipulators.gripper"));
    }
    capabilities
}

pub fn manifest_from_profile(profile: &HardwareProfile) -> CapabilityManifest {
    CapabilityManifest {
        robot_id: "profiled-robot".into(),
        name: profile.name.clone(),
        architecture: profile.architecture.clone(),
        capabilities: capabilities_from_profile(profile),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimum_robot_is_recommended_as_lite_and_keeps_control_local() {
        let plan = AdaptiveRuntimePlanner::plan(&HardwareProfile::minimum_robot());
        assert_eq!(plan.class, IntelligenceClass::N1Lite);
        assert!(plan
            .local_workloads
            .iter()
            .any(|item| item.contains("Safety")));
        assert!(plan
            .disabled
            .iter()
            .any(|item| item.contains("Local model")));
    }

    #[test]
    fn nxr2_has_enhanced_or_better_perception_features() {
        let plan = AdaptiveRuntimePlanner::plan(&HardwareProfile::nxr2());
        assert_eq!(plan.structure_scan, FeatureLevel::Advanced);
        assert_eq!(plan.sense_hopping, FeatureLevel::Advanced);
    }

    #[test]
    fn manifest_round_trip_preserves_profile_inputs() {
        let profile = HardwareProfile::minimum_robot();
        let parsed = HardwareProfile::from_manifest(&profile.to_manifest()).unwrap();
        assert_eq!(parsed.ram_mb, 1024);
        assert_eq!(parsed.motors, 2);
        assert_eq!(parsed.name, "Nexus Minimum Robot");
    }
}
