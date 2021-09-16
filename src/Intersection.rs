use crate::Object::ObjectTrait;
use crate::Material::*;
use std::ptr::null;

pub struct Intersection<'a> {
    pub happend: bool,
    pub coords: glm::Vec3,
    pub normal: glm::Vec3,
    pub distance: f32,
    pub obj: Option<&'a Box<dyn ObjectTrait>>,
    pub m: Option<&'a Box<Material>>
}

impl Default for Intersection {
    fn default() -> Self {
        Intersection {
            happend: false,
            coords: glm::zero(),
            normal: glm::zero(),
            distance: f32::INFINITY,
            obj: None,
            m: None
        }
    }
}
