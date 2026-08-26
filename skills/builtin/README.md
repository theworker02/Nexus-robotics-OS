# Built-in skills

These packages describe the skills provided by the Phase I runtime. Their controlled simulator implementations and behavior tests live in `crates/nexus-runtime`; the manifests are intentionally portable package metadata for future signed skill artifacts.

| Skill | Purpose |
| --- | --- |
| stop | Cancel movement immediately |
| speak | Speech output |
| look_at | Orient vision to a target |
| turn_left / turn_right | Safe rotation |
| move_forward / move_backward | Bounded teleoperation motion |
| walk_to | Navigation to a known location |
| follow_target | Vision-guided following |
| pick_up / place_object | Object manipulation |
| inspect_object / scan_room | Perception actions |
| return_home / dock | Recovery and charging |
