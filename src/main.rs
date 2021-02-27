//!
//! Don't forget to install `libvpx`.
//!
mod fmp4;
mod yuv_util;

use fmp4::{Fmp4};
use image::{imageops::FilterType, GenericImageView, ImageFormat};
use std::time::Instant;
use std::{fs::File, io::Cursor};
use std::{fs::OpenOptions, io::Write, u32};
use std::{io::Read, vec};
use yuv_util::convert_rgb_to_yuv420p;

const OUTPUT_DIR: &str = "m4s";
const HEADER_NAME: &str = "m4s/header.m4s";
const SEGMENT_PREFIX: &str = "m4s/body";

fn main() -> anyhow::Result<()> {
    let width = 1920;
    let height = 1080;
    let fps = 30u32;
    let bitrate = 1920 * 2;

    let mut vpx = vpx_encode::Encoder::new(vpx_encode::Config {
        width: width,
        height: height,
        timebase: [1, 1000_000_000],
        bitrate: bitrate,
        codec: vpx_encode::VideoCodecId::VP9,
        kf_max_dist: fps * 2,
        quantizer: (32, 32),
        threads: num_cpus::get() as _,
    })?;
    println!("created the encoder");

    std::fs::remove_dir_all(OUTPUT_DIR).ok();
    std::fs::create_dir(OUTPUT_DIR).ok();

    let mut fmp4 = Fmp4::new(fps, width as _, height as _);
    output_data(fmp4.init_segment(), false, HEADER_NAME);
    // Start recording.
    for i in 0..1200 {
        let buffer= read_image(i)?;

        let now = Instant::now();
        let yuv = convert_image(&buffer, width, height)?;

        for frame in vpx.encode(0i64, &yuv).unwrap() {
            output_data(fmp4.wrap_frame(frame.data, frame.key), frame.key, &format!("{}_{}.m4s", SEGMENT_PREFIX, i));
        }
        println!("#{}, cost={}", i, now.elapsed().as_millis());
    }

    // End things.
    let mut frames = vpx.finish().unwrap();
    while let Some(_frame) = frames.next().unwrap() {
        println!("WARNING, frame after finishing");
    }

    Ok(())
}

fn read_image(i: u32) -> anyhow::Result<Vec<u8>> {
    let mut buffer = vec![];
    File::open(format!("./frames/frame{}.png", 1 + i))?.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn convert_image(image_bytes: &[u8], out_width: u32, out_height: u32) -> anyhow::Result<Vec<u8>> {
    let reader = Cursor::new(image_bytes);
    let mut raw_img = image::io::Reader::with_format(reader, ImageFormat::Png).decode()?;
    // resize image, it costs a lot of time
    if (out_width, out_height) != (raw_img.width(), raw_img.height()) {
        // println!("wh={},{}", raw_img.width(), raw_img.height());
        raw_img = raw_img.resize_exact(out_width as _, out_height as _, FilterType::Nearest);
    }
    let img = raw_img.to_rgb8();

    let yuv = convert_rgb_to_yuv420p(img.as_ref(), img.width(), img.height(), 3);
    Ok(yuv)
}

#[allow(unused)]
fn output_data(bytes: Vec<u8>, _key: bool, filename: &str) {
    // println!("[output_data] len={}, key={}", bytes.len(), key);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
        .unwrap();
    file.write_all(&bytes).unwrap();
}
