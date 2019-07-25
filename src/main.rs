use std::convert::TryInto;
use std::net::UdpSocket;
use std::{thread, time};
use palette::Color;
use palette::LinSrgb;
use palette::Hsv;

// The beginning of an OpenPixelControl client in Rust

fn main() {
    let num_pixels = 240;
    let packet_size = 4 + (num_pixels * 3);

    let mut payload: Vec<u8> = vec![0u8; packet_size];

    payload[0] = 0x01; // Channel 1
    payload[1] = 0x00; // Command 0 (set 8-bit pixel colors)
    
    let len: u16 = (num_pixels * 3).try_into().unwrap();
    payload[2] = (len >> 8) as u8;
    payload[3] = len as u8;

    let my_rgb: LinSrgb = Hsv::new(155., 1., 0.01).into(); // see https://stackoverflow.com/a/12894053 for Srgb vs LinSrgb
    let my_u8_rgb: LinSrgb<u8> = my_rgb.into_format();

    for i in (4..packet_size-2).step_by(3) {
        payload[i] = my_u8_rgb.red;
        payload[i+1] = my_u8_rgb.green;
        payload[i+2] = my_u8_rgb.blue;
    }

    let socket = UdpSocket::bind("0.0.0.0:34567").unwrap();

    let frame_rate = 30;
    let frame_delay_millis = 1_000 / frame_rate;
    let frame_delay_millis = time::Duration::from_nanos(frame_delay_millis); // TODO use from_secs_f64 when it's stable

    loop {
        socket.send_to(&payload, "192.168.0.53:5000").unwrap();
        thread::sleep(frame_delay_millis);
    }
}

fn build_payload(pixels: Vec<Hsv>, payload: &mut Vec<u8>) {

    // TODO set first four bytes

    for (i, pixel) in pixels.iter().enumerate() {

        let pixel: LinSrgb = (*pixel).into(); // TODO did I use * right in Rust?
        let pixel: LinSrgb<u8> = pixel.into_format();

        payload[4 + i*3    ] = pixel.red;
        payload[4 + i*3 + 1] = pixel.green;
        payload[4 + i*3 + 2] = pixel.blue;
    }
}