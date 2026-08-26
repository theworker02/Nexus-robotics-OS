# Container development

The current container surface runs the real CLI and deterministic simulator. It is deliberately a test/demo environment, not a claim that Docker validates physical servo torque, collisions, electrical noise, camera latency, or battery behavior.

```bash
docker compose -f compose.dev.yml run --rm nexus-simulator
docker compose -f compose.dev.yml --profile nori run --rm nexus-nori-adapter
docker compose -f compose.dev.yml --profile test run --rm skill-test
```

All containers run as a non-root user, with a read-only root filesystem, no added Linux capabilities, and no device passthrough. Hardware device access must be declared explicitly in a future adapter-specific compose override.
