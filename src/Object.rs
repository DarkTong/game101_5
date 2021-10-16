use crate::ray::Ray;
use crate::intersection::IntersectData;
use crate::bounds3::Bounds3;

pub trait ObjectTrait {
    fn get_intersection(&self, ray: &Ray) -> Option<IntersectData>;

    fn get_bounds(&self) -> Bounds3;
}
