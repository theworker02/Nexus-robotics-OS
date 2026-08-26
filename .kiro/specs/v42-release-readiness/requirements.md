# Requirements Document

## Introduction

Nexus Robotics OS v4.2 is a public release-candidate maturity phase. It does not introduce another major runtime subsystem. It makes the existing Rust workspace, deterministic safety architecture, simulation and Proving Ground evidence, Model Fabric documentation, integrations, CLI, website, branding, and release automation coherent, truthful, installable, and maintainable.

The repository audit establishes the starting point: the effective workspace package version is currently 3.5.0; the changelog ends at 2.6.0; citation and security/website version references are inconsistent; the website and brand assets exist; CI runs workspace tests, two CLI smoke commands, and a website build; and no v4.2 release artifact, release workflow, or complete release gate exists. Requirements therefore distinguish implemented behavior, documentation/public-surface work, optional integrations, and evidence that is not yet run. No requirement may convert simulation or documentation into a physical validation claim.

The release identity is **Nexus Robotics OS**, with the primary tagline **“Make simple robots capable. Make capable robots yours.”** Public materials shall consistently communicate that Nexus is adaptive, interoperable, extensible, learnable, configurable, safe, and testable. The safety principle remains: **models propose, Nexus validates, Safety remains deterministic, robots act.**

## Glossary

- **Release_Candidate**: A versioned, installable, reviewable v4.2 candidate that has passed the applicable automated release gates but is not yet the stable release.
- **Release_Gate**: A named, reproducible check whose result is required before a release claim or artifact is published.
- **Evidence_Level**: The repository’s factual validation scale: L0 Software Verified, L1 Virtual Hardware Verified, L2 Physics Verified, L3 Adversarial Simulation Verified, L4 Hardware-in-the-Loop Verified, and L5 Physical Robot Verified.
- **Truth_Source**: The authoritative metadata from which versions, compatibility, validation status, and release references are generated or checked.
- **Compatibility_Record**: A source record describing an integration’s status, capabilities, validation evidence, documentation, and limitations.
- **Public_Surface**: README, root documentation, website, release notes, CLI help, generated API reference, examples, badges, social assets, and repository metadata.
- **Demo_Data**: Simulated or illustrative output that must be labeled `SIMULATION` or `DEMO DATA` when a reader could confuse it with physical evidence.
- **Actual_Implementation**: A behavior that exists in the repository and passes its applicable executable or contract checks.
- **Claim**: A public statement about functionality, compatibility, validation, safety, maturity, or project status.
- **Release_Manifest**: Machine-readable metadata describing component versions, protocol/specification versions, commit, artifacts, checksums, and evidence references.
- **Validation_Report**: Human- and machine-readable output summarizing executed tests, simulation, fault injection, compatibility checks, benchmarks, and explicitly unrun checks.
- **Integration_Manifest**: Source metadata for an integration, including status, capabilities, validation level, documentation, and limitations.
- **Skill_Manifest**: Source metadata for a skill, including name, description, required capabilities, compatible profiles, and evidence.
- **Canonical_Terminology**: The required names Nexus Robotics OS, Nexus Brain, Model Fabric, SenseHopping, StructureScan, Active Learning, Proving Ground, Connected Intelligence, and NCM.
- **Release_Channel**: One of `nightly`, `preview`, or `stable`.

## Requirements

### Requirement 1: Release Identity and Version Truth

**User Story:** As a user, I want every public surface to identify the same release, so that I can tell what I installed and what evidence applies to it.

#### Acceptance Criteria

1. THE repository SHALL define one authoritative v4.2 version source and SHALL derive or validate Cargo workspace/package metadata, README badges and prose, CHANGELOG, release notes, CITATION.cff, SECURITY.md, ROADMAP.md, website metadata and version banners, examples, reports, and package/spec references against it.
2. WHEN the v4.2 candidate is prepared, THE repository SHALL distinguish `v4.2.0-rc.1` from `v4.2.0` and SHALL not describe an RC as a stable release.
3. IF a public file contains a stale version, release claim, validation claim, or product identity that conflicts with the authoritative source, THEN the release gate SHALL fail with the file and mismatch identified.
4. THE canonical product name, primary tagline, secondary positioning, and seven product pillars SHALL be consistent across the README, website metadata, release notes, and core documentation.
5. THE release process SHALL never fabricate release dates, authors, contributors, DOI values, vendor verification, physical validation, customer claims, or organizational facts.

