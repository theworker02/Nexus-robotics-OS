# Requirements Document

## Introduction

Nexus Robotics OS Phase VI / v4.0 adds a provider-agnostic Model Fabric and Cognitive Runtime for model-assisted planning, evaluation, memory, learning advice, and skill composition. The feature extends the existing `nexus-core`, `nexus-runtime`, `nexus-brain`, NCM, NIL, Proving Ground, CLI, SDK, and documentation surfaces without replacing deterministic execution contracts.

The feature is local-first, hardware-aware, privacy-aware, auditable, and simulation-first. Models may propose structured plans, interpretations, critiques, and learning or skill changes. Nexus deterministic validation and the Safety Governor remain authoritative: no model may directly control an actuator, emit an executable actuator command, authorize an unsafe action, bypass capability or permission checks, weaken a vendor or Nexus limit, or resume motion after recovery without the existing operator and safety rules.

## Glossary

- **Model_Fabric**: The provider-agnostic Nexus subsystem that discovers, registers, invokes, routes, evaluates, and audits model workloads.
- **Cognitive_Runtime**: The Nexus subsystem that turns goals, observations, memory, and model outputs into validated structured plans and reviewable cognitive results.
- **Model_Provider**: A local or remote inference implementation exposed through a Model_Fabric provider adapter.
- **Provider_Adapter**: A contract implementation that translates a Model_Provider API into Model_Fabric requests, responses, health checks, limits, and evidence.
- **Model_Role**: A named cognitive purpose such as planner, critic, evaluator, perception interpreter, failure analyst, learning advisor, or skill composer.
- **Model_Registry**: The inventory of model metadata, roles, versions, capabilities, provenance, limits, privacy class, and availability.
- **Model_Manifest**: A signed or explicitly local-development-only declaration of a model’s identity, provider, roles, inputs, outputs, resource needs, and policy metadata.
- **Capability_Manifest**: The existing NCM-derived description of robot capabilities, constraints, provenance, health, and quality.
- **Placement_Policy**: A policy that selects local, edge, remote, deferred, or disabled execution based on hardware, connectivity, privacy, budget, latency, and safety constraints.
- **Model_Request**: A normalized, policy-checked invocation containing a Model_Role, task context, input references, output schema, budget, deadline, and privacy classification.
- **Structured_Plan**: A schema-validated proposal containing goal interpretation, ordered deterministic skill steps, required capabilities, expected observations, preconditions, failure handling, and confidence or uncertainty.
- **Deterministic_Validator**: Nexus code that validates schemas, capabilities, permissions, preconditions, operating envelopes, resource locks, timing, and other execution invariants without delegating authority to a model.
- **Safety_Governor**: The existing authoritative Nexus safety layer that approves, constrains, rejects, cancels, and monitors physical actions before Adapter_Dispatch.
- **RulePlanner**: A deterministic, model-independent planner that produces bounded reference plans from registered rules, skills, capabilities, and state.
- **Cognitive_Memory**: Policy-governed storage for approved observations, plans, outcomes, failures, evaluations, and operator-provided knowledge.
- **Retrieval**: Selection of relevant Cognitive_Memory records for a Model_Request or RulePlanner context.
- **ModelBench**: A repeatable benchmark suite measuring model and runtime quality against deterministic scenarios, schemas, latency, cost, privacy, and safety outcomes.
- **Model_Arena**: An evaluation mode that compares multiple eligible models or configurations on identical seeded tasks and reports winner criteria and uncertainty.
- **Cognitive_Door_Challenge**: A deterministic reference challenge in which the system interprets a door-related goal, gathers required evidence, proposes a plan, validates the plan, and reports safe success or a bounded failure without bypassing Safety_Governor controls.
- **Audit_Record**: An append-only evidence record linking request, policy decision, model/provider, inputs or references, output digest, validation results, execution outcome, and operator action.
- **Prompt_Injection**: Untrusted instruction content in a model input, retrieved record, tool result, document, or environment observation that attempts to alter authority, policy, safety, or data-access boundaries.
- **Nexus_Feature**: A Cargo feature or workspace-level opt-in that enables an integration or capability without making an unavailable provider or unsafe transport appear available.
- **NCM**: Nexus Capability Model, the existing capability contract for robot functions, constraints, quality, and provenance.
- **Nexus_Brain**: The existing hardware-aware planning layer that profiles compute and robot hardware, derives intelligence classes, and recommends feature and workload placement.
- **NIL**: The existing Nexus Intelligence Layer that turns approved goals into policy-governed, inspectable plans above skills and the Safety_Governor.
- **Proving_Ground**: The existing evidence framework that reports software, virtual hardware, physics, adversarial, HIL, and robot validation levels separately.
- **Studio**: A supported Nexus desktop or web operator surface for inspecting and controlling runtime workflows when implemented for the release.
- **Actuator**: A motor, servo, gripper, or other physical output device capable of changing the robot’s environment.
- **Adapter_Dispatch**: The existing runtime handoff from an authorized skill action to a hardware or simulator adapter.
- **Audit_Stream**: The append-only telemetry and evidence stream carrying Audit_Record events and correlation identifiers.
- **Security_Boundary**: The deterministic policy boundary that controls authority, data access, tools, secrets, permissions, and Safety_Governor interaction.
- **Evaluation_System**: The combined ModelBench, Model_Arena, deterministic scenario, replay, and Proving_Ground reporting surface.

