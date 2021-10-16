use crate::Material::Material;

pub struct IntersectData<'a> {
    pub coords: glm::Vec3,
    pub distance: f32,
    pub index: u32,
    pub normal: glm::Vec3,
    pub uv: glm::Vec2,
    pub st: glm::Vec2,
    pub m: &'a Material,
    pub eval_diffuse_color: glm::Vec3,
}

// impl<'a> Default for IntersectData<'a> {
//     fn default() -> Self {
//         IntersectData {
//             coords: glm::zero(),
//             distance: 0.,
//             index: -1,
//             normal: glm::vec3(0., 0., 1.),
//             uv: glm::zero(),
//             st: glm::zero(),
//             eval_diffuse_color: glm::zero(),
//         }
//     }
// }
