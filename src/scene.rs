use crate::vec3::*;
use crate::primitives::*;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
    pub ambient_light: f64,
}

#[derive(Debug, Clone)]
pub enum Light {
    Point((V3, f64)),
    Directional((V3, f64)),
}

impl Light {
    pub fn contents(&self) -> (V3, f64) {
        match self {
            Light::Point(t) => (t.0, t.1),
            Light::Directional(t) => (t.0, t.1),
        }
    }
}

pub fn new_scene() -> Scene{
    Scene {
        primitives: Vec::new(),
        lights: Vec::new(),
        ambient_light: 0.0,
    }
}