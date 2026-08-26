#!/usr/bin/env python3
"""Live ROS 2 transport assertion for the NXR-2 move-forward skill."""
import sys
import time

import rclpy
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
from rclpy.node import Node


class MoveForwardAssertion(Node):
    def __init__(self) -> None:
        super().__init__('nexus_physics_adapter')
        self.publisher = self.create_publisher(Twist, '/cmd_vel', 10)
        self.create_subscription(Odometry, '/nxr2/odometry', self.on_odometry, 10)
        self.samples: list[float] = []
        self.started = time.monotonic()
        self.timer = self.create_timer(0.1, self.command)

    def on_odometry(self, message: Odometry) -> None:
        self.samples.append(message.pose.pose.position.x)

    def command(self) -> None:
        elapsed = time.monotonic() - self.started
        message = Twist()
        if elapsed < 3.0:
            message.linear.x = 0.25
        self.publisher.publish(message)
        if elapsed > 8.0:
            self.timer.cancel()


def main() -> int:
    rclpy.init()
    node = MoveForwardAssertion()
    deadline = time.monotonic() + 10.0
    while time.monotonic() < deadline:
        rclpy.spin_once(node, timeout_sec=0.2)
    node.destroy_node()
    rclpy.shutdown()
    if len(node.samples) < 2:
        print('NEXUS_PHYSICS_ASSERTION FAIL: no bridged NXR-2 odometry received')
        return 1
    displacement = max(node.samples) - min(node.samples)
    if displacement < 0.20:
        print(f'NEXUS_PHYSICS_ASSERTION FAIL: move-forward displacement {displacement:.3f} m')
        return 1
    print(f'NEXUS_PHYSICS_ASSERTION PASS: move-forward displacement {displacement:.3f} m; ROS 2 cmd_vel -> Gazebo transport -> NXR-2 base')
    return 0


if __name__ == '__main__':
    sys.exit(main())