### Requirement 2: Repository Hygiene and Canonical Layout

**User Story:** As a contributor, I want a deliberate repository structure, so that I can find source, integrations, examples, documentation, evidence, and release tooling without abandoned detours.

#### Acceptance Criteria

1. THE repository audit SHALL classify every top-level directory and tracked public artifact as `KEEP`, `MERGE`, `RENAME`, `REIMPLEMENT`, `ARCHIVE`, or `REMOVE`, with the reason and disposition recorded.
2. THE audit SHALL identify and remove or exclude from release artifacts abandoned prototypes, duplicate modules, stale screenshots, unused configs, obsolete diagrams, dead stubs, generated build output, temporary notes, empty directories, and placeholder public documentation when they are not intentionally retained.
3. THE repository SHALL not introduce directories named `misc`, `stuff`, `random`, `old`, `done`, `temp`, or `unused` as a substitute for a documented disposition.
4. THE canonical layout SHALL preserve the existing workspace’s meaningful crate, app, integration, SDK, skills, examples, specifications, docs, website, assets, proving-ground, and GitHub surfaces; modules SHALL not be split into additional crates solely to increase crate count.
5. IF an existing requested surface is not implemented, THE repository SHALL document it as unavailable, deferred, or `NOT RUN` rather than create a nonfunctional placeholder that appears complete.

### Requirement 3: README and Public Product Communication

**User Story:** As a first-time visitor, I want the repository to explain and run Nexus quickly, so that I can evaluate it without owning a robot or relying on marketing claims.

#### Acceptance Criteria

1. THE README SHALL open with the canonical product name, primary tagline, concise product description, meaningful badges, and a professional SVG architecture visual representing Intelligence, Software, Physical, Skills, Safety Governor, Capability Layer, integrations, and Robot.
2. THE README SHALL include actual, tested paths for software-only quick start, Docker where supported, simulator/proving commands, hardware-profile inspection, Rust use, CLI use, documentation, contribution, citation, funding, roadmap, and license.
3. THE README SHALL explain Adaptive Hardware Profiling, Nexus Brain, Model Fabric, SenseHopping, StructureScan, Active Learning, Connected Intelligence, integrations, simulation, Proving Ground, and deterministic safety using implementation-accurate language.
4. THE README SHALL include a factual integration matrix and a dedicated Nori Robotics section that describes Nori as a community integration target and explicitly disclaims affiliation, endorsement, partnership, certification, and physical validation unless evidenced.
5. THE README SHALL define L0 through L5 and SHALL not use “production ready,” “fully autonomous,” “works with any robot,” “certified,” or equivalent claims without explicit evidence and scope.
6. THE README SHALL identify current limitations, including any unimplemented hardware probing, persistent memory, remote execution, live transport, GUI/Studio, package signing, or physical autonomy, when those remain true at release time.
7. THE README SHALL not contain unfinished public headings, contradictory feature counts, stale version language, fake metrics, unsupported badges, or commands that do not match the actual package and CLI names.

### Requirement 4: Documentation Completeness and Integrity

**User Story:** As a developer, I want coherent documentation from installation through architecture and safety, so that I can build an integration without outside explanation.

#### Acceptance Criteria

