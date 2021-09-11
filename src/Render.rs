use crate::Object::ObjectTrait;
use crate::Scene::Scene;
use crate::global::M_PI;

pub struct hit_payload<'a> {
    pub t_near: f32,
    pub index: u32,
    pub uv: glm::Vec2,
    pub hit_obj: &'a dyn ObjectTrait
}

pub trait RenderTrait {
    fn render(scene: &Scene);
}

pub struct Renderer;

fn deg_2_rad(deg: f32) -> f32 {
    return deg * M_PI / 180.0;
}

fn _reflect(I: &glm::Vec3, N: &glm::Vec3) -> glm::Vec3{
   return I - 2.0 * glm::dot(I, N) * N;
}

// [comment]
// Compute refraction direction using Snell's law
//
// We need to handle with care the two possible situations:
//
//    - When the ray is inside the object
//
//    - When the ray is outside.
//
// If the ray is outside, you need to make cosi positive cosi = -N.I
//
// If the ray is inside, you need to invert the refractive indices and negate the normal N
// [/comment]

fn reflect(I: &glm::Vec3, N: &glm::Vec3, ior: f32) -> glm::Vec3 {
    let mut cosi = f32::clamp(-1.0, 1.0, glm::dot(I, N));
    let mut etai = 1.0f32;
    let mut etat = ior;
    let mut n = N.clone();
    if cosi < 0.0 {
        cosi = -cosi;
    }
    else {
        let _1 = etai;
        etai = etat;
        etat = etai;
        n = -n;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1 - cosi * cosi);

    return if k < 0. {
        glm::Vec3(0., 0., 0.)
    } else {
        eta * I + (eta * cosi - k.sqrt()) * n
    }
}



impl RenderTrait for Renderer {
    fn render(scene: &Scene) {
        todo!()
    }
}
