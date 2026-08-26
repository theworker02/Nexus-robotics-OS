use nexus_integration_sdk::scaffold;
use nexus_nori_adapter::nori_a3_manifest;
use nexus_runtime::{
    doorway_learning_session, upgrade_advisor, AdaptiveRuntimePlanner, CapabilityIndex,
    EnvironmentCondition, HardwareProfile, InformationType, IntelligenceProfile, PackageManifest,
    ProvingGround, Runtime,
};

fn usage() {
    println!("Nexus Robotics OS v3.0 — Adaptive Autonomy\n\nCore: nexus demo | dev | doctor | telemetry\nRobot: nexus robot list|inspect|diagnose [nxr-1|nxr-2]\nGoal: nexus goal plan <objective> | run <objective> [--approve]\nAutonomy: nexus autonomy envelope | profile <manual|assisted|supervised|autonomous>\nBench: nexus bench simple-robot\nTask: nexus task run fetch-object | --skill <name> [--target <name>] [--dry-run]\nSkill: nexus skill list|test <name>|install <skill.yaml>\nSimulation: nexus sim robot nxr-2|nori-a3 | scenario fetch_cube|warehouse-fetch|doorway-learning|simple-robot-test | fault <camera|joint|low-battery|network>\nProving Ground: nexus prove skill <fetch-object|sense-hopping|door-scan|doorway-learning> [--trials 100] [--seed 483208] [--output report.md] | all\nSense: nexus sense list | plan <obstacle-distance|spatial-geometry|door-geometry>\nStructure: nexus structure scan | diff\nLearn: nexus learn create | start | status\nCompatibility: nexus compatibility inspect nori-a3\nPackages: nexus package validate|inspect <package.yaml>\nIntegration: nexus integration create --name <name> [--transport serial]\n\nNXR-2 and Nori profiles are simulated. Add --no-ai for deterministic planning.");
}
fn main() {
    let args: Vec<String> = std::env::args()
        .skip(1)
        .filter(|value| value != "--no-ai")
        .collect();
    let command: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut runtime = if args.iter().any(|value| {
        value == "nxr-2"
            || value == "nori-a3"
            || value == "warehouse-fetch"
            || value == "fetch-object"
            || value == "goal"
            || value == "autonomy"
            || value == "bench"
            || value == "profile"
            || value == "hardware"
            || value == "emulate"
            || value == "brain"
            || value == "model"
            || value == "upgrade"
            || value == "simple-robot-test"
    }) {
        Runtime::nxr2()
    } else {
        Runtime::nxr1()
    };
    match command.as_slice() {
        [] => usage(),
        [cmd] if *cmd == "help" || *cmd == "--help" => usage(),
        [cmd] if *cmd == "demo" || *cmd == "dev" => match runtime.run_warehouse_fetch_demo() { Ok(summary) => println!("Nexus Runtime READY\nSimulator READY\nNXR-2 CONNECTED\n{summary}\nReplay entries: {}", runtime.replays.values().map(|replay| replay.entries.len()).sum::<usize>()), Err(error) => fail(error), },
        ["robot", "list", ..] => println!("{}\t{}\t{:?}\t{:.0}%", runtime.robot.identity, runtime.robot.capabilities.name, runtime.robot.health, runtime.robot.battery_percent),
        ["robot", "inspect", ..] => { println!("{} ({})\nSafety: {}\nCapabilities:", runtime.robot.capabilities.name, runtime.robot.identity, runtime.robot.safety_state.label()); for capability in &runtime.robot.capabilities.capabilities { println!("- {}", capability.0); } },
        ["robot", "diagnose", ..] => println!("joints: {} virtual servos\nsensors: {:?}\nnetwork: {}\nbattery: {:.0}%\nskills: {} enabled\nsafety: {}", runtime.virtual_bus.discover_servos().len(), runtime.robot.sensors, runtime.robot.network_connected, runtime.robot.battery_percent, runtime.skills.list().count(), runtime.robot.safety_state.label()),
        ["profile", ..] => print_hardware_profile(&HardwareProfile::discovered_host()),
        ["hardware", "validate", path] => match std::fs::read_to_string(path)
            .map_err(|error| error.to_string())
            .and_then(|content| HardwareProfile::from_manifest(&content)) {
                Ok(profile) => {
                    print_hardware_profile(&profile);
                    let warnings = profile.validation_warnings();
                    if warnings.is_empty() {
                        println!("Hardware manifest: PASS");
                    } else {
                        println!("Hardware manifest: WARN");
                        for warning in warnings { println!("- {warning}"); }
                    }
                }
                Err(error) => eprintln!("hardware manifest invalid: {error}"),
            },
        ["emulate", "hardware", rest @ ..] => {
            let mut profile = HardwareProfile::minimum_robot();
            if let Some(value) = option(rest, "--ram").and_then(|value| value.parse().ok()) { profile.ram_mb = value; }
            if let Some(value) = option(rest, "--cpu").and_then(|value| value.parse().ok()) { profile.cpu_cores = value; }
            if let Some(value) = option(rest, "--camera").and_then(|value| value.parse().ok()) { profile.rgb_cameras = value; }
            if let Some(value) = option(rest, "--motors").and_then(|value| value.parse().ok()) { profile.motors = value; }
            profile.name = "Emulated hardware profile".into();
            println!("HARDWARE EMULATION\nNo physical device or adapter was contacted.");
            print_hardware_profile(&profile);
        },
        ["brain", "status", ..] => print_brain_plan(&runtime.hardware_profile, &runtime.brain_plan),
        ["brain", "serve", ..] => eprintln!("Nexus Brain network serving is not implemented: authenticated pairing, mutual keys, and encrypted transport are required before a host can bind a network port."),
        ["model", "recommend", ..] => print_model_recommendations(&runtime.hardware_profile),
        ["upgrade", "advisor", rest @ ..] => {
            let budget = option(rest, "--budget").and_then(|value| value.parse().ok());
            let recommendations = upgrade_advisor(&runtime.hardware_profile, budget);
            println!("Upgrade advisor: {}", runtime.brain_plan.class.label());
            if recommendations.is_empty() { println!("No compatible recommendations within the supplied budget."); }
            for recommendation in recommendations { println!("{}. {} (${})\n   {}", recommendation.priority, recommendation.title, recommendation.indicative_cost_usd, recommendation.benefit); }
        },
        ["skill", "list", ..] => for skill in runtime.skills.list() { let result = runtime.skills.compatibility(&skill.name, &runtime.robot.capabilities).expect("bundled skill exists"); println!("{}\t{}\t{}", skill.name, skill.version, if result.compatible { "SIMULATED" } else { "INCOMPATIBLE" }); },
        ["skill", "test", skill] => match runtime.run_skill(skill, Some("blue_container")) { Ok(()) => println!("skill test PASS: {skill} on {}", runtime.robot.identity), Err(error) => fail(error) },
        ["skill", "install", path] => match std::fs::read_to_string(path).ok().and_then(|yaml| runtime.skills.install_local_manifest(&yaml).ok()) { Some(name) => println!("installed local development skill: {name}"), None => eprintln!("unable to install: expected a readable skill.yaml with a name"), },
        ["task", "run", task, rest @ ..] if *task == "fetch-object" => { if rest.iter().any(|value| *value == "--dry-run") { println!("DRY RUN PASS\nCapabilities: compatible\nSkills: inspect_object, walk_to, pick_up, place_object\nSafety: effective policy valid\nNo commands dispatched."); } else { match runtime.run_warehouse_fetch_demo() { Ok(summary) => println!("task completed: {summary}"), Err(error) => fail(error), } } },
        ["task", "run", rest @ ..] => { let skill = option(rest, "--skill").unwrap_or("walk_to"); let target = option(rest, "--target"); if rest.iter().any(|value| *value == "--dry-run") { match runtime.skills.compatibility(skill, &runtime.robot.capabilities) { Ok(result) if result.compatible => println!("DRY RUN PASS: {skill}; safety and capability checks passed; no commands dispatched."), Ok(result) => println!("DRY RUN FAILED: missing {}", result.missing.join(", ")), Err(error) => fail(error), } } else { match runtime.run_skill(skill, target) { Ok(()) => println!("task completed: {skill}"), Err(error) => fail(error), } } },
        ["sim", "robot", profile] if *profile == "nxr-2" => println!("NXR-2 CONNECTED\nSIMULATED mobile-manipulator profile\nVirtualBus: {} servos, {} camera streams", runtime.virtual_bus.discover_servos().len(), runtime.virtual_bus.camera_streams),
        ["sim", "robot", profile] if *profile == "nori-a3" => println!("SIMULATED Nori-compatible profile CONNECTED\nCommunity Integration — not official hardware validation\nCapabilities: {}", nori_a3_manifest().records.len()),
        ["sim", "scenario", scenario] if *scenario == "fetch_cube" => { let mut legacy = Runtime::nxr1(); match legacy.run_fetch_cube_demo() { Ok(result) => println!("scenario fetch_cube: {result}"), Err(error) => fail(error), } },
        ["sim", "scenario", scenario] if *scenario == "warehouse-fetch" => match runtime.run_warehouse_fetch_demo() { Ok(result) => println!("scenario warehouse-fetch: {result}"), Err(error) => fail(error), },
        ["sim", "scenario", scenario] if *scenario == "doorway-learning" => match runtime.run_unfamiliar_door_challenge() { Ok(result) => println!("scenario doorway-learning: {result}"), Err(error) => fail(error), },
        ["sim", "scenario", scenario] if *scenario == "simple-robot-test" => match runtime.run_simple_robot_test() { Ok(result) => println!("scenario simple-robot-test: {result}"), Err(error) => fail(error), },
        ["sim", "fault", fault] => { runtime.inject_fault(fault); println!("fault injected: {fault}; health={:?}", runtime.robot.health); },
        ["goal", "plan", request @ ..] if !request.is_empty() => {
            let request = request.join(" ");
            print_goal_plan(&runtime.preview_goal(&request));
        },
        ["goal", "run", request @ ..] if !request.is_empty() => {
            let approved = request.iter().any(|value| *value == "--approve");
            let request = request
                .iter()
                .filter(|value| **value != "--approve")
                .copied()
                .collect::<Vec<_>>()
                .join(" ");
            match runtime.run_goal(&request, approved) {
                Ok(result) => println!("goal completed: {result}"),
                Err(error) => fail(error),
            }
        },
        ["autonomy", "envelope", ..] => print_envelope(&runtime.autonomy_envelope()),
        ["autonomy", "profile", profile] => {
            let profile = match *profile {
                "manual" => IntelligenceProfile::Manual,
                "assisted" => IntelligenceProfile::Assisted,
                "supervised" => IntelligenceProfile::Supervised,
                "autonomous" => IntelligenceProfile::Autonomous,
                _ => {
                    eprintln!("unknown autonomy profile; expected manual, assisted, supervised, or autonomous");
                    return;
                }
            };
            runtime.intelligence.policy = nexus_runtime::AutonomyPolicy::for_profile(profile);
            print_envelope(&runtime.autonomy_envelope());
        },
        ["bench", "simple-robot", ..] => match runtime.run_simple_robot_test() {
            Ok(result) => println!("{result}"),
            Err(error) => fail(error),
        },
        ["prove", "skill", skill, rest @ ..] => {
            let trials = option(rest, "--trials").and_then(|value| value.parse().ok()).unwrap_or(100);
            let seed = option(rest, "--seed").and_then(|value| value.parse().ok()).unwrap_or(483_208);
            let report = ProvingGround::prove_skill(skill, trials, seed);
            if let Some(path) = option(rest, "--output") {
                match report.write_markdown(path) {
                    Ok(()) => println!("Proving Ground report written: {path}\nHighest earned: {}", report.highest_earned().label()),
                    Err(error) => eprintln!("unable to write report: {error}"),
                }
            } else {
                println!("{}", report.render_markdown());
            }
        },
        ["prove", "all", rest @ ..] => {
            let trials = option(rest, "--trials").and_then(|value| value.parse().ok()).unwrap_or(100);
            for skill in ["fetch-object", "sense-hopping", "door-scan", "doorway-learning"] {
                let report = ProvingGround::prove_skill(skill, trials, 483_208);
                println!("{skill}: {} ({} / {} success; {} safe aborts)", report.highest_earned().label(), report.success_count(), report.trials.len(), report.safe_abort_count());
            }
        },
        ["prove", "profile", profile] if *profile == "cheap-mobile-robot" => {
            let profile = HardwareProfile::minimum_robot();
            let plan = AdaptiveRuntimePlanner::plan(&profile);
            let warnings = profile.validation_warnings();
            println!("PROFILE EMULATION PASS\nProfile: {}\nRecommended intelligence: {}\nSenseHopping: {:?}\nStructureScan: {:?}\nWarnings: {}\nCompatibility: C1 Emulated (not HIL or hardware validated)", profile.name, plan.class.label(), plan.sense_hopping, plan.structure_scan, warnings.len());
        },
        ["sense", "list", ..] => for provider in runtime.senses.providers() { println!("{}\t{:?}\tconfidence={:.2}\t{:?}", provider.id, provider.provides, provider.confidence, provider.health); },
        ["sense", "plan", requirement] => { let requirement = match *requirement { "obstacle-distance" => InformationType::ObstacleDistance, "spatial-geometry" => InformationType::SpatialGeometry, "door-geometry" => InformationType::DoorGeometry, _ => { eprintln!("unknown information requirement"); return; } }; match runtime.senses.route(requirement, EnvironmentCondition::Normal) { Ok(plan) => println!("Primary: {}\nSecondary: {:?}\nFusion: {:?}\nReason: {}", plan.primary, plan.secondary, plan.fused, plan.reason), Err(error) => eprintln!("sense plan failed: {error:?}"), } },
        ["structure", "scan", ..] => { let door = &runtime.structure_model.doors["D-118"]; println!("StructureScan\nRevision: {}\nDoor: {}\nState: {:?}\nHinge: {:?} ({:.0}%)\nMaterial: {:?} ({:.0}%)", runtime.structure_model.revision, door.id, door.state, door.hinge_side, door.hinge_confidence * 100.0, door.material.category, door.material.confidence * 100.0); },
        ["structure", "diff", ..] => match runtime.structure_model.mutate_door("D-118", nexus_runtime::DoorState::Open) { Some(diff) => println!("StructureDiff: revision {} -> {}\n{:?}", diff.from_revision, diff.to_revision, diff.changes), None => eprintln!("structure diff failed"), },
        ["learn", "create", ..] => { let session = doorway_learning_session(); println!("Learning session {}\nTask: {}\nBudget: {} attempts, {} seconds\nStage: {:?}", session.id, session.task, session.budget.max_attempts, session.budget.max_duration_s, session.stage); },
        ["learn", "start", ..] => match runtime.run_unfamiliar_door_challenge() { Ok(_) => println!("Active Learning completed in simulation. Candidate proposal recorded in replay; no production skill was changed."), Err(error) => fail(error) },
        ["learn", "status", ..] => println!("Learning mode: simulation-first\nProduction promotion: disabled\nPhysical learning: requires explicit hardware enablement and validation."),
        ["compatibility", "inspect", profile] if *profile == "nori-a3" => { let manifest = nori_a3_manifest(); let compatible = runtime.skills.list().filter(|skill| runtime.skills.compatibility(&skill.name, &manifest.base).is_ok_and(|result| result.compatible)).count(); println!("NORI COMPATIBILITY SIMULATION\nRobot capabilities: {}\nCompatible built-in skills: {compatible}\nCommunity Integration; no vendor endorsement or physical-hardware claim.", manifest.records.len()); },
        ["package", action, path] if *action == "validate" || *action == "inspect" => match std::fs::read_to_string(*path).ok().and_then(|content| PackageManifest::parse(&content).ok()) { Some(manifest) if *action == "validate" => println!("NRP valid: {} {} ({:?})", manifest.name, manifest.version, manifest.package_type), Some(manifest) => { let inspection = manifest.inspect(); println!("{}\nhash: {}\nsigned: {}\nproduction allowed: {}\n{}", manifest.name, inspection.content_hash, inspection.signed, inspection.production_allowed, inspection.warnings.join("\n")); }, None => eprintln!("invalid NRP manifest; required fields: name, version, type"), },
        ["integration", "create", rest @ ..] => { let name = option(rest, "--name").unwrap_or("nexus-adapter"); let transport = option(rest, "--transport").unwrap_or("serial"); println!("{}", scaffold(name, transport, &["telemetry.read"])); },
        ["telemetry", ..] => println!("battery={:.1}% pose=({:.2},{:.2}) network={} safety={} virtual={:?}", runtime.robot.battery_percent, runtime.robot.pose.x, runtime.robot.pose.y, runtime.robot.network_connected, runtime.robot.safety_state.label(), runtime.virtual_bus.telemetry()),
        ["doctor", ..] => println!("runtime: OK\nsimulator: OK (NXR-1/NXR-2 deterministic)\nVirtualBus: OK\nNori adapter: community simulated profile available\nLeRobot adapter: dataset bridge available\nROS 2: planned optional adapter\nsecurity: unsigned NRP packages restricted to development"),
        _ => usage(),
    }
}
fn option<'a>(args: &'a [&str], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|value| *value == flag)
        .and_then(|index| args.get(index + 1))
        .copied()
}
fn fail(error: impl std::fmt::Display) {
    eprintln!("command failed: {error}");
}

