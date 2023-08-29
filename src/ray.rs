use crate::vec3::*;
use crate::primitives::*;
use crate::util::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: V3,
    pub direction: V3,
}

impl Ray {
    pub fn point_at_scale(&self, scalar: f64) -> V3{
        self.origin + self.direction * scalar
    }

    pub fn closest_point(&self, scene_primitives: &Vec<Primitive>, t_min: f64, t_max: f64) -> Option<(V3, Primitive)> {
        let mut closest_t: f64 = t_max;
        let mut closest_primitive = None;

        for current in scene_primitives.iter() {
            let (t1, t2) = match current.ray_intersection(*self) {
                Some(p) => p,
                None => {continue;},
            };

            if in_range(t1, t_min, t_max) && t1 < closest_t {
                closest_t = t1;
                closest_primitive = Some(current);
            }
            
            if t1 != t2 && in_range(t2, t_min, t_max) && t2 < closest_t {
                closest_t = t2;
                closest_primitive = Some(current);
            }
        }

        return match closest_primitive {
            None => None,
            Some(p) => Some((self.point_at_scale(closest_t), p.clone()))
        };      
    }
}

pub fn new(origin: V3, direction: V3) -> Ray{
    Ray {
        origin: origin,
        direction: direction,
    }
}

pub fn reflect_ray(ray: V3, surface_normal: V3) -> V3{
    2.0 * surface_normal * dot(surface_normal, ray) - ray
}