use crate::_ray::Ray;
use crate::_intersection::IntersectData;
use crate::_bounds3::Bounds3;

pub trait ObjectTrait {
    fn get_intersection(&self, ray: &Ray) -> Option<IntersectData>;

    fn get_bounds(&self) -> Bounds3;
}
