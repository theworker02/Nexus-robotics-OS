# Active Learning

Nexus Active Learning is a controlled, simulation-first improvement loop. It records task attempts, classifies failure, proposes a bounded parameter adjustment, validates it in the scenario, and creates an auditable candidate artifact. It does not silently change a production skill.

```powershell
cargo run -p nexus-cli -- learn create
cargo run -p nexus-cli -- learn start --no-ai
```

Physical learning requires a separately implemented explicit enablement mode, safety envelope, attempt budget, operator stop, HIL validation, and hardware review.
