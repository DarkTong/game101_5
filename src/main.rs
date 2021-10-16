mod global;
mod object;
mod intersection;
mod sphere;
mod triangle;
mod scene;
mod light;
mod render;
mod material;
mod area_light;
mod bounds3;
mod ray;
mod bvh;

extern crate nalgebra_glm as glm;
extern crate image;
extern crate obj as obj_rs;

use object::ObjectTrait;
use render::RenderTrait;

const SCALE     :i32 = 10;
const WIDTH     :i32 = 128i32   * SCALE;
const HEIGHT    :i32 = 96i32    * SCALE;

fn main() {
    let mut scene = scene::Scene::new(WIDTH, HEIGHT);


    let bunny_mat = material::Material::default();
    let bunny_data = global::load_mesh("../res/models/bunny.obj".to_string()).unwrap();
    let mut bunny_obj = 
        triangle::MeshTriangle::new(
            bunny_data.0, 
            bunny_data.1, 
            bunny_data.2, 
            &bunny_mat
        );

    scene.add_object(&bunny_obj as &dyn ObjectTrait);


    let l1 = light::Light {
        position: glm::vec3(-20., 70., 20.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    let l2 = light::Light {
        position: glm::vec3(30., 50., -12.),
        intensity: glm::vec3(0.5, 0.5, 0.5),
    };
    scene.add_light(l1);
    scene.add_light(l2);

    let r = render::Renderer{};
    r.render(&scene);
}
