use std::convert::TryInto;
use std::net::UdpSocket;
use palette::Color;
use palette::LinSrgb;
use palette::Hsv;
use actix_web::{web, App, Responder, HttpServer};

fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new().service(
        web::resource("/set_hsv/{h}/{s}/{v}").to(set_hsv))
    )
        .bind("0.0.0.0:80")?
        .run()
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

fn set_hsv(info: web::Path<(f32, f32, f32)>) -> impl Responder {
    let num_pixels = 240;
    let packet_size = 4 + (num_pixels * 3);

    let mut pixels: Vec<Color> = vec![Color::default(); num_pixels];
    let mut payload: Vec<u8> = vec![0u8; packet_size];

    fill_solid_color(&mut pixels, Hsv::new(info.0, info.1, info.2).into());
    build_payload(&mut payload, &pixels);

    let socket = UdpSocket::bind("0.0.0.0:34567").unwrap();
    socket.send_to(&payload, "192.168.0.53:5000").unwrap();
}