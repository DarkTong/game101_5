
use crate::Object::*;
use crate::global::*;
use std::boxed::Box;
use crate::Material;
use crate::Ray::Ray;
use crate::Intersection::Intersection;
use crate::Bounds3::Bounds3;

pub struct Sphere {
    pub center: glm::Vec3,
    pub radius: f32,
    pub radius2: f32,
    pub m: Box<Material>
}

impl Sphere {
    pub fn new(c: &glm::Vec3, r: f32) -> Sphere {
        Sphere {
            center: c.clone(),
            radius: r,
            radius2: r*r,
            m: Material::new(None, None, None),
        }
    }
}

impl ObjectTrait for Sphere {
    fn intersect_check(&self, ray: &Ray) -> bool {
        let mut t_near = 0.f32;
        let mut index = 0u32;
        return self.intersect_nearest(ray, &mut t_near, &mut index);
    }

    fn intersect_nearest(&self, ray: &Ray, t_near: &mut f32, index: &mut u32) -> bool {
        let L = ray.origin - self.center;
        let a = glm::dot(&ray.direction, &ray.direction);
        let b = 2.0 * glm::dot(&ray.direction, &L);
        let c = glm::dot(&L, &L) - self.radius2;
        let mut t0 = 0.0f32;
        let mut t1 = 0.0f32;
        if !solve_quadratic(a, b, c, &mut t0, &mut t1) {
            return false;
        }
        if t0 < 0.0 {
            t0 = t1;
        }
        if t0 < 0.0 {
            return false;
        }
        *t_near = t0;
        return true;
    }

    fn get_intersection(&self, ray: &Ray) -> Intersection {
        let mut result = Intersection::new();
        result.happend = false;

        let L = ray.origin - self.center;
        let a = glm::dot(&ray.direction, &ray.direction);
        let b = 2.0 * glm::dot(&ray.direction, &L);
        let c = glm::dot(&L, &L) - self.radius2;
        let mut t0 = 0.0f32;
        let mut t1 = 0.0f32;
        if !solve_quadratic(a, b, c, &mut t0, &mut t1) {
            return result;
        }

        result.happend = true;
        result.coords = ray.get_pos(t0);
        result.normal = (result.coords - self.center).normalize();
        result.m = Some(&self.m);
        result.obj = Some(&self);
        result.distance = t0;

        return result;
    }

    fn get_surface_properties(&self, P: &glm::Vec3, I: &glm::Vec3, index: u32, uv: &mut glm::Vec2, N: &mut glm::Vec3, st: &mut glm::Vec2) {
        *n = (p - self.center).normalize();
    }

    fn eval_diffuse_color(&self, _: &glm::Vec2) -> glm::Vec3 {
        return self.object.diffuse_color;
    }

    fn get_bounds(&self) -> Bounds3 {
        let r = self.radius;
        let r3 = glm::vec3(r, r, r);
        Bounds3 {
            p_min: self.center - r,
            p_max: self.center + r,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sphere::Sphere;
    use crate::Object::ObjectTrait;

    #[test]
    fn test_sphere_intersect() {
        let s = Sphere::new(&glm::vec3(1., 1., 1.), 1.);

        let mut t_near = 0f32;
        let mut _1 = 0u32;
        let mut _2 = glm::vec2(0., 0.);
        let r = s.intersect(&glm::vec3(0., 0., 0.), &glm::vec3(1., 1., 0.).normalize(), &mut t_near, &mut _1, &mut _2);
        assert!(r && f32::abs(t_near - 2.0_f32.sqrt()) < 0.001,  format!("{}:{}", r, t_near));
        let r = s.intersect(&glm::vec3(0., 0., 0.), &glm::vec3(1., 1., 1.).normalize(), &mut t_near, &mut _1, &mut _2);
        assert!(r && f32::abs(t_near - (3.0_f32.sqrt() - 1.0)) < 0.001, format!("{}:{}", r, t_near));
    }
}