## Requirements

### Requirement 1: Provider-Agnostic Fabric Contract

**User Story:** As a Nexus integrator, I want one model contract independent of inference vendors, so that model-assisted features remain portable across local runtimes and approved remote providers.

#### Acceptance Criteria

1. THE Model_Fabric SHALL expose a normalized Model_Request and normalized response contract that does not require callers to depend on a specific Model_Provider.
2. WHEN a Model_Request is submitted, THE Model_Fabric SHALL return a typed result containing the response payload or typed failure, provider identity, model identity, elapsed time, usage accounting, and policy outcome.
3. IF a Model_Provider does not support the requested Model_Role, input modality, output schema, or policy constraints, THEN THE Model_Fabric SHALL reject the request or select an eligible alternative without exposing unsupported capability as available.
4. THE Model_Fabric SHALL preserve the existing Nexus separation in which model inference produces advisory data and deterministic Nexus components retain execution authority.

### Requirement 2: Model Roles and Registry

**User Story:** As a platform operator, I want models registered by explicit roles and metadata, so that routing and evaluation are explainable and reproducible.

#### Acceptance Criteria

1. THE Model_Registry SHALL store each Model_Manifest with stable identity, version, provider, supported Model_Role values, input and output schemas, context limits, resource requirements, privacy class, cost metadata, provenance, validation status, and availability state.
2. WHEN a Model_Manifest is registered, THE Model_Registry SHALL validate required fields, unique identity and version, supported schema references, and provider compatibility before making the manifest eligible for routing.
3. WHEN a Model_Manifest is replaced by a newer version, THE Model_Registry SHALL retain the prior version and its Audit_Record references.
4. IF a Model_Manifest is unsigned outside local-development mode or has failed required validation, THEN THE Model_Registry SHALL mark the manifest ineligible for production routing and report the reason.
5. THE Model_Registry SHALL support at least planner, critic, evaluator, perception interpreter, failure analyst, learning advisor, and skill composer Model_Role values.

### Requirement 3: Provider Adapters

**User Story:** As an adapter author, I want a stable Provider_Adapter contract, so that local engines, approved edge hosts, and future providers can be integrated without changing runtime semantics.

#### Acceptance Criteria

1. THE Provider_Adapter SHALL implement provider identification, model discovery or manifest loading, request execution, health reporting, cancellation or deadline handling, usage reporting, and typed error mapping.
2. WHEN a Provider_Adapter returns a response, THE Model_Fabric SHALL normalize the response into the requested output schema or return a typed schema error.
3. IF a Provider_Adapter reports timeout, unavailable transport, malformed output, policy refusal, or resource exhaustion, THEN THE Model_Fabric SHALL record the typed failure and make the configured fallback decision.
4. THE Provider_Adapter contract SHALL prevent an adapter response from being dispatched as an actuator command without Structured_Plan validation and Safety_Governor authorization.
5. WHERE a Provider_Adapter is compiled behind a Nexus_Feature, THE build and runtime SHALL report the feature as unavailable when the feature is disabled rather than silently selecting a different provider.

### Requirement 4: Capability and Hardware-Aware Routing

