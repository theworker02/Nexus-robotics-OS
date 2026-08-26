# Releases

Nexus uses semantic versioning where practical and publishes three channels:

- **nightly**: development builds; APIs and behavior may change.
- **preview**: release candidates such as `4.2.0-rc.1`; intended for validation before stable promotion.
- **stable**: reviewed releases such as `4.2.0`.

A release must identify its Git commit, component versions, compatibility state, validation evidence, known limitations, and available artifacts. Security fixes may be released independently. Deprecations should identify the affected surface, replacement, and expected removal window.

Publishing a tag does not by itself establish physical, HIL, vendor, or production validation. Those claims require corresponding evidence.
