# ros_camera_driver_rs

A ROS 2 driver for webcam or other V4L devices written with the Rust.

## Requirements
* [ROS 2](https://docs.ros.org/) Humble/Iron/Rolling
* [ros2_rust](https://github.com/ros2-rust/ros2_rust) client library
* [opencv](https://opencv.org/) v3.4 or v4.x

## Build with colcon
```bash
mkdir ~/ws_camera_driver/src -p && cd ~/ws_camera_driver/src
git clone https://github.com:Yadunund/ros_camera_driver_rs
cd ~/ws_camera_driver
// Source ros2_rust workspace 
colcon build --mixin release
```

## Build with cargo
```bash
git clone https://github.com:Yadunund/ros_camera_driver_rs
cd ~/ws_camera_driver
// Source ros2_rust workspace 
colcon build --release
```

## Usage
```bash
Usage: camera_driver_node [OPTIONS]

Options:
  -c, --camera-idx <CAMERA_IDX>  The index of the camera to open [default: 0]
  -e, --encoding <ENCODING>      The topic to publish the frames [default: bgr8]
  -f, --frame-id <FRAME_ID>      The frame_id for the camera [default: camera_link]
  -r, --rate <RATE>              The frequency in hz at which to publish the frames [default: 30]
  -t, --topic <TOPIC>            The topic to publish the frames [default: image_raw]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Run
```bash
// If built using colcon, source the workspace and
ros2 run ros_camera_driver_rs camera_driver_node // [OPTIONS]

// If built using cargo,
cargo run --release // [OPTIONS] 
```