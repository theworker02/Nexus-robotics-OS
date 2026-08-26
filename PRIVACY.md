# Privacy

Nexus is local-first and simulation-first. The default repository workflows do not upload telemetry or require a cloud account.

## Data categories

Nexus may process robot capability manifests, sensor summaries, task and skill events, replay records, model context, MCP responses, wireless observations, and operator-provided configuration. The exact data depends on enabled adapters and integrations.

## Storage and external routing

Local runtime state and simulation evidence remain local unless an operator configures an external service. Remote model providers, MCP servers, telemetry endpoints, and wireless devices are optional integrations and must be treated as explicit data flows. Do not send camera frames, credentials, private robot data, or safety-policy internals externally without an explicit policy permitting it.

## Controls

Operators should review enabled providers, retention settings, package permissions, MCP permissions, and adapter configuration before connecting hardware or external services. Persistent memory, remote execution, encrypted synchronization, and several live transports remain implementation- and validation-dependent in this release candidate.
