
use crate::Object::*;
use std::panic::panic_any;
use crate::global::MaterialType;
use crate::Material::Material;
use std::default::default;
use crate::Bounds3::Bounds3;
use crate::Ray::Ray;
use crate::Intersection::Intersection;

fn ray_triangle_intersect(v0: &glm::Vec3, v1: &glm::Vec3,
                          v2: &glm::Vec3, orig: &glm::Vec3,
                          dir: &glm::Vec3, tnear: &mut f32, u: &mut f32, v: &mut f32) -> bool {
    let e1 = v1 - v0;
    let e2 = v2 - v0;
    let s = orig - v0;
    let s1 = glm::cross(dir, &e2);
    let s2 = glm::cross(&s, &e1);

    let r_m00 = glm::dot(&s2, &e2);
    let r_m01 = glm::dot(&s1, &s);
    let r_m02 = glm::dot(&s2, dir);

    let l = 1.0 / (glm::dot(&s1, &e1));

    let t  = l * r_m00;
    let b1 = l * r_m01;
    let b2 = l * r_m02;

    if  t >= 0. && b1 >= 0. && b2 >= 0. && (1.-b1-b2) >= 0.{
        *tnear = t;
        *u = b1;
        *v = b2;
        return true;
    }
    return false;
}

pub struct Triangle {
    pub v0: glm::Vec3,
    pub v1: glm::Vec3,
    pub v2: glm::Vec3,
    pub e0: glm::Vec3,
    pub e1: glm::Vec3,
    pub t0: glm::Vec3,
    pub t1: glm::Vec3,
    pub t2: glm::Vec3,
    pub normal: glm::Vec3,
}

pub struct MeshTriangle {
    pub bounding_box: Bounds3,
    pub num_triangles: u32,
    pub vertices: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
    pub st_coordinates: Vec<glm::Vec2>,
    pub triangles: Vec<Triangle>,
    pub m: Box<Material>
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            v0: _v0.clone(),
            v1: _v1.clone(),
            v2: _v2.clone(),
            e0: _v1 - _v0,
            e1: _v2 - _v0,
            ..default::Default()
        }
    }
}

impl MeshTriangle {
    pub fn new(vertices: Vec<glm::Vec3>, indices: Vec<u32>,
               num_triangles: u32, st_coordinates: Vec<glm::Vec2>) -> Self {
        MeshTriangle {
            bounding_box: Bounds3::default(),
            num_triangles,
            vertices,
            indices,
            st_coordinates,
            triangles: Vec::new(),
            m: Box::new(Material::default(None, None, None)),
        }
    }
}

impl ObjectTrait for MeshTriangle {
    fn intersect_check(&self, ray: &Ray) -> bool {
        let mut _1 = 0.;
        let mut _2 = 0;
        return self.intersect_nearest(ray, &mut _1, &mut _2);
    }
    
    fn intersect_nearest(&self, ray: &Ray, t_near: &mut f32, index: &mut u32) -> bool {
        let mut intersect = false;
        let orig = &ray.origin;
        let dir = &ray.direction;

        for k in 0..self.num_triangles as usize {
            let v0 = &self.vertices[self.indices[k * 3 + 0] as usize];
            let v1 = &self.vertices[self.indices[k * 3 + 1] as usize];
            let v2 = &self.vertices[self.indices[k * 3 + 2] as usize];

            let mut t = 0.;
            let mut u = 0.;
            let mut v = 0.;
            if ray_triangle_intersect(v0, v1, v2, orig, dir, &mut t, &mut u, &mut v) && t < *tnear{
                *tnear = t;
                uv.x = u;
                uv.y = v;
                *index = k as u32;
                intersect = true
            }
        }

        return intersect;
    }

    fn get_intersection(&self, ray: &Ray) -> Intersection {
        todo!()
    }

    fn get_surface_properties(&self, _: &glm::Vec3, _: &glm::Vec3, index: &u32, uv: &glm::Vec2, n: &mut glm::Vec3, st: &mut glm::Vec2) {
        let v0 = &self.vertices[self.indices[(index * 3 + 0) as usize] as usize];
        let v1 = &self.vertices[self.indices[(index * 3 + 1) as usize] as usize];
        let v2 = &self.vertices[self.indices[(index * 3 + 2) as usize] as usize];

        let e0 = (v1 - v0).normalize();
        let e1 = (v2 - v1).normalize();
        *n = glm::cross(&e0, &e1).normalize();

        let st0 = &self.st_coordinates[self.indices[(index * 3 + 0) as usize] as usize];
        let st1 = &self.st_coordinates[self.indices[(index * 3 + 1) as usize] as usize];
        let st2 = &self.st_coordinates[self.indices[(index * 3 + 2) as usize] as usize];

        *st = st0 * (1.0 - uv.x - uv.y) + st1 * uv.x + st2 * uv.y;
    }

    fn eval_diffuse_color(&self, st: &glm::Vec2) -> glm::Vec3 {
        let scale = 5.0;
        let pattern = ((st.x * scale).rem_euclid(1.0) > 0.5) ^ ((st.y * scale).rem_euclid(1.0) > 0.5);
        let pattern = pattern as u32 as f32;
        return glm::lerp(&glm::vec3(0.815, 0.235, 0.031), &glm::vec3(0.937, 0.937, 0.231), pattern);
    }
}

#[cfg(test)]
mod tests {
    use crate::Triangle::ray_triangle_intersect;

    #[test]
    fn test_ray_triangle_intersect() {
        let v0 = glm::vec3(1.0, 0.0, 0.0);
        let v1 = glm::vec3(0.0, 1.0, 0.0);
        let v2 = glm::vec3(0.0, 0.0, 1.0);

        let mut t = 0f32;
        let mut u = 0f32;
        let mut v = 0f32;
        let r = ray_triangle_intersect(
            &v0, &v1, &v2,
            &glm::vec3(0., 0., 0.), &glm::vec3(1., 1., 0.).normalize(),
            &mut t, &mut u, &mut v);
        assert!(r && (f32::abs(t - 0.5_f32.sqrt()) <= 0.001), format!("{}:{} {} {}", r, t, u, v));
        let r = ray_triangle_intersect(
            &v0, &v1, &v2,
            &glm::vec3(0., 0., 0.), &glm::vec3(1., 1., 1.).normalize(),
            &mut t, &mut u, &mut v);
        assert!(r && (f32::abs(t - 0.577) <= 0.001), format!("{}:{} {} {}", r, t*t, u, v));
    }
}