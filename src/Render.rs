use crate::Object::ObjectTrait;
use crate::Scene::Scene;
use crate::global::M_PI;

pub struct hit_payload<'a> {
    pub hit_obj: &'a Box<dyn ObjectTrait>,
    pub t_near: f32,
    pub index: u32,
    pub uv: glm::Vec2,
}

pub trait RenderTrait {
    fn render(scene: &Scene);
}

pub struct Renderer;

fn deg_2_rad(deg: f32) -> f32 {
    return deg * M_PI / 180.0;
}

fn reflect(I: &glm::Vec3, N: &glm::Vec3) -> glm::Vec3{
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
// \param ior: index of refraction 折射率
// [/comment]

fn refract(I: &glm::Vec3, N: &glm::Vec3, ior: f32) -> glm::Vec3 {
    let mut cosi = f32::clamp(-1.0, 1.0, glm::dot(I, N));
    let mut etai = 1.0f32;
    let mut etat = ior;
    let mut n = N.clone();
    if cosi < 0.0 {
        cosi = -cosi;
    }
    else {
        (etai, etat) = (etat, etai);
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

// [comment]
// Compute Fresnel equation
//
// \param I is the incident view direction
//
// \param N is the normal at the intersection point
//
// \param ior is the material refractive index
// [/comment]
fn fresnel(I: &glm::Vec3, N: &glm::Vec3, ior: f32) -> f32{
    let cosi = f32::clamp(-1.0, 1.0, glm::dot(I, N));
    let etai = 1.0f32;
    let etat = ior;
    if cosi > 0. {
        (etai, etat) = (etat, etai);
    }
    let sint = etai / etat * (1.0-cosi*cosi).max(0.0).sqrt();
    if sint >= 1. {
        return 1.;
    }
    else {
        let cost = (1.0-sint*sint).max(0.0).sqrt();
        let cosi = cosi.abs();
        let Rs = ((etat*cosi)-(etai*cost)) / ((etat*cosi)+(etai*cost));
        let Rp = ((etai*cosi)-(etat*cost)) / ((etai*cosi)+(etat*cost));
        return (Rs * Rs + Rp * Rp) / 2.0;
    }
}

// [comment]
// Returns true if the ray intersects an object, false otherwise.
//
// \param orig is the ray origin
// \param dir is the ray direction
// \param objects is the list of objects the scene contains
// \param[out] tNear contains the distance to the cloesest intersected object.
// \param[out] index stores the index of the intersect triangle if the interesected object is a mesh.
// \param[out] uv stores the u and v barycentric coordinates of the intersected point
// \param[out] *hitObject stores the pointer to the intersected object (used to retrieve material information, etc.)
// \param isShadowRay is it a shadow ray. We can return from the function sooner as soon as we have found a hit.
// [/comment]
fn trace(orig: &glm::Vec3, dir: &glm::Vec3, objects: &Vec<Box<ObjectTrait>>) -> Option<hit_payload> {
    let mut t_near = f32::INFINITY;

    let mut payload = None;

    for object in objects {
        let mut t_near_k = f32::INFINITY;
        let mut index_k = 0u32;;
        let mut uv_k = glm::vec2(0., 0.);
        if object.intersect(orig, dir, &mut t_near_k, &mut index_k, &mut uv_k) && t_near_k < t_near{
            payload = Some(hit_payload{
                hit_obj: object,
                t_near: t_near_k,
                index: index_k,
                uv: uv_k,
            });
            t_near = t_near_k;
        }
    }

    return payload;
}

// [comment]
// Implementation of the Whitted-style light transport algorithm (E [S*] (D|G) L)
//
// This function is the function that compute the color at the intersection point
// of a ray defined by a position and a direction. Note that thus function is recursive (it calls itself).
//
// If the material of the intersected object is either reflective or reflective and refractive,
// then we compute the reflection/refraction direction and cast two new rays into the scene
// by calling the castRay() function recursively. When the surface is transparent, we mix
// the reflection and refraction color using the result of the fresnel equations (it computes
// the amount of reflection and refraction depending on the surface normal, incident view direction
// and surface refractive index).
//
// If the surface is diffuse/glossy we use the Phong illumation model to compute the color
// at the intersection point.
// [/comment]
fn cast_ray(orig: &Vec3, dir: &Vec3, scene: &Scene, depth: i32) -> glm::Vec3 {

    return glm::vec3(0., 0., 0.);
}

impl RenderTrait for Renderer {
    // [comment]
    // The main render function. This where we iterate over all pixels in the image, generate
    // primary rays and cast these rays into the scene. The content of the framebuffer is
    // saved to a file.
    // [/comment]
    fn render(scene: &Scene) {
        let mut frame_buffer =
            Vec::<glm::Vec3>::with_capacity((scene.width * scene.height) as usize);

        let scale = deg_2_rad(scene.fov * 0.5).tan();
        let image_aspect_radio = scene.width as f32 / scene.height as f32;
        let eye_pos = glm::vec3(0., 0., 0.);
        let mut m = 0;

        for j in 0..scene.height as usize {
            for i in 0..scene.width as usize {
                // generate primary ray direction
                let x = 0f32;
                let y = 0f32;
                // TODO: Find the x and y positions of the current pixel to get the direction
                // vector that passes through it.
                // Also, don't forget to multiply both of them with the variable *scale*, and
                // x (horizontal) variable with the *imageAspectRatio*

                let dir = glm::vec3(x, y, -1.0);
                m = m + 1;
                frame_buffer[m] = cast_ray(&eye_pos, &dir, &scene, 0);
            }
        }

        // write to file
    }
}
