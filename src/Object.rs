use crate::Ray::Ray;
use crate::Intersection::IntersectData;
use crate::Bounds3::Bounds3;

pub trait ObjectTrait {
    fn get_intersection(&self, ray: &Ray) -> Option<IntersectData>;

    fn get_bounds(&self) -> Bounds3;
}
