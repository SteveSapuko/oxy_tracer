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

use nalgebra::{Vector3, Rotation3, vector};
use std::ops::Mul;

fn main() {
<<<<<<< HEAD
    let canvas_width: i32 = 1920 * 7;
    let canvas_height: i32 = 1080 * 7;
=======
    let canvas_width: i32 = 2000;
    let canvas_height: i32 = 2000;
>>>>>>> parent of f416b50 (version 0.1.)

    let viewframe_width: f64 = 1.0;
    let viewframe_height: f64 = 1.0;
    let viewframe_distance: f64 = 1.0;

    let camera = vec3::new(0.0, 0.0, 0.0);
    let ambient_light: f64 = 0.2;

    let rotation: Rotation3<f64> = Rotation3::from_euler_angles(0.0, 0.0, 0.0);
    
    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);   
    let mut img = RgbImage::new(canvas_width as u32, canvas_height as u32);
<<<<<<< HEAD
=======

    //draw_pixel(&mut img, 100, 100, Rgb([255, 255, 255]));
>>>>>>> parent of f416b50 (version 0.1.)

    let mut previous_pixel = vec3::new(0.0, 0.0, 0.0);
    let mut times = 0;
    for image_x in -canvas_width/2 .. canvas_width/2  {
        for image_y in -canvas_height/2 .. canvas_height/2 {

<<<<<<< HEAD
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
    let mut final_color: V3 = vec3::new(0.0, 0.0, 0.0);
    
    if n_samples == 1 {
        let viewframe_x: f64 = current.0 as f64 * (viewframe.0 as f64 / canvas.0 as f64);
        let viewframe_y: f64 = current.1 as f64 * (viewframe.1 as f64 / canvas.1 as f64);
        
        let na_ray = vector![viewframe_x, viewframe_y, viewframe_distance]; //convert to a vector that nalgebra can use
        let rotated_ray = rotation.mul(na_ray);
        //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

        let ray = ray::new(camera, vec3::new(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
        let draw_color = trace_ray(&scene, ray, EPSILON, INF, 3);
        final_color = final_color + (draw_color / (n_samples.pow(2)) as f64);
        return final_color;
    }

    for sample_x in -n_samples/2 .. n_samples/2 {
        for sample_y in -n_samples/2.. n_samples/2 {
            let viewframe_x: f64 = (sample_x as i32 + current.0) as f64 * (viewframe.0 * n_samples as f64 / (n_samples as i32 * canvas.0) as f64);
            let viewframe_y: f64 = (sample_y as i32 + current.1) as f64 * (viewframe.1 * n_samples as f64 / (n_samples as i32 * canvas.1) as f64);
            
            let na_ray = vector![viewframe_x, viewframe_y, viewframe_distance]; //convert to a vector that nalgebra can use
=======
            let na_ray = vector![viewframe_x, viewframe_y, viewframe_distance];
>>>>>>> parent of f416b50 (version 0.1.)
            let rotated_ray = rotation.mul(na_ray);

            let ray = ray::new(camera, vec3::new(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
<<<<<<< HEAD
            let draw_color = trace_ray(&scene, ray, EPSILON, INF, 3);
            final_color = final_color + (draw_color / (n_samples.pow(2)) as f64);
=======
            
            

            let draw_color: Rgb<u8> = match ray.closest_point(&scene.primitives, 1.0, INF) { //returns first point ray intersects, and clone of the primitive it belongs to
                Some(temp) => {
                    let point: V3 = temp.0;
                    let primitive = temp.1;
                    let normal = primitive.normal_at_point(point);

                    let lighting_factor = compute_lighting(&scene, point, normal, ambient_light);

                    let r = (primitive.color()[0] as f64 * lighting_factor).clamp(0.0, 255.0); //multiply by lighting factor and then turn back to u8
                    let g = (primitive.color()[1] as f64 * lighting_factor).clamp(0.0, 255.0);
                    let b = (primitive.color()[2] as f64 * lighting_factor).clamp(0.0, 255.0);

                    Rgb([r as u8, g as u8, b as u8])
                },
                None => Rgb([255, 255, 255])
            };
            
            draw_pixel(&mut img, image_x, image_y, draw_color);
>>>>>>> parent of f416b50 (version 0.1.)
        }
        
    }
    
    final_color
}

fn compute_lighting(scene: &Scene, point: V3, normal: V3, ambient: f64) -> f64 {
    let mut lighting_factor = ambient; //light strength at given point
    for light in scene.lights.iter() {
        let light_dir: V3;

        match *light {
            Light::Point(temp) => light_dir = temp.0 - point, //temp.0 is the V3 component of a light
            Light::Directional(temp) => light_dir = temp.0,
        }

        let n_dot_l = dot(normal, light_dir);
        if n_dot_l > 0.0 {
            lighting_factor += light.contents().1 * n_dot_l / (normal.length() * light_dir.length());
        }
    }

    //println!("{}", lighting_factor);
    lighting_factor
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
<<<<<<< HEAD
        vec3::new(0.0, 0.0, 255.0),
        500.0,
        1.0));
=======
        Rgb([0, 0, 255]),
        0,
        0.0));
>>>>>>> parent of f416b50 (version 0.1.)
    
    scene.primitives.push(new_sphere(
        vec3::new(-2.0, 0.0, 4.0),
        1.0,
        Rgb([0, 255, 0]),
        0,
        0.0));

    scene.primitives.push(new_sphere(
        vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        Rgb([255, 255, 0]),
        0,
        0.0));

<<<<<<< HEAD
    scene.ambient_light = 0.2;
    scene.lights.push(Light::Point((vec3::new(-10.0,10.0, -10.0), 2.0)));
=======
    scene.lights.push(Light::Point((vec3::new(2.0, 1.0, 0.0), 0.6)));
>>>>>>> parent of f416b50 (version 0.1.)
    scene.lights.push(Light::Directional((vec3::new(1.0, 4.0, 4.0), 0.2)));
}

fn draw_pixel(img: &mut RgbImage, x: i32, y: i32, draw_color: Rgb<u8>) {
    let corrected_x: i32 = img.width() as i32 / 2 + x;
    let corrected_y: i32 = img.height() as i32 / 2 - y; 

    //println!("Drawing at: x({}), y({})", corrected_x, corrected_y);

    img.put_pixel(corrected_x as u32, (corrected_y - 1) as u32, draw_color);
}