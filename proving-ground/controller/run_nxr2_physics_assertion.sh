#!/usr/bin/env bash
set -euo pipefail

export HOME=/tmp
export XDG_CACHE_HOME=/tmp/cache
export GZ_LOG_PATH=/tmp/gz-log
export ROS_LOG_DIR=/tmp/ros-log
export GZ_PARTITION=nexus_proving

gz sim -s -r /opt/nexus/worlds/doorway-lab.sdf > /tmp/gazebo.log 2>&1 &
gazebo_pid=$!
sleep 2
ros2 launch ros_gz_bridge ros_gz_bridge.launch.py bridge_name:=nexus_proving config_file:=/opt/nexus/bridge/topics.yaml > /tmp/bridge.log 2>&1 &
bridge_pid=$!
sleep 2

set +e
python3 /opt/nexus/controller/nexus_physics_adapter.py
status=$?
set -e
kill "$bridge_pid" "$gazebo_pid" 2>/dev/null || true
wait "$bridge_pid" 2>/dev/null || true
wait "$gazebo_pid" 2>/dev/null || true
if [ "$status" -ne 0 ]; then
  echo "Gazebo log:"
  cat /tmp/gazebo.log
  echo "Bridge log:"
  cat /tmp/bridge.log
fi
exit "$status"
