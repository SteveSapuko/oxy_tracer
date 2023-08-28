use crate::vec3::*;
use crate::primitives::*;

pub struct Scene {
    pub primitives: Vec<Primitive>,
    pub lights: Vec<Light>,
}

pub fn new_scene() -> Scene{
    Scene {
        primitives: Vec::new(),
        lights: Vec::new(),
    }
}