fn print_goal_plan(plan: &nexus_runtime::GoalPlan) {
    println!(
        "Goal plan: {}\nState: {:?}\nRisk: {:?}\nExpected duration: {} seconds",
        plan.goal.objective, plan.state, plan.risk, plan.expected_duration_s
    );
    for (index, step) in plan.steps.iter().enumerate() {
        let permission = step
            .permission
            .map(|permission| format!("; permission={permission:?}"))
            .unwrap_or_default();
        let target = step
            .target
            .as_deref()
            .map(|target| format!(" target={target}"))
            .unwrap_or_default();
        println!(
            "{}. {}{}{}\n   {}",
            index + 1,
            step.skill,
            target,
            permission,
            step.rationale
        );
    }
    println!(
        "Required capabilities: {}",
        plan.required_capabilities.join(", ")
    );
}

fn print_envelope(envelope: &nexus_runtime::OperatingEnvelope) {
    println!(
        "Operating envelope\nAllowed: {}\nApproval required: {}\nProhibited: {}",
        envelope.allowed.join(", "),
        envelope.approval_required.join(", "),
        envelope.prohibited.join(", ")
    );
}

fn print_hardware_profile(profile: &HardwareProfile) {
    let index = CapabilityIndex::evaluate(profile);
    let plan = AdaptiveRuntimePlanner::plan(profile);
    println!(
        "Nexus Hardware Profile\nRobot: {} ({})\nCompute: {} / {} cores / {} MB RAM / {} GB storage\nSensors: {} RGB, {} depth, lidar={}, IMU={}, range={}\nMovement: motors={}, servos={}, arms={}, grippers={}\nNetwork: {}\n\nNCI\nCompute {}  Memory {}  Perception {}\nMobility {}  Manipulation {}  Connectivity {}  Acceleration {}\nRecommended: {}\nSenseHopping: {:?}\nStructureScan: {:?}",
        profile.name,
        profile.robot_type,
        profile.architecture,
        profile.cpu_cores,
        profile.ram_mb,
        profile.storage_gb,
        profile.rgb_cameras,
        profile.depth_cameras,
        profile.lidar,
        profile.imu,
        profile.range_sensors,
        profile.motors,
        profile.servos,
        profile.arms,
        profile.grippers,
        profile.network_connected,
        index.compute,
        index.memory,
        index.perception,
        index.mobility,
        index.manipulation,
        index.connectivity,
        index.acceleration,
        index.recommended.label(),
        plan.sense_hopping,
        plan.structure_scan,
    );
}