1. THE root documentation set SHALL include substantive README, LICENSE, CHANGELOG, ROADMAP, ARCHITECTURE, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, PRIVACY, GOVERNANCE, SUPPORT, FUNDING, AUTHORS, CITATION, RELEASES, and COMPATIBILITY content, with each file’s claims grounded in repository evidence.
2. THE documentation SHALL cover system overview, core runtime, NCM, skills, safety, adaptive intelligence, Model Fabric, memory, learning, Sense Fabric, Connect Layer, integrations, simulation, Proving Ground, data flow, trust boundaries, deployment, configuration, installation, CLI, and Rust API usage.
3. THE repository SHALL add or update ADRs for Rust-first runtime, capability abstraction, deterministic safety, model authority limits, simulation-first validation, software senses, memory, integration isolation, package architecture, and local/edge/cloud routing where those decisions apply.
4. THE repository SHALL provide a canonical glossary and SHALL use Canonical_Terminology consistently, including capitalization and names for validation levels and major subsystems.
5. THE documentation SHALL distinguish development-host support from robot-side runtime support and SHALL state that hardware control requires compatible adapters, configured limits, appropriate safety equipment, and physical validation.
6. THE documentation SHALL state that Active Learning does not weaken safety constraints or automatically promote production behavior, and that models never receive raw actuator authority.
7. Documentation examples and internal links SHALL be executable or link-checked where practical, and stale commands SHALL fail documentation CI rather than remain public.

### Requirement 5: Compatibility and Validation Truth

**User Story:** As a prospective integrator, I want compatibility and validation status generated from evidence, so that I can make a responsible adoption decision.

#### Acceptance Criteria

1. THE repository SHALL maintain source Compatibility_Record and Skill_Manifest metadata from which README tables, website integration/skills pages, compatibility reports, and release notes are generated or checked.
2. EACH compatibility record SHALL state discovery, telemetry, skills, simulation, current validation level, implementation status, documentation link, and limitations; unsupported fields SHALL be labeled `NOT RUN`, `UNVALIDATED`, or `UNAVAILABLE` as appropriate.
3. THE release gate SHALL reconcile known status conflicts, including skill counts, NCM version, ROS 2 live-transport status, Gazebo/L2 evidence scope, Nori status, LeRobot status, and physical/HIL claims.
4. THE repository SHALL preserve the distinction between L0, L1, L2, L3, L4, and L5 and SHALL not infer a higher level from a lower-level test, a documentation statement, a model score, or a simulated interface.
5. THE final validation report SHALL list every executed and unexecuted gate, its command or evidence source, commit/version, scenario, seed where applicable, robot profile, environment, and result.
6. Real hardware, HIL, vendor verification, and external provider checks SHALL be reported only when their corresponding evidence exists; otherwise the status SHALL remain explicitly unvalidated.

### Requirement 6: Release Validation and CI Gates

**User Story:** As a maintainer, I want release quality checks enforced automatically, so that v4.2 does not depend on a manual memory of commands.

#### Acceptance Criteria

1. CI SHALL run, or explicitly report why it cannot run, workspace formatting, Clippy, unit/integration/schema tests, Rust documentation, CLI smoke tests, simulation smoke scenarios, website installation/build/lint checks, documentation/link checks, and security/dependency checks appropriate to the repository.
2. The simulation gate SHALL cover the supported headless minimum scenarios: basic navigation, sensor failure, fetch object, connected workshop, and doorway learning, or shall mark unavailable scenarios `NOT RUN` with an explanation.
3. Heavy ModelBench, physics, and benchmark workloads SHALL be separate from normal pull-request checks and SHALL be available through manual, nightly, or release triggers with reproducible seeds.
4. Security CI SHALL include dependency auditing, secret scanning, and applicable static/container/SBOM checks without treating a filename grep as a complete security audit.
5. Website CI SHALL run `npm ci` from the locked website dependency state and SHALL run the actual lint/build commands supported by the current website package; it SHALL fail on broken internal links where the stack supports link checking.
6. THE release gate SHALL verify that generated reports, release metadata, examples, configuration samples, and package artifacts correspond to the candidate version.
7. A failed mandatory gate SHALL block RC/stable promotion and SHALL identify the failed check, environment, and remediation path.

### Requirement 7: Release Engineering and Artifacts

**User Story:** As a release manager, I want repeatable candidate and stable release procedures, so that users receive identifiable and verifiable artifacts.

#### Acceptance Criteria

