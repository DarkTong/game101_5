
#[derive(Copy, Clone)]
pub enum MaterialType{
    INVALID,
    DIFFUSE_AND_GLOSSY,
    REFLECTION_AND_REFRACTION,
    REFLECTION,
}

pub struct Material {
    pub m_type: MaterialType,
    pub m_color: glm::Vec3,
    pub m_emission: glm::Vec3,
    pub ior: f32,
    pub Kd: f32,
    pub Ks: f32,
    pub specular_exponent: f32
}

impl Default for Material {
    fn default() -> Self {
        Material {
            m_type: MaterialType::INVALID,
            m_color: glm::vec3(1.0, 1.0, 1.0),
            m_emission: glm::vec3(1.0, 1.0, 1.0),
            ior: 1.0,
            Kd: 1.0,
            Ks: 1.0,
            specular_exponent: 150.0,
        }
    }
}

impl Material {
    pub fn new(t: Option<MaterialType>, c: Option<glm::Vec3>, e: Option<glm::Vec3>) -> Material {
        Material {
            m_type: if let Some(_t)=t {_t} else {MaterialType::DIFFUSE_AND_GLOSSY},
            m_color: if let Some(_c)=c {_c} else {glm::vec3(1., 1., 1.)},
            m_emission: if let Some(_e)=e {_e} else {glm::vec3(0., 0., 0.)},
            ior: 1.0,
            Kd: 1.0,
            Ks: 1.0,
            specular_exponent: 150.0,
        }
    }

    pub fn get_type(&self) -> MaterialType { return self.m_type;}

    pub fn get_color(&self) -> glm::Vec3 { return self.m_color.clone(); }

    pub fn get_emission(&self) -> glm::Vec3 { return self.m_emission.clone(); }

    pub fn get_color_at(u: f32, v: f32) -> glm::Vec3{
        return glm::zero();
    }
}
