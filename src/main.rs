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

extern crate nalgebra_glm as glm;
extern crate image;
extern crate obj as obj_rs;

use Object::ObjectTrait;
use Render::RenderTrait;

const SCALE     :i32 = 10;
const WIDTH     :i32 = 128i32   * SCALE;
const HEIGHT    :i32 = 96i32    * SCALE;

fn main() {
    let mut scene = Scene::Scene::new(WIDTH, HEIGHT);


    let bunny_mat = Material::Material::default();
    let bunny_data = global::load_mesh("../res/models/bunny.obj".to_string()).unwrap();
    let mut bunny_obj = 
        Triangle::MeshTriangle::new(
            bunny_data.0, 
            bunny_data.1, 
            bunny_data.2, 
            &bunny_mat
        );

    scene.add_object(&bunny_obj as &dyn ObjectTrait);


    let l1 = Light::Light {
        position: glm::vec3(-20., 70., 20.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    let l2 = Light::Light {
        position: glm::vec3(30., 50., -12.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    scene.add_light(l1);
    scene.add_light(l2);

    let r = Render::Renderer{};
    r.render(&scene);
}
