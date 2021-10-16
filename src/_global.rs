
use rand::Rng;

pub const M_PI:f32 = 3.14159265358979323846;

pub const EPSILON: f32 = 0.00001;

pub fn solve_quadratic(a:f32, b:f32, c:f32, x0:&mut f32, x1:&mut f32) -> bool{
    let discr = b*b-4.0*a*c; // 判别式
    if discr < 0.0 {
        return false;
    }
    else if discr == 0.0 {
        *x1 = -0.5 * b / a;
        *x0 = *x1;
    }
    else {
        let q = match b > 0. {
            true    => -0.5 * (b + discr.sqrt()),
            false   => -0.5 * (b - discr.sqrt())
        };
        *x0 = q / a;
        *x1 = c / q;
    }
    if x0 > x1 {
        let _t = *x0;
        *x0 = *x1;
        *x1 = _t;
    }

    return true;
}

pub fn get_random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0..1.0f32);
}

pub fn update_progress(progress: f32){
    const bar_width :usize = 70;
    let pos = (bar_width as f32 * progress) as usize;
    print!("[");
    for i in 0..bar_width {
        if i < pos {
            print!("=");
        }
        else if i == pos {
            print!(">");
        }
        else {
            print!(" ");
        }
    }
    println!("] {}%", (progress * 100.0) as i32);
}

pub fn deg_2_rad(deg: f32) -> f32 {
    return deg * M_PI / 180.0;
}

pub fn reflect(I: &glm::Vec3, N: &glm::Vec3) -> glm::Vec3{
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
pub fn refract(I: &glm::Vec3, N: &glm::Vec3, ior: f32) -> glm::Vec3 {
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
pub fn fresnel(I: &glm::Vec3, N: &glm::Vec3, ior: f32) -> f32{
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


pub fn load_mesh(path: String) 
    -> obj_rs::ObjResult<(
        Vec<glm::Vec3>, Vec<glm::Vec2>, 
        Vec<u32>
    )> 
{
    use obj_rs::*;

    let _file = std::fs::File::open(path)?;
    let read_buf = std::io::BufReader::new(_file);

    let wf_obj: Obj<TexturedVertex> = load_obj(read_buf)?;

    let mut vertices = Vec::with_capacity(wf_obj.vertices.len());
    let mut sts = Vec::with_capacity(wf_obj.vertices.len());

    for wf_v in &wf_obj.vertices {
        let pos = glm::vec3(wf_v.position[0], wf_v.position[1], wf_v.position[2]);
        let uv = glm::vec2(wf_v.texture[0], wf_v.texture[1]);
        vertices.push(pos);
        sts.push(uv);
    }

    let mut indices = Vec::with_capacity(wf_obj.indices.len());
    for i in 0..wf_obj.indices.len() {
        indices.push(wf_obj.indices[i] as u32);
    }

    return Ok((vertices, sts, indices))
}
#[cfg(test)]
mod tests {
    use crate::_global::update_progress;

    #[test]
    fn test_update_progress() {
        update_progress(0.5);
    }
}
