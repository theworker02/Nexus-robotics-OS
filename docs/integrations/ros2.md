# ROS 2 compatibility

The ROS 2 adapter maps common topic and action types into discovered NCM 2.0 capabilities: image feeds, IMU, battery, lidar/point clouds, velocity commands, and joint trajectories. It is an adapter boundary, not the Nexus operating model.

The current crate provides and tests graph-to-capability mapping. It does not ship a live ROS middleware transport or claim certification against a physical ROS 2 robot.
