
pub struct Ray {
    pub origin: glm::Vec3,
    pub direction: glm::Vec3,
    pub direction_inv: glm::Vec3,
    pub t: f32,
    pub t_min: f32,
    pub t_max: f32,
}

impl Ray {
    pub fn new(ori: &glm::Vec3, dir: &glm::Vec3, t: Option<f32>) -> Ray{
        Ray {
            origin: ori.clone(),
            direction: dir.clone(),
            direction_inv: 1. / dir,
            t: if let Some(_t) = t {_t} else {0.0},
            t_min: 0.0,
            t_max: f32::INFINITY,
        }
    }

    pub fn get_pos(&self, t: f32) -> glm::Vec3{
        return self.origin + t * self.direction;
    }
}
