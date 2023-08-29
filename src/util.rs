use image::{Rgb, RgbImage};
use crate::vec3::*;

pub const INF: f64 = 999999999999.0;
pub const EPSILON: f64 = 0.0000000001;

pub fn in_range<T: PartialOrd>(x: T, min: T, max: T) -> bool {
    min < x && x < max
}

pub fn v3_to_rgb(v: V3) -> Rgb<u8> {
    let r:u8 = v.x.clamp(0.0, 255.0).round() as u8;
    let g:u8 = v.y.clamp(0.0, 255.0).round() as u8;
    let b:u8 = v.z.clamp(0.0, 255.0).round() as u8;

    Rgb([r, g, b])
}

pub fn draw_pixel(img: &mut RgbImage, x: i32, y: i32, draw_color: Rgb<u8>) {
    let corrected_x: i32 = img.width() as i32 / 2 + x;
    let corrected_y: i32 = img.height() as i32 / 2 - y; 

    //println!("Drawing at: x({}), y({})", corrected_x, corrected_y);

    img.put_pixel(corrected_x as u32, (corrected_y - 1) as u32, draw_color);
}