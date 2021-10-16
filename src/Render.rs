use crate::Scene::Scene;
use crate::global::*;
use crate::Ray::*;


pub trait RenderTrait {
    fn render(&self, scene: &Scene);
}

pub struct Renderer;


pub fn output_to_file(path: &String, frame_buffer: &Vec<glm::Vec3>, width: i32, height: i32) {
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
                // TODO: Find the x and y positions of the current pixel to get the direction
                // vector that passes through it.
                // Also, don't forget to multiply both of them with the variable *scale*, and
                // x (horizontal) variable with the *imageAspectRatio*
                let x = (2.0 / scene.width  as f32 * (i as f32 + 0.5) - 1.0f32) * scale * image_aspect_radio;
                let y = (2.0 / scene.height as f32 * (j as f32 + 0.5) - 1.0f32) * scale * -1.0f32;

                let dir = glm::vec3(x, y, -1.0).normalize();
                frame_buffer[m] = scene.cast_ray(&Ray::new(&eye_pos, &dir), 0);
                m = m + 1;
            }
        }

       output_to_file(&"binary.png".to_string(), &frame_buffer, scene.width, scene.height);
    }
}

#[cfg(test)]
mod tests {
    use super::output_to_file;

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
