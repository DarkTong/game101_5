
use crate::Object::*;
use crate::global::solve_quadratic;

pub struct Sphere {
    pub object: Object,
    pub center: glm::Vec3,
    pub radius: f32,
    pub radius2: f32
}

impl Sphere {
    pub fn new(c: &glm::Vec3, r: f32) -> Sphere {
        Sphere {
            object: Object::new(),
            center: c.clone(),
            radius: r,
            radius2: r*r,
        }
    }
}

impl ObjectTrait for Sphere {
    fn intersect(&self, orig: &glm::Vec3, dir: &glm::Vec3, tnear: &mut f32,
                 _: &mut u32, _: &mut glm::Vec2) -> bool {
        let L = orig - self.center;
        let a = glm::dot(&dir, &dir);
        let b = 2.0 * glm::dot(&dir, &L);
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
        *tnear = t0;
        return true;
    }

    fn get_surface_properties(&self, p: &glm::Vec3, _: &glm::Vec3, _: &u32,
                              _: &glm::Vec2, n: &mut glm::Vec3, _: &glm::Vec2) {
        *n = (p - self.center).normalize();
    }

    fn eval_diffuse_color(&self, _: &glm::Vec2) -> glm::Vec3 {
        return glm::zero();
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