1. THE repository SHALL document semantic versioning, `nightly`/`preview`/`stable` channels, release branches or tags, RC handling, security patches, deprecation policy, and compatibility guarantees appropriate to the project’s actual size.
2. THE repository SHALL provide a reviewable release-notes generator or script that derives content from changelog entries, commits, version metadata, compatibility state, and validation results without silently inventing content.
3. A v4.2 release candidate SHALL produce, where the component exists and passes its gate, CLI binaries, checksums, release manifest, compatibility report, validation report, benchmark summary, Docker images, SBOM, and signatures; absent components SHALL be marked unavailable rather than claimed.
4. `release-manifest.json` SHALL record component versions, protocol/specification versions, Git commit, artifacts, checksums, and evidence references.
5. The repository SHALL generate `validation-report.json` and `validation-report.html` with actual results and explicit `NOT RUN` entries.
6. Tagging `v4.2.0-rc.1` SHALL prepare candidate validation and tagging `v4.2.0` SHALL be blocked until all mandatory gates, documentation, package, install, and artifact checks pass.
7. Release rollback or correction procedures SHALL preserve the prior stable configuration and explain how users identify a superseded artifact.

### Requirement 8: Website and GitHub Pages

**User Story:** As a visitor, I want a fast, accessible, evidence-based project site, so that I can understand Nexus and navigate into its documentation.

#### Acceptance Criteria

1. THE official public website SHALL be built from `website/` and deployed through GitHub Pages workflow configuration; the release SHALL not introduce a separate commercial marketing site.
2. THE site SHALL provide navigation for Nexus, Platform, Skills, Intelligence, Models, Learning, Connect, Integrations, Simulation, Safety, Developers, Docs, and GitHub, with every published route either implemented or clearly labeled unavailable.
3. THE homepage SHALL use the v4.2 identity, hero message, real terminal snippets, existing brand assets, and a product story from hardware profile through capabilities, Brain, skills, sensing, learning, connected intelligence, and safe execution.
4. Hardware profiler, Model Fabric, connected-intelligence, benchmark, compatibility, and skills demonstrations SHALL use actual repository data or visibly label illustrative output as `SIMULATION` or `DEMO DATA`.
5. Skills and integrations pages SHALL be generated from Skill_Manifest and Integration_Manifest metadata where practical and SHALL display status, capabilities, validation, documentation, and limitations.
6. THE site SHALL include responsive desktop/tablet/mobile layouts, semantic HTML, keyboard navigation, focus states, sufficient contrast, reduced-motion support, metadata, favicon, Open Graph/Twitter cards, sitemap, robots.txt, version banner, and a documentation-focused 404 page.
7. THE site SHALL avoid external analytics as a required dependency, huge JavaScript bundles, stock imagery, generic AI imagery, fake screenshots, unsupported benchmarks, and unverified compatibility claims.

### Requirement 9: Brand, Social, and Repository Metadata

**User Story:** As a project maintainer, I want a consistent visual and repository identity, so that Nexus looks professional without pretending to be a company it is not.

#### Acceptance Criteria

1. THE brand asset set SHALL include usable logo, mark, wordmark, horizontal, stacked, monochrome, dark, light, favicon, GitHub social, crates.io social, Pages social, and brand guide assets, reusing or deliberately superseding the existing assets.
2. THE brand guide SHALL specify safe area, minimum size, light/dark and monochrome usage, backgrounds, incorrect usage, typography, palette, and the prohibition on robot heads, humanoid faces, brain/neural clichés, sparkles, lightning bolts, and cyberpunk styling.
3. THE GitHub social preview SHALL be a professional 1280×640 design containing Nexus Robotics OS, the primary tagline, logo, and a restrained system visualization.
4. Repository description, topics, favicon, website metadata, and social assets SHALL use the canonical product identity and SHALL not claim unsupported integrations or organizational relationships.
5. Public materials SHALL not imply offices, employees, investors, customers, enterprise deployments, vendor partnerships, or vendor endorsement unless factually documented.
6. Contributor and author attribution SHALL be derived from actual repository history or manually verified real contributors; no fictitious names SHALL be added.

### Requirement 10: GitHub Community and Governance Surfaces

**User Story:** As a contributor, I want clear contribution, security, governance, and issue paths, so that I can participate safely and effectively.

