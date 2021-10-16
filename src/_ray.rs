
pub struct Ray {
    pub origin          : glm::Vec3,
    pub direction       : glm::Vec3,
    pub direction_inv   : glm::Vec3,
    pub t               : f32,
    pub t_min           : f32,
    pub t_max           : f32,
}

impl Ray {
    pub fn new(origin: &glm::Vec3, direction: &glm::Vec3) -> Self {
        let direction_inv = glm::vec3(
            1.0f32 / direction.x,
            1.0f32 / direction.y,
            1.0f32 / direction.z,
        );
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
            direction_inv,
            t: 0.0,
            t_min: 0.0,
            t_max: f32::MAX,
        }
    }
}