**User Story:** As a robot integrator, I want routing to use confirmed hardware and capability data, so that cognitive workloads fit the robot and remain functional under degraded resources.

#### Acceptance Criteria

1. WHEN the Model_Fabric routes a Model_Request, THE Placement_Policy SHALL consider the current Capability_Manifest, Nexus Brain hardware profile, intelligence class, available memory, acceleration, battery, connectivity, latency budget, privacy class, and provider health.
2. WHEN local resources satisfy the request’s policy and resource constraints, THE Placement_Policy SHALL prefer local execution for private or safety-adjacent workloads.
3. IF a request requires a capability, modality, memory limit, accelerator, or latency bound unavailable at the selected placement, THEN THE Placement_Policy SHALL select another eligible placement or return a typed unavailable result.
4. WHILE connectivity is unavailable, THE Model_Fabric SHALL continue deterministic local operation and SHALL not classify remote-only model workloads as successful.
5. THE routing result SHALL identify selected placement, rejected alternatives, governing constraints, and the reason for selection in an Audit_Record.

### Requirement 5: Budgets, Privacy, and Fallbacks

**User Story:** As an operator, I want model use bounded by explicit budgets and privacy policies, so that cognition cannot consume uncontrolled resources or disclose protected data.

#### Acceptance Criteria

1. THE Model_Fabric SHALL enforce per-request and aggregate budgets for time, memory, tokens or equivalent provider usage, monetary cost when applicable, and retry count.
2. IF a Model_Request would exceed a configured budget, THEN THE Model_Fabric SHALL reject, truncate, defer, or route the request according to the configured policy and SHALL record the decision.
3. THE Placement_Policy SHALL classify inputs and retrieved records by privacy class and SHALL prevent a placement from receiving data above the placement’s allowed privacy level.
4. WHEN model inference is unavailable, over budget, disallowed by privacy policy, or returns invalid output, THE Cognitive_Runtime SHALL invoke the deterministic RulePlanner or a bounded safe failure path.
5. THE fallback path SHALL preserve capability checks, permission decisions, operating envelopes, resource locks, watchdogs, emergency-stop behavior, and vendor limits.
6. THE Model_Fabric SHALL not use remote execution as a fallback for a request explicitly classified as local-required or private-local.

### Requirement 6: Deterministic RulePlanner

**User Story:** As a safety-conscious operator, I want useful behavior without a model, so that the robot remains predictable when inference is absent or untrusted.

#### Acceptance Criteria

1. THE RulePlanner SHALL generate plans only from registered deterministic rules, current state, Capability_Manifest data, approved skills, and explicit goal inputs.
2. WHEN a Cognitive_Runtime request cannot obtain an eligible model result, THE RulePlanner SHALL produce a bounded plan or a typed explanation that no safe plan is available.
3. THE RulePlanner SHALL produce the same plan and evidence for identical versioned inputs, configuration, seed, and state snapshot.
4. THE RulePlanner SHALL never emit a physical command that bypasses the Deterministic_Validator or Safety_Governor.
5. IF deterministic rules cannot satisfy required capabilities, preconditions, or operating limits, THEN THE RulePlanner SHALL refuse the plan and identify the unmet condition.

### Requirement 7: Cognitive Runtime and Structured Plans

**User Story:** As a skill and application developer, I want model-assisted goals compiled into inspectable plans, so that model output can improve interpretation without becoming an execution authority.

#### Acceptance Criteria

1. WHEN a goal and approved context are submitted, THE Cognitive_Runtime SHALL produce a Structured_Plan or a typed refusal with reason, provenance, and fallback status.
2. THE Structured_Plan SHALL contain a normalized goal, ordered deterministic skill references, required capabilities, permissions, preconditions, expected observations, timeout or budget bounds, uncertainty, and failure handling.
3. WHEN a model returns free text, THE Cognitive_Runtime SHALL treat the text as untrusted input and SHALL require conversion into the Structured_Plan schema before validation.
4. THE Deterministic_Validator SHALL validate every Structured_Plan against current capabilities, skill contracts, permissions, state, operating envelope, resource locks, and configured budgets before execution.
5. IF validation fails or the world state changes such that a validated plan is stale, THEN THE Cognitive_Runtime SHALL invalidate the plan and require replanning or bounded recovery.
6. THE Safety_Governor SHALL authorize every physical action proposed by a Structured_Plan immediately before Adapter_Dispatch, and a model result SHALL not authorize, override, or weaken that decision.
7. THE Cognitive_Runtime SHALL not expose direct actuator APIs to Model_Provider, Provider_Adapter, or Model_Role implementations.

