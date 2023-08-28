use crate::vec3::*;
use crate::ray::*;
use image::Rgb;

#[derive(Debug, Clone)]
pub enum Primitive {
    Sphere(Sphere),
}

#[derive(Debug, Clone)]
pub enum Light {
    Ambient(f64),
    Point(V3, f64),
    Directional(V3, f64),
}

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: V3,
    pub radius: f64,
    pub color: Rgb<u8>,
    pub specular: i32, //larger value is more shiny.
    pub reflective: f32, //value should be between 0 and 1. 1 is a mirror.
}
pub trait Collideable {
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)>;
}

impl Collideable for Sphere {
    /*returns scalar of ray at point(s) where ray collides with the sphere.
    t1 and t2 may be equal, meaning there is only one collision */
    
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)> { 
        let r = self.radius;
        let co = ray.origin - self.center;

        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(co, ray.direction);
        let c = dot(co, co) - r*r;

        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return None;
        }

        let rhs = f64::sqrt(discriminant); //rhs is the righthand side of the quadratic formula
        let two_a = 2.0 * a; //so as to not compute twice

        let t1 = (rhs - b) / two_a;
        let t2 = (-b - rhs) / two_a;

        return Some((t1, t2));
    }
}

impl Collideable for Primitive {
    fn ray_intersection(&self, ray: Ray) -> Option<(f64, f64)> {
        match self {
            Primitive::Sphere(p) => p.ray_intersection(ray)
        }
    }
}

impl Primitive {
    pub fn color(&self) -> Rgb<u8> {
        match self {
            Primitive::Sphere(t) => t.color
        }
    }
}

//reflective value is between 0 and 1
pub fn new_sphere(center: V3, radius: f64, color: Rgb<u8>, specular: i32, reflective: f32) -> Primitive {
    Primitive::Sphere(
        Sphere {
            center: center,
            radius: radius,
            color: color,
            specular: specular,
            reflective: reflective,
        }
    )
}