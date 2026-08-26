# NCM 2.0 — Dynamic capability manifest

NCM 2.0 extends NCM 1.0 with per-capability provenance. A record must identify whether its value is `STATIC`, `DISCOVERED`, `OVERRIDDEN`, or `DERIVED`, plus its provider and observation timestamp when available. Applications consume the capability layer rather than vendor-specific implementation.

```yaml
capability:
  name: vision.rgb
  value: true
  source:
    type: discovered
    provider: nori-community-adapter
```

Adapters must never infer unsupported capabilities. In particular, RGB cameras do not imply depth sensing.
