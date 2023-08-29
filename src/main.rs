#![allow(dead_code, unused_imports)]

const INF: f64 = 999999999999.0;
const EPSILON: f64 = 0.0000000001;

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
    let canvas_width: i32 = 1920 * 7;
    let canvas_height: i32 = 1080 * 7;

    let viewframe_width: f64 = 2.0;
    let viewframe_height: f64 = 1.125;
    let viewframe_distance: f64 = 1.0;

    let camera = vec3::new(0.0, 0.0, 0.0);

    let rotation: Rotation3<f64> = Rotation3::from_euler_angles(0.0, 0.0, 0.0);
    
    let mut scene: Scene = scene::new_scene();
    init_scene(&mut scene);   
    let mut img = RgbImage::new(canvas_width as u32, canvas_height as u32);

    let mut previous_pixel = vec3::new(0.0, 0.0, 0.0);
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
            let rotated_ray = rotation.mul(na_ray);
            //after rotation, we'll convert it back to V3 (I should have just used nalgebra from the beginning)

            let ray = ray::new(camera, vec3::new(rotated_ray[0], rotated_ray[1], rotated_ray[2])); //ray from the camera to a physical point on the viewframe
            let draw_color = trace_ray(&scene, ray, EPSILON, INF, 3);
            final_color = final_color + (draw_color / (n_samples.pow(2)) as f64);
        }
        
    }
    
    final_color
}

fn trace_ray(scene: &Scene, ray: Ray, t_min: f64, t_max: f64, recursion_depth: i32) -> V3 {
    let temp = ray.closest_point(&scene.primitives, t_min, t_max);
    if temp.is_none() {
        return vec3::new(255.0, 255.0, 255.0);
    }

    let (point, primitive) = temp.unwrap();
    
    let local_color = primitive.color() * compute_lighting(scene, point, &primitive);

    let current_reflective = primitive.reflective();
    if recursion_depth <= 0 || current_reflective <= 0.0 {
        return local_color;
    }

    let reflected = ray::reflect_ray(-1.0 * ray.direction, primitive.normal_at_point(point));
    let reflected = ray::new(point, reflected);
    let reflected_color = trace_ray(scene, reflected, EPSILON, INF, recursion_depth - 1);

    return local_color * (1.0 - current_reflective) + reflected_color * current_reflective;
}

fn compute_lighting(scene: &Scene, point: V3, primitive: &Primitive) -> f64 {
    let mut lighting_factor = scene.ambient_light; //light strength at given point
    let normal = primitive.normal_at_point(point); 

    for light in scene.lights.iter() {
        let light_dir: V3;
        let t_max: f64; //furthest away to search when checking shadow collision

        match *light {
            Light::Point(temp) => {light_dir = temp.0 - point; t_max = 1.0}, //temp.0 is the V3 component of a light
            Light::Directional(temp) => {light_dir = temp.0; t_max = INF},
        }

        //shadow check
        match ray::new(point, light_dir)
            .closest_point(&scene.primitives, EPSILON, t_max) {
                Some(_) => continue,
                None => ()
            }

        let distance_factor = match *light {
            Light::Point((light_pos, _intensity)) => 1.0 / f64::sqrt((light_pos - point).length()),
            Light::Directional(_) => 1.0,
        };

        //diffuse reflection
        let n_dot_l = dot(normal, light_dir);
        if n_dot_l > 0.0 {
            lighting_factor += distance_factor * light.contents().1 * n_dot_l / (normal.length() * light_dir.length());
        } //light.contents().1 is the same as light.intensity()

        
        //specular reflection
        if primitive.specular() != -1.0 {
            let point_to_camera = -1.0 * point;
            let r = reflect_ray(light_dir, normal);
            let r_dot_v = dot(r, point_to_camera);

            if r_dot_v > 0.0 {
                lighting_factor += light.contents().1 * ((r_dot_v / (r.length() * point_to_camera.length())).powf(primitive.specular()));
            }
        }
    }

    //println!("{}", lighting_factor);
    lighting_factor
}

fn init_scene(scene: &mut Scene) {
    scene.primitives.push(new_sphere(
        vec3::new(0.0, -1.0, 3.0),
        1.0,
        vec3::new(255.0, 0.0, 0.0),
        500.0,
        0.0));
    
    scene.primitives.push(new_sphere(
        vec3::new(2.0, 0.0, 4.0),
        1.0,
        vec3::new(0.0, 0.0, 255.0),
        500.0,
        1.0));
    
    scene.primitives.push(new_sphere(
        vec3::new(-2.0, 0.0, 4.0),
        1.0,
        vec3::new(0.0, 255.0, 0.0),
        10.0,
        0.0));

    scene.primitives.push(new_sphere(
        vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        vec3::new(255.0, 255.0, 0.0),
        1000.0,
        0.0));

    scene.ambient_light = 0.2;
    scene.lights.push(Light::Point((vec3::new(-10.0,10.0, -10.0), 2.0)));
    scene.lights.push(Light::Directional((vec3::new(1.0, 4.0, 4.0), 0.2)));
}

fn draw_pixel(img: &mut RgbImage, x: i32, y: i32, draw_color: Rgb<u8>) {
    let corrected_x: i32 = img.width() as i32 / 2 + x;
    let corrected_y: i32 = img.height() as i32 / 2 - y; 

    //println!("Drawing at: x({}), y({})", corrected_x, corrected_y);

    img.put_pixel(corrected_x as u32, (corrected_y - 1) as u32, draw_color);
}