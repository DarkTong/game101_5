use crate::bounds3::Bounds3;
use crate::global::*;
use crate::object::ObjectTrait;
use crate::material::{Material};
use crate::intersection::IntersectData;

pub struct Sphere<'a> {
    pub center          : glm::Vec3,
    pub radius          : f32,
    pub radius2         : f32,
    pub m               : &'a Material,
}

impl<'a> Sphere<'a> {
    pub fn new(center: &glm::Vec3, radius: f32, m: &'a Material) -> Self {
        Sphere {
            center: center.clone(),
            radius,
            radius2: radius* radius,
            m,
        }
    }
}

impl<'a> ObjectTrait for Sphere<'a> {
    fn get_intersection(&self, ray: &crate::ray::Ray) -> Option<IntersectData> {
        let L = ray.origin - self.center;
        let a = glm::dot(&ray.direction, &ray.direction);
        let b = 2.0 * glm::dot(&ray.direction, &L);
        let c = glm::dot(&L, &L) - self.radius2;
        let mut t0 = 0.0f32;
        let mut t1 = 0.0f32;
        if !solve_quadratic(a, b, c, &mut t0, &mut t1) {
            return None;
        }
        if t0 < 0.0 { t0 = t1; }
        if t0 < 0.0 { return None;}

        let coords = ray.origin + t0 * ray.direction;


        return Some(IntersectData {
            coords: coords.clone(),
            normal: (coords - self.center).normalize(),
            distance: t0,
            index: u32::MAX,
            m: self.m,
            eval_diffuse_color: self.m.get_color(),
            uv: glm::zero(),
            st: glm::zero(),
        });
    }

    fn get_bounds(&self) -> Bounds3 {
        let _r3 = glm::vec3(self.radius, self.radius, self.radius);
        let p_min = self.center - _r3;
        let p_max = self.center + _r3;

        Bounds3 { p_min, p_max }
    }

}

#[cfg(test)]
mod tests {
    use crate::material;
    use crate::object::ObjectTrait;
    use crate::sphere::Sphere;
    use crate::ray::Ray;

    #[test]
    fn test_sphere_intersect() {
        let mat = material::Material::default();
        let s = Sphere::new(&glm::vec3(1., 1., 1.), 1., &mat);

        let mut t_near = 0f32;
        let mut _1 = 0u32;
        let ray = Ray::new(
            &glm::vec3(0., 0., 0.),
            &glm::vec3(1., 1., 0.).normalize()
        );
        let inter = s.get_intersection(&ray).unwrap();
        assert!(f32::abs(inter.distance - 2.0_f32.sqrt()) < 0.001,  format!("{}", inter.distance));

        let ray = Ray::new(
            &glm::vec3(0., 0., 0.),
            &glm::vec3(1., 1., 1.).normalize(),
        );
        let inter = s.get_intersection(&ray).unwrap();
        assert!(f32::abs(inter.distance - (3.0_f32.sqrt() - 1.0)) < 0.001, format!("{}", inter.distance));
    }
}

