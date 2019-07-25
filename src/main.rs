use std::convert::TryInto;
use std::net::UdpSocket;
use std::{thread, time};
use palette::Color;
use palette::LinSrgb;
use palette::Hsv;
use palette::Gradient;

// The beginning of an OpenPixelControl client in Rust

fn main() {
    let num_pixels = 240;
    let packet_size = 4 + (num_pixels * 3);

    let mut pixels: Vec<Color> = vec![Color::default(); num_pixels];
    let mut payload: Vec<u8> = vec![0u8; packet_size];

    let color1 = Hsv::new(100.0, 1.0, 1.0);
    let color2 = Hsv::new(200.0, 1.0, 0.0);

    let interp = Gradient::new(vec![color1, color2]);
    let states: Vec<Hsv> = interp.take(120).collect();

    let socket = UdpSocket::bind("0.0.0.0:34567").unwrap();

    let frame_rate = 30;
    let frame_delay_millis = 1_000 / frame_rate;
    let frame_delay_millis = time::Duration::from_nanos(frame_delay_millis); // TODO use from_secs_f64 when it's stable

    let mut i = 0;
    loop {
        let next_color = states[i];

        fill_solid_color(&mut pixels, next_color.into());
        build_payload(&mut payload, &pixels);

        socket.send_to(&payload, "192.168.0.53:5000").unwrap();

        i = (i + 1) % 120;
        thread::sleep(frame_delay_millis);
    }
}

fn fill_solid_color(pixels: &mut Vec<Color>, color: Color) {
    for i in 0..pixels.len() {
        pixels[i] = color;
    }
}

fn build_payload(payload: &mut Vec<u8>, pixels: &Vec<Color>) {

    payload[0] = 0x01; // Channel 1
    payload[1] = 0x00; // Command 0 (set 8-bit pixel colors)
    
    let len: u16 = (payload.len() - 4).try_into().unwrap(); // asinine but required by OPC
    payload[2] = (len >> 8) as u8;
    payload[3] = len as u8;

    for (i, pixel) in pixels.iter().enumerate() {

        let pixel: LinSrgb = (*pixel).into();
        let pixel: LinSrgb<u8> = pixel.into_format();

        payload[4 + i*3] = pixel.red;
        payload[4 + i*3 + 1] = pixel.green;
        payload[4 + i*3 + 2] = pixel.blue;
    }
}