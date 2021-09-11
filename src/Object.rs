use crate::global::*;

pub struct Object {
    pub material_type: MaterialType,
    pub ior :f32, // index of reflect
    pub Kd :f32,
    pub Ks :f32,
    pub diffuse_color: glm::Vec3,
    pub specular_exponent: f32,
}

impl Object {
    pub fn new() -> Object {
        Object {
            material_type: MaterialType::DIFFUSE_AND_GLOSSY,
            ior: 1.3,
            Kd: 0.8,
            Ks: 0.2,
            diffuse_color: glm::vec3(0.2, 0.2, 0.2),
            specular_exponent: 25.0
        }
    }
}

pub trait ObjectTrait {
    fn intersect(&self, _: &glm::Vec3, _: &glm::Vec3, _: &mut f32, _: &mut u32, _: &mut glm::Vec2) -> bool;

    fn get_surface_properties(&self, _: &glm::Vec3, _: &glm::Vec3, _: &u32, _: &glm::Vec2, _: &mut glm::Vec3, _: &mut glm::Vec2);

    fn eval_diffuse_color(&self, _: &glm::Vec2) -> glm::Vec3;
}
