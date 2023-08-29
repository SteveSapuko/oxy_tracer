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

fn main() {
    let canvas_width: i32 = 1920 * 2;
    let canvas_height: i32 = 1080 * 2;

    let viewframe_width: f64 = 2.0;
    let viewframe_height: f64 = 1.125;
    let viewframe_distance: f64 = 1.0;

    let camera = new_vec(0.0, 0.0, 0.0);

    let rotation: Rotation3<f64> = Rotation3::from_euler_angles(0.0, 0.0, 0.0);
    
    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);   
    let mut img = RgbImage::new(canvas_width as u32, canvas_height as u32);

    let mut previous_pixel = new_vec(0.0, 0.0, 0.0);
    let mut times = 0;
    for image_x in -canvas_width/2 .. canvas_width/2  {
        for image_y in -canvas_height/2 .. canvas_height/2 {

            let mut draw_color = generate_pixel(&scene,
                (image_x, image_y),
                (viewframe_width, viewframe_height),
                (canvas_width, canvas_height),
                viewframe_distance,
                camera,
                &rotation,
                2);
            
            if (draw_color - previous_pixel).length() > 100.0{
                draw_color = (draw_color + generate_pixel(&scene, (image_x, image_y), (viewframe_width, viewframe_height), (canvas_width, canvas_height), viewframe_distance, camera, &rotation,
                4)) / 2.0;
                times += 1;
            }

            previous_pixel = draw_color;
            let draw_color = v3_to_rgb(draw_color);
            draw_pixel(&mut img, image_x, image_y, draw_color);
        }
    }

    println!("{}", times);
    img.save("output.png").unwrap();
}

fn generate_pixel(scene: &Scene, current: (i32, i32), viewframe: (f64, f64), canvas: (i32, i32), viewframe_distance: f64, camera: V3, rotation: &Rotation3<f64>, n_samples: i8) -> V3 {
    let mut final_color: V3 = new_vec(0.0, 0.0, 0.0);
    
    if n_samples == 1 {
        let viewframe_x: f64 = current.0 as f64 * (viewframe.0 as f64 / canvas.0 as f64);
        let viewframe_y: f64 = current.1 as f64 * (viewframe.1 as f64 / canvas.1 as f64);
        
        let na_ray = vector![viewframe_x, viewframe_y, viewframe_distance]; //convert to a vector that nalgebra can use
        let rotated_ray = rotation.mul(na_ray);
        //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

        let ray = new_ray(camera, new_vec(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
        let draw_color = ray::trace_ray(&scene, ray, EPSILON, INF, 3);
        final_color = final_color + (draw_color / (n_samples.pow(2)) as f64);
        return final_color;
    }

    for sample_x in -n_samples/2 .. n_samples/2 {
        for sample_y in -n_samples/2.. n_samples/2 {
            let viewframe_x: f64 = (sample_x as i32 + current.0) as f64 * (viewframe.0 * n_samples as f64 / (n_samples as i32 * canvas.0) as f64);
            let viewframe_y: f64 = (sample_y as i32 + current.1) as f64 * (viewframe.1 * n_samples as f64 / (n_samples as i32 * canvas.1) as f64);
            
            let na_ray = vector![viewframe_x, viewframe_y, viewframe_distance]; //convert to a vector that nalgebra can use
            let rotated_ray = rotation.mul(na_ray);
            //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

            let ray = new_ray(camera, new_vec(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
            let draw_color = trace_ray(&scene, ray, EPSILON, INF, 3);
            final_color = final_color + (draw_color / (n_samples.pow(2)) as f64);
        }
        
    }
    
    final_color
}

fn init_scene(scene: &mut Scene) {
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
        1.0));
    
    scene.primitives.push(new_sphere(
        new_vec(-2.0, 0.0, 4.0),
        1.0,
        new_vec(0.0, 255.0, 0.0),
        10.0,
        0.0));

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