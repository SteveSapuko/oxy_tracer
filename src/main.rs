#![allow(dead_code, unused_imports)]

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
use util::*;

use nalgebra::{Vector3, Rotation3, vector};
use std::ops::Mul;

const MAX_COLOR_DIFFERENCE: f64 = 10.0;

fn main() {
    let now = std::time::Instant::now();
    
    let rotation: Rotation3<f64> = Rotation3::from_euler_angles(0.0, 0.0, 0.0);
    
    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);   
    let mut img = RgbImage::new(scene.canvas_width as u32, scene.canvas_height as u32);

    let mut times = 0;
    for canvas_x in -scene.canvas_width/2 .. scene.canvas_width/2  { //draw entire image without supersampling (1 ray per pixel)
        for canvas_y in -scene.canvas_height/2 .. scene.canvas_height/2 {

            let draw_color = generate_pixel(&scene,
                (canvas_x, canvas_y),
                &rotation,
                1);

            let draw_color = v3_to_rgb(draw_color);
            draw_pixel(&mut img, canvas_x, canvas_y, draw_color).unwrap();
        }
    } 

    /*Goes through generated image. Regenerates pixels that have high variation from their neighbors with supersampling.
    For simplicity, only checks pixels that have 8 neighbors*/
    
    for canvas_x in 1 .. scene.canvas_width - 1 {
        for canvas_y in 1 .. scene.canvas_height -1{
            let current_color = img.get_pixel(canvas_x as u32, canvas_y as u32);
            let current_color = new_vec(current_color[0].into(), current_color[1].into(), current_color[2].into());
            let mut should_redraw: bool = false;
            
            'outer: for x in -1..=1 {
                for y in -1..=1 {
                    if x == 0 && y == 0 {continue;}

                    let checking = img.get_pixel((canvas_x + x) as u32, (canvas_y + y) as u32);
                    let checking = new_vec(checking[0].into(), checking[1].into(), checking[2].into());

                    if (current_color - checking).length() > MAX_COLOR_DIFFERENCE {
                        should_redraw = true;
                        break 'outer;
                    }

                }
            }

            if should_redraw {
                let (corrected_x, corrected_y) = topleft_to_middle(&img, canvas_x as u32, canvas_y as u32);

                let draw_color = generate_pixel(&scene,
                    (corrected_x, corrected_y),
                    &rotation,
                    4);
    
                let draw_color = v3_to_rgb(draw_color);
                times += 1;
                img.put_pixel(canvas_x as u32, canvas_y as u32, draw_color);
            }
        }
    }

    let elapsed_time = now.elapsed();
    println!("Rendered in {:.2} Seconds", elapsed_time.as_secs_f32());
    println!("Redrew {} Pixels", times);
    img.save("output.png").unwrap();
}


fn generate_pixel(scene: &Scene, current: (i32, i32), rotation: &Rotation3<f64>, n_samples: i8) -> V3 {
    //divides each pixel into subpixels (n_samples + 1)^2 big
    let mut final_color: V3 = new_vec(0.0, 0.0, 0.0);

    for sample_x in -n_samples/2 ..= n_samples/2 {
        for sample_y in -n_samples/2..= n_samples/2 {
            let viewframe_x: f64 = (current.0) as f64 * (scene.viewframe_width / scene.canvas_width as f64) + sample_x as f64 * (scene.viewframe_width / (scene.canvas_width as f64 * n_samples as f64));
            let viewframe_y: f64 = (current.1) as f64 * (scene.viewframe_height / scene.canvas_height as f64) + sample_y as f64 * (scene.viewframe_height / (scene.canvas_height as f64 * n_samples as f64));
            
            let na_ray = vector![viewframe_x, viewframe_y, scene.viewframe_distance]; //convert to a vector that nalgebra can use
            let rotated_ray = rotation.mul(na_ray);
            //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

            let ray = new_ray(scene.camera, new_vec(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
            let draw_color = trace_ray(&scene, ray, EPSILON, INF, 3);

            let divisions_in_pixel = match n_samples {
                1 => 1,
                _ => (n_samples + 1).pow(2),
            };

            final_color = final_color + (draw_color / divisions_in_pixel as f64);
        }
    }

    final_color
}

fn init_scene(scene: &mut Scene) {
    scene.canvas_width = 1920;
    scene.canvas_height = 1080;
    
    scene.viewframe_width = 2.0;
    scene.viewframe_height = 1.125;
    scene.viewframe_distance = 1.0;

    scene.camera = new_vec(0.0, 0.0, 0.0);
    
    scene.primitives.push(new_sphere(
        new_vec(0.0, -1.0, 3.0),
        1.0,
        new_vec(255.0, 0.0, 0.0),
        500.0,
        0.0));
    
    scene.primitives.push(new_sphere(
        new_vec(2.0, 0.0, 4.0),
        1.0,
        new_vec(0.0, 0.0, 255.0),
        500.0,
        0.0));
    
    scene.primitives.push(new_sphere(
        new_vec(-2.0, 0.0, 4.0),
        1.0,
        new_vec(0.0, 255.0, 0.0),
        10.0,
        0.0));

    scene.primitives.push(new_sphere(
        new_vec(0.0, 3.0, 8.0),
        2.0,
        new_vec(255.0 ,0.0,255.0),
        500.0,
        1.0));

    scene.primitives.push(new_sphere(
        new_vec(0.0, -5001.0, 0.0),
        5000.0,
        new_vec(255.0, 255.0, 0.0),
        1000.0,
        0.0));

    scene.ambient_light = 0.2;
    scene.lights.push(Light::Point((new_vec(-10.0,10.0, -10.0), 2.0)));
    scene.lights.push(Light::Directional((new_vec(1.0, 4.0, 4.0), 0.2)));
}