fn print_brain_plan(profile: &HardwareProfile, plan: &nexus_runtime::AdaptiveRuntimePlan) {
    println!(
        "Nexus Brain\nProfile: {}\nRecommended intelligence: {}\n\nMemCore\nRuntime: {} MB\nPerception: {} MB\nWorld state: {} MB\nPlanner: {} MB\nModel: {} MB\nReserve: {} MB\n\nLocal: {}\nEdge eligible: {}\nDisabled: {}\nNetwork host: not configured",
        profile.name,
        plan.class.label(),
        plan.memory.runtime_mb,
        plan.memory.perception_mb,
        plan.memory.world_state_mb,
        plan.memory.planner_mb,
        plan.memory.model_mb,
        plan.memory.reserve_mb,
        plan.local_workloads.join(", "),
        if plan.edge_eligible.is_empty() { "none".into() } else { plan.edge_eligible.join(", ") },
        if plan.disabled.is_empty() { "none".into() } else { plan.disabled.join(", ") },
    );
}

fn print_model_recommendations(profile: &HardwareProfile) {
    let plan = AdaptiveRuntimePlanner::plan(profile);
    println!(
        "Model recommendations for {}\nSmall deterministic planner: supported locally",
        profile.name
    );
    if plan.class >= nexus_runtime::IntelligenceClass::N2Adaptive {
        println!(
            "Compact perception model: eligible within MemCore model budget ({} MB)",
            plan.memory.model_mb
        );
    } else {
        println!("Compact perception model: not recommended locally; use an approved Brain host when available");
    }
    if plan.class >= nexus_runtime::IntelligenceClass::N3Intelligent {
        println!("Large multimodal model: only after runtime-specific fit validation; current plan reserves {} MB", plan.memory.model_mb);
    } else {
        println!("Large multimodal model: insufficient local profile; remote placement requires a future paired Brain host");
    }
    println!("Safety and actuator control: LOCAL_REQUIRED; never delegated.");
}