### Requirement 8: Critics, Evaluators, Failure Analysis, Learning Advice, and Skill Composition

**User Story:** As a runtime maintainer, I want specialized cognitive roles to improve plans and reliability, so that every improvement remains reviewable and constrained by Nexus contracts.

#### Acceptance Criteria

1. WHEN a Structured_Plan is generated, a Critic Model_Role SHALL be able to identify missing assumptions, unsupported steps, uncertainty, and policy concerns without changing execution authority.
2. WHEN a scenario or execution completes, an Evaluator Model_Role SHALL compare observed evidence with declared expectations and return typed findings with evidence references.
3. WHEN a skill or plan fails, a Failure_Analyst Model_Role SHALL classify failure causes and confidence without claiming facts absent from telemetry, replay, or approved observations.
4. WHEN sufficient evidence exists, a Learning_Advisor Model_Role SHALL produce bounded parameter or data recommendations that identify affected skills, expected benefit, evidence, risks, and required validation level.
5. WHEN a Skill_Composer Model_Role proposes a new or changed skill, THE Cognitive_Runtime SHALL require a package or contract representation, capability requirements, safety metadata, deterministic validation, and Proving Ground evidence before the skill becomes executable.
6. THE Cognitive_Runtime SHALL prevent critics, evaluators, failure analysts, learning advisors, and skill composers from directly modifying production safety limits, permissions, actuator commands, or validated skill contracts.
7. IF a cognitive recommendation cannot be supported by available evidence or fails validation, THEN THE Cognitive_Runtime SHALL mark the recommendation rejected or advisory-only and SHALL preserve the prior executable behavior.

### Requirement 9: Cognitive Memory and Retrieval

**User Story:** As an operator, I want useful experience retained with privacy controls, so that future plans can use approved history without turning memory into an uncontrolled authority source.

#### Acceptance Criteria

1. THE Cognitive_Memory SHALL store versioned records for approved observations, goals, Structured_Plans, validation results, execution outcomes, failures, evaluations, recommendations, and operator annotations with source and retention metadata.
2. WHEN a record is written, THE Cognitive_Memory SHALL apply the configured privacy class, retention rule, consent or operator policy, integrity metadata, and deletion or expiration behavior.
3. WHEN Retrieval supplies context to a Model_Request, THE Cognitive_Memory SHALL return only records permitted for the requester, placement, privacy class, and task scope.
4. THE Cognitive_Runtime SHALL distinguish retrieved evidence from authoritative current state, capabilities, permissions, and Safety_Governor decisions.
5. IF retrieved content contains Prompt_Injection or conflicts with current policy, THEN THE Retrieval layer SHALL mark or exclude the content and SHALL not elevate the content to instructions or authority.
6. THE Cognitive_Memory SHALL support deterministic replay of retrieval inputs for a versioned query, policy, record set, and ordering configuration.

### Requirement 10: Evaluation, ModelBench, and Model Arena

**User Story:** As a release engineer, I want repeatable model evaluation, so that model selection is based on evidence rather than opaque claims.

#### Acceptance Criteria

1. THE ModelBench SHALL execute versioned, seeded scenarios against one or more models and the RulePlanner using identical task inputs, capability manifests, policies, and available evidence.
2. THE ModelBench SHALL measure at least schema validity, deterministic validation acceptance, safety violations or attempted boundary violations, task outcome, fallback rate, latency, resource usage, privacy policy outcomes, and configured cost metrics.
3. THE Model_Arena SHALL compare eligible models or configurations on the same scenario set and SHALL report per-metric results, aggregate scoring rules, sample counts, uncertainty or missing evidence, and the selected configuration.
4. THE Evaluation_System SHALL label software, virtual hardware, physics, adversarial, HIL, and robot evidence according to the existing Proving Ground levels and SHALL not infer an unearned level from a model score.
5. IF an evaluation run encounters invalid output, unavailable infrastructure, or incomplete evidence, THEN the report SHALL identify the condition and SHALL not present the run as a successful certification.
6. THE ModelBench and Model_Arena SHALL produce replayable Audit_Record references sufficient to reproduce the evaluated request, policy, model manifest, seed, and validator result.

