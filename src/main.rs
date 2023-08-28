#![allow(dead_code, unused_imports)]

pub const INF: f64 = 9999999999999999999.0;

mod ray;
mod vec3;
mod primitives;
mod scene;
mod util;

use ray::*;
use vec3::*;
use image::{RgbImage, Rgb, ImageBuffer};
use scene::*;
use primitives::*;


fn main() {
    let canvas_width: i32 = 1000;
    let canvas_height: i32 = 1000;

    let viewframe_width: f64 = 1.0;
    let viewframe_height: f64 = 1.0;
    let viewframe_distance: f64 = 1.0;

    let camera = vec3::new(0.0, 0.0, 0.0);

    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);   
    let mut img = RgbImage::new(canvas_width as u32, canvas_height as u32);

    //draw_pixel(&mut img, 100, 100, Rgb([255, 255, 255]));

    for image_x in -canvas_width/2 .. canvas_width/2  {
        for image_y in -canvas_height/2 .. canvas_height/2 {
            let viewframe_x:f64 = image_x as f64 * (viewframe_width / canvas_width as f64);
            let viewframe_y:f64 = image_y as f64 * (viewframe_height / canvas_height as f64);

            let ray = ray::new(camera, vec3::new(viewframe_x, viewframe_y, viewframe_distance)); //ray from the camera to a physical point on the viewframe
            
            let draw_color: Rgb<u8> = match ray.closest_point(&scene.primitives, 1.0, INF) {
                Some(t) => t.1.color(),
                None => Rgb([255, 255, 255])
            };

            draw_pixel(&mut img, image_x, image_y, draw_color);
        }
    }


    img.save("output.png").unwrap();
}

fn init_scene(scene: &mut Scene) {
    scene.primitives.push(new_sphere(
        vec3::new(0.0, -1.0, 3.0),
        1.0,
        Rgb([255, 0, 0]),
        0,
        0.0));
    
    scene.primitives.push(new_sphere(
        vec3::new(2.0, 0.0, 4.0),
        1.0,
        Rgb([0, 0, 255]),
        0,
        0.0));
    
    scene.primitives.push(new_sphere(
        vec3::new(-2.0, 0.0, 4.0),
        1.0,
        Rgb([0, 255, 0]),
        0,
        0.0));
}

fn draw_pixel(img: &mut RgbImage, x: i32, y: i32, draw_color: Rgb<u8>) {
    let corrected_x: i32 = img.width() as i32 / 2 + x;
    let corrected_y: i32 = img.height() as i32 / 2 - y; 

    println!("Drawing at: x({}), y({})", corrected_x, corrected_y);

    img.put_pixel(corrected_x as u32, (corrected_y - 1) as u32, draw_color);
}