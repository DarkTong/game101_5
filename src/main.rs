mod global;
mod Object;
mod Intersection;
mod Sphere;
mod Triangle;
mod Scene;
mod Light;
mod Render;
mod Material;
mod AreaLight;
mod Bounds3;
mod Ray;
mod BVH;
mod bvh;
mod Render;

extern crate nalgebra_glm as glm;
extern crate image;

use crate::Object::ObjectTrait;
use image::error::ImageError::Limits;
use crate::Scene::{AddLight, AddObject};

const SCALE     :i32 = 10;
const WIDTH     :i32 = 128i32   * SCALE;
const HEIGHT    :i32 = 96i32    * SCALE;

fn main() {
    let mut scene = Scene::Scene::new(WIDTH, HEIGHT);

    let mut sph1 = Box::new(
        Sphere::Sphere::new(
            &glm::vec3(-1.0, 0.0, -12.0),
            2.0)
        );
    sph1.object.material_type = MaterialType::DIFFUSE_AND_GLOSSY;
    sph1.object.diffuse_color = glm::vec3(0.6, 0.7, 0.8);

    let mut sph2 = Box::new(
        Sphere::Sphere::new(
            &glm::vec3(0.5, -0.5, -8.0),
            1.5)
    );
    sph2.object.material_type = MaterialType::REFLECTION_AND_REFRACTION;
    sph2.object.ior = 1.5;

    (&mut scene as &mut AddObject).Add(sph1 as Box<dyn ObjectTrait>);
    (&mut scene as &mut AddObject).Add(sph2 as Box<dyn ObjectTrait>);

    let verts = [
        glm::vec3(-5.0f32, -3., -6.),
        glm::vec3(5., -3., -6.),
        glm::vec3(5., -3., -16.),
        glm::vec3(-5., -3.0, -16.),
    ];
    let vert_index = [0, 1, 3, 1, 2, 3];
    let st = [
        glm::vec2(0.0f32, 0.),
        glm::vec2(1., 0.),
        glm::vec2(1., 1.),
        glm::vec2(0., 1.),
    ];
    let mut mesh = Box::new(
       Triangle::MeshTriangle::new(
           verts.to_vec(),
           vert_index.to_vec(),
           2,
           st.to_vec()
       )
    );
    mesh.object.material_type = MaterialType::DIFFUSE_AND_GLOSSY;

    (&mut scene as &mut AddObject).Add(mesh as Box<dyn ObjectTrait>);
    let l1 = Light::Light {
        position: glm::vec3(-20., 70., 20.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    let l2 = Light::Light {
        position: glm::vec3(30., 50., -12.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    (&mut scene as &mut AddLight).Add(l1);
    (&mut scene as &mut AddLight).Add(l2);

    let r = Renderer{};
    r.render(&scene);
}