#### Acceptance Criteria

1. CONTRIBUTING.md SHALL document Rust and Docker setup, architecture, tests, skills, integrations, hardware, model providers, documentation, pull requests, and commit conventions using commands that exist.
2. CODE_OF_CONDUCT.md SHALL use a recognized open-source code of conduct without unnecessary invented legal language.
3. SECURITY.md SHALL document supported versions, reporting, response, threat model, skill/model/MCP/wireless/package security, and robot-control boundaries, while matching the current release version and actual support policy.
4. PRIVACY.md SHALL explain observable data, stored data, external routing, model providers, wireless observations, MCP data, telemetry, retention, and user controls in clear configurable terms.
5. GOVERNANCE.md SHALL define the actual project owner/maintainer/committer/contributor model, RFC process, release authority, and security responsibility without pretending the project has a larger foundation than it does.
6. SUPPORT.md SHALL direct users to Issues, Discussions, documentation, security reporting, feature requests, and integration requests.
7. GitHub issue templates SHALL cover bug, feature, skill, integration, hardware, simulation, and documentation requests; feature requests SHALL ask for problem, behavior, hardware impact, safety impact, compatibility impact, and alternatives.
8. CODEOWNERS, Dependabot, labels, discussion categories, and funding configuration SHALL contain only actual owners/providers and supported settings; absent ownership SHALL not be fabricated.

### Requirement 11: Funding, Citation, and Legal Attribution

**User Story:** As a user or researcher, I want accurate ways to support or cite Nexus, so that attribution and funding do not contain fabricated information.

#### Acceptance Criteria

1. `.github/FUNDING.yml` SHALL contain only the GitHub Sponsors account `theworker02` in the form `github: [theworker02]` or the equivalent valid GitHub syntax, and SHALL not add unsupported providers.
2. FUNDING.md SHALL identify `@theworker02` and explain that support may help development, hardware, sensors, servos, HIL rigs, embedded devices, simulation, CI, and documentation without promising specific spending.
3. CITATION.cff SHALL be valid, identify the canonical title, current release version, real authors, repository, license, and real release date only when known; it SHALL not fabricate a DOI.
4. README and website citation/funding sections SHALL link to the canonical files and GitHub Sponsors presence.
5. LICENSE, dependency licenses, integrations, vendored content, and third-party assets SHALL be audited for required attribution, with NOTICE.md added when warranted.

### Requirement 12: CLI, Rust Distribution, and Examples

**User Story:** As a developer, I want a polished local CLI and Rust package surface, so that I can install, inspect, simulate, and extend Nexus predictably.

#### Acceptance Criteria

1. THE CLI SHALL provide consistent help, errors, formatting, exit codes, and useful `--json` machine output where supported, including `nexus --help`, `nexus version`, `nexus doctor`, and the canonical `nexus demo` path when those commands are claimed publicly.
2. `nexus version` SHALL report CLI, runtime, NCM, skill specification, commit, build date, and target architecture values when available, without displaying guessed values.
3. CLI failures SHALL identify the operation, reason, available capabilities or alternatives, and recommended next step; opaque numeric errors alone SHALL not be used for public failures.
4. Primary published crates SHALL have accurate description, repository, homepage, documentation, license, keywords, categories, readme, and feature metadata; undocumented public APIs SHALL be reduced, documented, or excluded from the release surface.
5. Public Rust APIs SHALL undergo a naming and abstraction review and SHALL preserve `unsafe_code = "forbid"`, existing safety contracts, and optional-provider feature boundaries.
6. The ergonomic prelude and examples SHALL be added only for APIs that exist and compile; examples SHALL cover discovery, capabilities, skills, model configuration, MCP connection, simulation, and Active Learning only where those surfaces are implemented.
7. Documentation CI SHALL compile or execute public Rust examples and CLI snippets where practical, preventing stale commands from shipping.

### Requirement 13: Safety, Security, and Claim Audits

**User Story:** As a safety-conscious user, I want release materials to preserve the execution boundary and expose unsupported claims, so that polish never hides risk.

#### Acceptance Criteria