### Requirement 11: Security and Prompt-Injection Resistance

**User Story:** As a security operator, I want model inputs and outputs treated as hostile data, so that cognitive features cannot expand authority or exfiltrate protected information.

#### Acceptance Criteria

1. THE Model_Fabric SHALL treat user text, retrieved memory, documents, sensor interpretations, tool output, provider output, and environment observations as untrusted data unless independently authorized by deterministic Nexus policy.
2. IF a Model_Request or model output contains an instruction to bypass safety, change permissions, reveal protected data, invoke an unauthorized tool, or alter system policy, THEN THE Cognitive_Runtime SHALL reject or isolate the instruction and SHALL record the security finding.
3. THE Cognitive_Runtime SHALL enforce an allowlisted tool and data-access surface for each Model_Role and placement.
4. THE Model_Fabric SHALL prevent secrets, credentials, private records, and safety-policy internals from being included in a request unless an explicit policy permits the specific data flow.
5. THE Security_Boundary SHALL remain effective when a provider is remote, a model is fine-tuned, a retrieved record is malicious, or a provider returns malformed or adversarial structured output.
6. THE Model_Fabric SHALL expose security-relevant refusals, quarantine decisions, and prompt-injection findings through Audit_Record values without exposing protected payload contents.

### Requirement 12: Auditability and Observability

**User Story:** As a reviewer, I want every cognitive decision traceable, so that model-assisted behavior can be investigated, replayed, and distinguished from deterministic authorization.

#### Acceptance Criteria

1. WHEN the Model_Fabric or Cognitive_Runtime handles a request, THE Audit_Record SHALL link request identity, caller, goal or task identity, model and provider manifests, placement decision, policy inputs, input references or redacted digests, output digest, validation findings, fallback, execution result, and operator action.
2. THE Audit_Stream SHALL distinguish model proposal, deterministic validation, Safety_Governor authorization, Adapter_Dispatch, cancellation, rejection, and recovery events.
3. THE Audit_Stream SHALL preserve correlation identifiers across Model_Fabric, Cognitive_Runtime, RulePlanner, skills, telemetry, replay, gateway, and Proving Ground evidence.
4. IF sensitive data is redacted from an Audit_Record, THEN THE record SHALL retain a stable digest and redaction reason sufficient to detect tampering or compare replay inputs without restoring the data.
5. THE audit and telemetry surfaces SHALL report model use, fallback use, budget consumption, provider health, validation rejection, safety rejection, and prompt-injection findings.
6. THE audit design SHALL not represent a model proposal as an executed or safety-approved action before the corresponding deterministic events exist.

### Requirement 13: Rust APIs, Workspace Features, and Compatibility

**User Story:** As a Rust developer, I want typed APIs integrated with the existing workspace, so that applications and adapters can adopt cognitive features without bypassing current contracts.

#### Acceptance Criteria

1. THE Rust Model_Fabric API SHALL provide typed manifests, roles, requests, responses, placement decisions, budget policies, privacy policies, provider errors, and audit events with validation methods that return structured errors.
2. THE Rust Cognitive_Runtime API SHALL provide typed goal submission, Structured_Plan generation, deterministic validation, critique and evaluation results, fallback status, and execution handoff through existing runtime abstractions.
3. THE Rust APIs SHALL represent model proposals separately from executable skill or adapter commands and SHALL not expose a model-only actuator dispatch function.
4. WHERE a Model_Provider or transport is optional, THE workspace SHALL gate the implementation behind a named Nexus_Feature and SHALL retain a deterministic no-provider build and test path.
5. THE Rust APIs SHALL preserve `unsafe_code = "forbid"`, existing NCM capability semantics, existing Safety_Governor contracts, and compatibility with `cargo test --workspace` when optional providers are disabled.
6. THE integration surface SHALL allow future local, edge, and remote providers to implement Provider_Adapter without requiring changes to deterministic skill or adapter contracts.

### Requirement 14: CLI, UI, and Documentation

**User Story:** As an operator and developer, I want inspectable controls across the supported surfaces, so that Model Fabric behavior is understandable and manageable without hidden defaults.

#### Acceptance Criteria

