use crate::Object::ObjectTrait;
use crate::Material::*;
use std::ptr::null;

struct Intersection<'a> {
    pub happend: bool,
    pub coords: glm::Vec3,
    pub normal: glm::Vec3,
    pub distance: f64,
    pub obj: Option<&'a Box<dyn ObjectTrait>>,
    pub m: Option<&'a Box<Material>>
}

impl Intersection {
    pub fn new() -> Intersection {
        Intersection {
            happend: false,
            coords: glm::zero(),
            normal: glm::zero(),
            distance: f64::INFINITY,
            obj: None,
            m: None
        }
    }
}
