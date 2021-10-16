
pub struct Light {
    pub position: glm::Vec3,
    pub intensity: glm::Vec3,
}

impl Light {
    pub fn new(position: &glm::Vec3, intensity: &glm::Vec3) -> Light{
        Light {
            position : *position,
            intensity : *intensity,
        }
    }
}