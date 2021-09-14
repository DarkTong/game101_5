use crate::Light::Light;
use crate::global::get_random_f32;

struct AreaLight {
    pub base: Light,
    pub length: f32,
    pub normal: glm::Vec3,
    pub u: glm::Vec3,
    pub v: glm::Vec3,
}

impl AreaLight {
    pub fn new(p: &glm::Vec3, i: &glm::Vec3) -> AreaLight {
        AreaLight {
            base: Light::new(p, i),
            normal: glm::vec3(0., -1., 0.),
            u: glm::vec3(1., 0., 0.),
            v: glm::vec3(0., 0., 1.),
            length: 100.0
        }
    }

    pub fn sample_point(&self) -> glm::Vec3 {
        let random_u = get_random_f32();
        let random_v = get_random_f32();
        return self.base.position + random_u * self.u + random_v * self.v;
    }
}