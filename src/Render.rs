use crate::Object::ObjectTrait;
use crate::Scene::Scene;
use crate::global::{M_PI, MaterialType};
use std::fs::File;
use std::io::Write;
use std::mem::transmute;
use crate::Material::MaterialType;

pub struct hit_payload<'a> {
    pub hit_obj: &'a Box<dyn ObjectTrait>,
    pub t_near: f32,
    pub index: u32,
    pub uv: glm::Vec2,
}

pub trait RenderTrait {
    fn render(&self, scene: &Scene);
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
    let mut cosi = glm::dot(I, N).clamp(-1.0, 1.0);
    let mut etai = 1.0f32;
    let mut etat = ior;
    let mut n = N.clone();
    if cosi < 0.0 {
        cosi = -cosi;
    }
    else {
        std::mem::swap(&mut etai, &mut etat);
        n = -n;
    }

    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);

    return if k < 0. {
        glm::vec3(0., 0., 0.)
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
    let cosi = glm::dot(I, N).clamp(-1.0, 1.0);
    let mut etai = 1.0f32;
    let mut etat = ior;
    if cosi > 0. {
        std::mem::swap(&mut etai, &mut etat);
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
fn trace<'a>(orig: &glm::Vec3, dir: &glm::Vec3, objects: &'a Vec<Box<dyn ObjectTrait>>) -> Option<hit_payload<'a>> {
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
fn cast_ray(orig: &glm::Vec3, dir: &glm::Vec3, scene: &Scene, depth: i32) -> glm::Vec3 {

    if depth > scene.max_depth {
        return glm::vec3(0., 0., 0.);
    }

    let mut hit_color = scene.background_color.clone();
    if let Some(mut payload) = trace(orig, dir, scene.get_objects()) {
        let hit_point = orig + dir * payload.t_near;
        let mut n = glm::vec3(0., 0., 0.);
        let mut st = glm::vec2(0., 0.);
        payload.hit_obj.get_surface_properties(&hit_point, &dir, payload.index, &mut payload.uv, &mut n, &mut st);
        match payload.hit_obj.get_material_type() {
            MaterialType::REFLECTION_AND_REFRACTION => {
                let reflection_direction = glm::normalize(&reflect(&dir, &n));
                let refraction_direction = glm::normalize(&refract(&dir, &n, payload.hit_obj.get_base().ior));
                let reflection_ray_orig = if glm::dot(&reflection_direction, &n) < 0. {
                    hit_point - n * scene.epsilon
                } else {
                    hit_point + n * scene.epsilon
                };
                let refraction_ray_orig = if glm::dot(&refraction_direction, &n) < 0. {
                    hit_point - n * scene.epsilon
                } else {
                    hit_point + n * scene.epsilon
                };
                let reflection_color = cast_ray(&reflection_ray_orig, &reflection_direction, &scene, depth + 1);
                let refraction_color = cast_ray(&refraction_ray_orig, &refraction_direction, &scene, depth + 1);
                let kr = fresnel(&dir, &n, payload.hit_obj.get_base().ior);
                hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
            }
            MaterialType::REFLECTION => {
                let kr = fresnel(&dir, &n, payload.hit_obj.get_base().ior);
                let reflection_direction = glm::normalize(&reflect(&dir, &n));
                let reflection_ray_orig = if glm::dot(&reflection_direction, &n) < 0. {
                    hit_point + n * scene.epsilon
                } else {
                    hit_point - n * scene.epsilon
                };
                let reflection_color = cast_ray(&reflection_ray_orig, &reflection_direction, &scene, depth + 1);
                hit_color = reflection_color;
            }
            MaterialType::DIFFUSE_AND_GLOSSY => {
                // [comment]
                // We use the Phong illumation model int the default case. The phong model
                // is composed of a diffuse and a specular reflection component.
                // [/comment]
                let mut light_amt = glm::vec3(0., 0., 0.);
                let mut specular_color = glm::vec3(0., 0., 0.);
                let shadow_point_orig = if glm::dot(&dir, &n) < 0. {
                    hit_point + n * scene.epsilon
                } else {
                    hit_point - n * scene.epsilon
                };

                // [comment]
                // Loop over all lights in the scene and sum their contribution up
                // We also apply the lambert cosine law
                // [/comment]
                for light in scene.get_lights() {
                    let light_dir = light.position - hit_point;
                    let light_distance2 = glm::dot(&light_dir, &light_dir);
                    let light_dir = light_dir.normalize();
                    let LdotN = glm::dot(&light_dir, &n).max(0.0f32);
                    let shadow_res = trace(&shadow_point_orig, &light_dir, scene.get_objects());
                    let in_shadow = match shadow_res {
                        Some(v) => v.t_near * v.t_near < light_distance2,
                        _ => false,
                    };

                    light_amt += if in_shadow {
                        glm::vec3(0., 0., 0.)
                    } else {
                        light.intensity * LdotN
                    };
                    let reflection_direction = reflect(&-light_dir, &n);
                    specular_color += (-glm::dot(&reflection_direction, &dir))
                        .max(0.)
                        .powf(payload.hit_obj.get_base().specular_exponent) * light.intensity;
                }

                hit_color =
                    glm::matrix_comp_mult(&light_amt, &payload.hit_obj.eval_diffuse_color(&st))
                        * payload.hit_obj.get_base().Kd
                    + specular_color * payload.hit_obj.get_base().Ks;
            }
        }

    }
    return hit_color;
}

fn output_to_file(path: &String, frame_buffer: &Vec<glm::Vec3>, width: i32, height: i32) {
    let mut u8_d = Vec::<u8>::new();
    u8_d.resize((width * height * 3) as usize, 0);
    for i in 0..(width * height) as usize {
        for j in 0..3 as usize {
            u8_d[i * 3 + j] = (frame_buffer[i][j].clamp(0., 1.) * 255.0) as i32 as u8;
        }
    }
    image::save_buffer(&path, &u8_d, width as u32, height as u32, image::ColorType::Rgb8);
    // // write to file
    // let mut fp = File::create(path).unwrap();
    // let head_str = format!("P6\n{} {}\n255\n", width, height);
    // fp.write(head_str.as_bytes()).unwrap();
    // for i in 0..(width * height) as usize{
    //     let mut color3 = [0u8;3];
    //     for j in 0..3 as usize {
    //         color3[j] = (frame_buffer[i][j].clamp(0., 1.) * 255.0) as i32 as u8;
    //     }
    //     fp.write(&color3);
    // }
}

impl RenderTrait for Renderer {
    // [comment]
    // The main render function. This where we iterate over all pixels in the image, generate
    // primary rays and cast these rays into the scene. The content of the framebuffer is
    // saved to a file.
    // [/comment]
    fn render(&self, scene: &Scene) {
        let mut frame_buffer = Vec::<glm::Vec3>::new();
        frame_buffer.resize((scene.width * scene.height) as usize, glm::zero());

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

                let x = (2.0 / scene.width  as f32 * (i as f32 + 0.5)- 1.0f32) * scale * image_aspect_radio;
                let y = (2.0 / scene.height as f32 * (j as f32 + 0.5)- 1.0f32) * scale * -1.0f32;

                let dir = glm::vec3(x, y, -1.0);
                frame_buffer[m] = cast_ray(&eye_pos, &dir, &scene, 0);
                m = m + 1;
            }
        }

       output_to_file(&"binary.png".to_string(), &frame_buffer, scene.width, scene.height);
    }
}

#[cfg(test)]
mod tests {
    use crate::Render::output_to_file;

    #[test]
    fn write_to_file() {
        let mut frame_buffer = Vec::<glm::Vec3>::with_capacity(4);
        frame_buffer.push(glm::vec3(1., 0., 0.));
        frame_buffer.push(glm::vec3(0., 1., 0.));
        frame_buffer.push(glm::vec3(0., 0., 1.));
        frame_buffer.push(glm::vec3(1., 1., 0.));
        output_to_file(&"test.png".to_string(), &frame_buffer, 2, 2);

        // let d = std::fs::read(&"test.png".to_string()).unwrap();
        // println!("data:{:?}", d);
    }
}