1. Public safety documentation SHALL show the boundary `Model → Candidate → Validation → Safety → Skill → Robot` and SHALL state that language models do not receive raw actuator authority.
2. THE deterministic runtime SHALL remain authoritative for actuator limits, joint/torque limits, emergency stop, vendor limits, permissions, resource locks, motion policies, and no-auto-resume behavior; documentation changes SHALL not weaken these controls.
3. THE release audit SHALL search for unsupported or high-risk phrases such as `production-ready`, `industry-leading`, `works with any robot`, `fully autonomous`, `certified`, and `official integration`, and each occurrence SHALL be removed, scoped, or evidence-linked.
4. THE release audit SHALL search for `TODO`, `TBD`, `Coming soon`, `Placeholder`, `Lorem ipsum`, `example.com`, `your-name`, and `your-org`; intentional internal occurrences SHALL be documented or excluded from public surfaces.
5. THE release audit SHALL scan tracked content and release artifacts for tokens, API keys, passwords, Wi-Fi credentials, private URLs, and other secrets, with any finding blocking release until resolved.
6. External text, model output, MCP data, wireless data, integrations, skills, peer robots, and network services SHALL remain untrusted inputs and SHALL not rewrite safety policy or authority.

### Requirement 14: Demo, Screenshots, and Public Experience

**User Story:** As a visitor, I want one honest path from clone to meaningful behavior, so that Nexus is understandable without outside explanation.

#### Acceptance Criteria

1. `nexus demo` SHALL provide one canonical software-only guided experience using available deterministic profiles, skills, SenseHopping, StructureScan, Brain/profile information, MCP mocks or available connected-intelligence fixtures, learning/simulation data, replay, and Proving Ground evidence without requiring hardware or cloud services.
2. THE guided demo SHALL label simulated output and SHALL never present a fake screenshot, synthetic metric, or illustrative profile as physical evidence.
3. Real screenshots used in README, Pages, release notes, or social assets SHALL be generated from actual Nexus Studio, Proving Ground, Model Arena, Hardware Profiler, or Connections output; nonexistent UI SHALL not be depicted.
4. THE demo SHALL explain the progression from robot discovery/profile through capability resolution, task/skill execution, safety validation, failure or learning evidence where implemented, and replay.
5. Demo commands, screenshots, and public instructions SHALL pass the same version and claim audits as the rest of the release.

### Requirement 15: Release Scope, RC Gate, and Non-Goals

**User Story:** As a maintainer, I want a credible release boundary, so that v4.2 can ship without pretending deferred work is complete.

#### Acceptance Criteria

1. THE v4.2 scope SHALL prioritize release identity, repository hygiene, documentation, factual compatibility, CI/release gates, website, branding, CLI/package polish, security/privacy/governance, citation/funding, and the honest demo path.
2. THE v4.2 scope SHALL not silently introduce claims of universal robot compatibility, unrestricted model control, custom-kernel replacement, automatic physical safety, complete vendor replacement, HIL/physical validation without hardware, or a large-company operating model.
3. THE release plan SHALL explicitly distinguish implemented, contract-tested, simulated, experimental, unavailable, deferred, and `NOT RUN` work for Model Fabric, MCP, live transports, GUI/Studio, persistent memory, remote execution, HIL, hardware probing, package signing, and physical autonomy.
4. THE `v4.2.0-rc.1` gate SHALL require passing mandatory CI, documentation, website, metadata, security, package, install, demo, compatibility, and release-artifact checks; it SHALL produce the release and validation reports.
5. THE stable `v4.2.0` gate SHALL additionally require review of RC install instructions, crate packaging, GitHub Pages deployment, Docker artifacts where present, release archives, known limitations, and public-claim audit results.
6. IF a gate cannot run because hardware, an external service, a provider, or a tool is unavailable, THE report SHALL state `NOT RUN`, the reason, the affected claim, and the next validation path; it SHALL not convert absence of evidence into success.
7. THE final release summary SHALL use the restrained title **Nexus Robotics OS v4.2 — Public Release** and SHALL include mandatory Known Limitations and Validation sections.