1. THE Nexus CLI SHALL provide commands to inspect model manifests and registry state, validate provider adapters, preview routing and placement, inspect budgets and privacy decisions, run the RulePlanner, submit or preview cognitive goals, and show fallback and validation results.
2. THE Nexus CLI SHALL provide commands to run ModelBench and Model_Arena scenarios with explicit seed, model, provider, policy, and output options and SHALL label unavailable evidence as `NOT RUN`.
3. THE supported UI or Studio surface SHALL display eligible models and roles, placement rationale, budget and privacy status, structured plans, deterministic validation findings, Safety_Governor decisions, audit references, and operator approval state.
4. THE CLI and UI SHALL make model availability, remote transport status, provider health, and local fallback status explicit rather than implying that an unconfigured provider is active.
5. THE project documentation SHALL describe Model_Fabric architecture, provider adapter contracts, Model_Manifest format, routing and privacy policy, Cognitive_Runtime plan schema, RulePlanner fallback, security boundaries, audit interpretation, evaluation methodology, Rust feature flags, and known non-goals.
6. THE documentation SHALL state that simulation, ModelBench, and Model_Arena evidence do not by themselves establish HIL or physical-robot validation.

### Requirement 15: Cognitive Door Challenge

**User Story:** As a release reviewer, I want a canonical Cognitive_Door_Challenge, so that the complete cognitive path can be demonstrated with deterministic evidence and a meaningful safety boundary.

#### Acceptance Criteria

1. WHEN the Cognitive_Door_Challenge receives a versioned goal, world state, Capability_Manifest, policy, and seed, THE Cognitive_Runtime SHALL produce a reproducible cognitive result containing interpretation, required observations, Structured_Plan or bounded refusal, validation findings, and final outcome.
2. WHEN eligible model inference is enabled, THE Cognitive_Door_Challenge SHALL permit model proposals for goal interpretation, perception interpretation, critique, evaluation, or failure analysis while preserving deterministic validation and Safety_Governor authorization.
3. WHEN model inference is disabled, unavailable, over budget, or rejected, THE Cognitive_Door_Challenge SHALL complete through the RulePlanner or report a deterministic safe refusal with a typed reason.
4. THE Cognitive_Door_Challenge SHALL exercise at least capability resolution, perception evidence requirements, navigation or approach skill planning, door-state interpretation, permission or operating-envelope checks, validation, fallback, audit evidence, and replay.
5. THE Cognitive_Door_Challenge SHALL include cases for successful evidence gathering, missing capability, stale or contradictory observation, provider failure, prompt-injection content, budget exhaustion, and Safety_Governor rejection.
6. THE challenge report SHALL separate model proposal, deterministic plan validation, Safety_Governor authorization, adapter or simulator execution, and evidence level, and SHALL mark unrun levels as `NOT RUN`.
7. THE Cognitive_Door_Challenge SHALL never grant a model direct actuator authority or allow a model to bypass the Safety_Governor, even when the model proposes a successful door operation.

### Requirement 16: Scope, Rollout, and Non-Goals

**User Story:** As a project maintainer, I want the Phase VI/v4.0 boundary explicit, so that implementation can be delivered incrementally without overstating capability.

#### Acceptance Criteria

1. THE feature specification SHALL treat local deterministic operation, RulePlanner fallback, simulation, replay, auditability, and provider-independent contracts as required baseline scope.
2. THE feature specification SHALL treat live remote providers, hardware probing beyond existing profile inputs, encrypted remote synchronization, physical robot autonomy, and production model certification as gated integrations requiring their own transport, security, or validation evidence.
3. IF an optional provider, UI, transport, or hardware integration is not built or validated, THEN Nexus SHALL report the component as unavailable or `NOT RUN` rather than silently substituting an unverified capability.
4. THE release process SHALL require Proving Ground evidence appropriate to each changed deterministic runtime, skill, adapter, provider adapter, and Cognitive_Door_Challenge path before claiming the corresponding validation level.
5. THE feature SHALL preserve the existing architecture in which `nexus-core` owns stable contracts, adapters translate hardware or simulation, `nexus-runtime` owns lifecycle and safety dispatch, `nexus-brain` informs hardware-aware placement, and `nexus-gateway` preserves local safety and telemetry behavior during central disconnection.
