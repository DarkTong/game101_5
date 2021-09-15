use crate::global::*;
use crate::Ray::Ray;
use crate::Material::MaterialType;
use crate::Intersection::*;
use crate::Bounds3::Bounds3;

pub trait ObjectTrait {
    fn intersect_check(&self, ray: &Ray) -> bool;

    fn intersect_nearest(&self, ray: &Ray, t_near: &mut f32, index: &mut u32) -> bool;

    fn get_intersection(&self, ray: &Ray) -> Intersection;

    fn get_surface_properties(&self, P: &glm::Vec3, I: &glm::Vec3, index: u32, uv: &mut glm::Vec2, N: &mut glm::Vec3, st: &mut glm::Vec2);

    fn eval_diffuse_color(&self, _: &glm::Vec2) -> glm::Vec3;

    fn get_bounds(&self) -> Bounds3;
}
