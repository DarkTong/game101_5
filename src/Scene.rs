use crate::Object::ObjectTrait;
use crate::Light::Light;
use std::boxed::Box;

pub struct Scene {
    pub width: i32,
    pub height: i32,
    pub fov: f32,
    pub background_color: glm::Vec3,
    pub max_depth: i32,
    pub epsilon: f32,

    objects: Vec<Box<dyn ObjectTrait>>,
    lights: Vec<Light>
}

pub trait AddObject {
    fn Add(&mut self, object: Box<dyn ObjectTrait>);
}

pub trait AddLight {
    fn Add(&mut self, light: Light);
}

impl Scene {
    pub fn new(width: i32, height: i32) -> Scene{
        Scene {
            width,
            height,
            fov: 90.0,
            background_color: glm::vec3(0.235294, 0.67451, 0.843137);
            max_depth: 5,
            epsilon: 0.00001,
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn ObjectTrait>> {
        return &self.objects;
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        return &self.lights;
    }

}

impl AddObject for Scene {
    fn Add(&mut self, object: Box<dyn ObjectTrait>) {
        self.objects.push(object);
    }
}

impl AddLight for Scene {
    fn Add(&mut self, light: Light) {
        self.lights.push(light);
    }
}
