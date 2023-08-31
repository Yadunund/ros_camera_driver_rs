use byteorder::{BigEndian, NativeEndian};

use clap::Parser;

use opencv::{highgui, prelude::*, videoio, Result};

use sensor_msgs::msg::Image as Image;

use core::slice;
use std::env;
use std::{error, mem, time::{Duration, SystemTime, UNIX_EPOCH}, thread::{self, sleep}};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct CommandLineArgs {
    /// The topic to publish the frames.
    #[arg(short, long, default_value_t = String::from("bgr8"))]
    encoding: String,

    /// The frame_id for the camera.
    #[arg(short, long, default_value_t = String::from("camera_link"))]
    frame_id: String,

    /// The frequency in hz at which to publish the frames.
    #[arg(short, long, default_value_t = 15.0)]
    rate: f32,

    /// The topic to publish the frames.
    #[arg(short, long, default_value_t = String::from("image_raw"))]
    topic: String,
}


fn main() -> Result<(), Box<dyn error::Error>> {
    // Get the CLI args for t
    let args = CommandLineArgs::parse();

    let context = rclrs::Context::new(env::args())?;
    let node = rclrs::create_node(&context, "camera_driver_node")?;
    let _spin_node = node.clone();
    thread::spawn(move || {
        rclrs::spin(_spin_node);
    });


    let publisher = node.create_publisher::<Image>(&args.topic, rclrs::QOS_PROFILE_SENSOR_DATA)?;

	let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?; // 0 is the default camera
	let opened = videoio::VideoCapture::is_opened(&cam)?;
	if !opened {
		panic!("Unable to open default camera!");
	}

    let mut last_now = SystemTime::now();
    let period = Duration::from_millis((1000.0 / args.rate) as u64 );

    while context.ok() {
		let mut frame = Mat::default();
		cam.read(&mut frame)?;
		if frame.size()?.width == 0 {
            continue;
		}

        let mut message = Image::default();
        let time_now = SystemTime::now();
        message.header.frame_id = args.frame_id.clone();
        message.header.stamp.sec = time_now.duration_since(UNIX_EPOCH)?.as_secs() as i32;
        message.header.stamp.nanosec = time_now.duration_since(UNIX_EPOCH)?.as_nanos() as u32;

        message.height = frame.size()?.height as u32;
        message.width = frame.size()?.width as u32;

        message.encoding = args.encoding.clone();
        // message.is_bigendian = NativeEndian == BigEndian;

        let length = message.height * message.width - 1;
        // unsafe {
        //     let data = slice::from_raw_parts(frame.data(), length as usize);
        //     message.data = Vec::from(data);
        // }
        // message.data = frame.data_bytes()?.to_vec();
        message.data = Vec::with_capacity(length as usize);
        message.data = vec![255; length as usize ];
        // let bytes = unsafe {slice::from_raw_parts(bytes, bytes_length)};
        // let mut bytes: Vec<u8> = Vec::from(bytes);
        publisher.publish(&message)?;

        let sleep_till = last_now + period;
        let sleep_for : Duration = sleep_till.duration_since(time_now)?;
        last_now = sleep_till;
        sleep(sleep_for);
    }

	Ok(())
}