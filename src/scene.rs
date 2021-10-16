use crate::global::*;
use crate::intersection::IntersectData;
use crate::object::ObjectTrait;
use crate::light::Light;
use crate::ray::Ray;
use crate::bvh::{BVHAccel, SplitMethod};
use crate::material::*;
use std::boxed::Box;

pub struct Scene<'a> {
    pub width: i32,
    pub height: i32,
    pub fov: f32,
    pub background_color: glm::Vec3,
    pub max_depth: i32,
    pub bvh: Option<Box<BVHAccel<'a>>>,

    objects: Vec<&'a dyn ObjectTrait>,
    lights: Vec<Light>,
}


impl<'a> Scene<'a> {
    pub fn new(width: i32, height: i32) -> Scene<'a>{
        Scene {
            width,
            height,
            fov: 90.0,
            background_color: glm::vec3(0.235294, 0.67451, 0.843137),
            max_depth: 5,
            objects: Vec::new(),
            lights: Vec::new(),
            bvh: None,
        }
    }

    pub fn get_objects(&self) -> &Vec<&'a dyn ObjectTrait> {
        return &self.objects;
    }

    pub fn get_lights(&self) -> &Vec<Light> {
        return &self.lights;
    }

    pub fn add_object(&mut self, object: &'a dyn ObjectTrait) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn get_intersect(&self, ray: &Ray) -> Option<IntersectData> {
        return self.bvh.as_ref().unwrap().get_intersection(ray);
    }

    pub fn cast_ray(&self, ray: &Ray, depth: i32
    ) -> glm::Vec3 {
        if depth > self.max_depth {
            return glm::zero();
        }

        let inter_opt = self.get_intersect(ray);
        let mut hit_color = self.background_color.clone();
        if let Some(inter) = inter_opt {
            let mut hit_point = inter.coords;
            let mut n = inter.normal;
            let mut st = inter.st;
            match inter.m.get_type() {
                MaterialType::REFLECTION_AND_REFRACTION => {
                    let reflection_direction 
                        = glm::normalize(&reflect(&ray.direction, &n));
                    let refraction_direction 
                        = glm::normalize(&refract(&ray.direction, &n, inter.m.ior));
                    let reflection_ray_orig 
                        = if glm::dot(&reflection_direction, &n) < 0. {
                            hit_point - n * EPSILON
                        } else {
                            hit_point + n * EPSILON
                        };
                    let refraction_ray_orig 
                        = if glm::dot(&refraction_direction, &n) < 0. {
                            hit_point - n * EPSILON 
                        } else {
                            hit_point + n *EPSILON 
                        };

                    let reflection_color 
                        = self.cast_ray(
                            &Ray::new(&reflection_ray_orig, &reflection_direction), 
                            depth + 1
                        );
                    let refraction_color 
                        = self.cast_ray(
                            &Ray::new(&refraction_ray_orig, &refraction_direction), 
                            depth + 1
                        );
                    let kr = fresnel(&ray.direction, &n, inter.m.ior);
                    hit_color = reflection_color * kr + refraction_color * (1.0 - kr);
                },
                MaterialType::REFLECTION => {
                    let kr = fresnel(&ray.direction, &n, inter.m.ior);

                    let reflection_direction 
                        = glm::normalize(&reflect(&ray.direction, &n));
                    let reflection_ray_orig 
                        = if glm::dot(&reflection_direction, &n) < 0. {
                            hit_point + n * EPSILON 
                        } else {
                            hit_point - n * EPSILON
                        };
                    let reflection_color 
                        = self.cast_ray(
                            &Ray::new(&reflection_ray_orig, &reflection_direction), 
                            depth + 1
                        );
                    hit_color = reflection_color;
                },
                _ => {
                    // [comment]
                    // We use the Phong illumation model int the default case. The phong model
                    // is composed of a diffuse and a specular reflection component.
                    // [/comment]
                    let mut light_amt = glm::vec3(0., 0., 0.);
                    let mut specular_color = glm::vec3(0., 0., 0.);
                    let shadow_point_orig 
                        = if glm::dot(&ray.direction, &n) < 0. {
                            hit_point + n * EPSILON
                        } else {
                            hit_point - n * EPSILON
                        };
                    // [comment]
                    // Loop over all lights in the scene and sum their contribution up
                    // We also apply the lambert cosine law
                    // [/comment]
                    for light in self.get_lights() {
                        let light_dir = light.position - hit_point;
                        let light_distance2 = glm::dot(&light_dir, &light_dir);
                        let light_dir = light_dir.normalize();
                        let LdotN = glm::dot(&light_dir, &n).max(0.0f32);
                        let in_shadow = self.get_intersect(
                            &Ray::new(&shadow_point_orig, &light_dir),
                        ).is_some() as i32 as f32;
                        let _light_amt = (1.0 - in_shadow) * light.intensity * LdotN;
                        light_amt += _light_amt;
                        let reflection_direction = reflect(&-light_dir, &n);
                        specular_color += (-glm::dot(&reflection_direction, &ray.direction))
                            .max(0.)
                            .powf(inter.m.specular_exponent) * light.intensity;
                    }

                    hit_color = 
                        inter.eval_diffuse_color * inter.m.Kd + specular_color * inter.m.Ks;
                    hit_color = glm::vec3(
                        hit_color.x * light_amt.x,
                        hit_color.y * light_amt.y,
                        hit_color.z * light_amt.z,
                    );
                }
            }
        }

        return hit_color;
    }

    // private
    fn build_bvh(&mut self) {
        let mut bvh = BVHAccel::new(self.objects.clone(), 1, SplitMethod::NAIVE);
        self.bvh = Some(Box::new(bvh))
    }

}
