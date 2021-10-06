
use rand::seq::index::IndexVec;

use crate::{Bounds3::Bounds3, Intersection::IntersectData, Material::Material, Object::*};

fn ray_triangle_intersect(v0: &glm::Vec3, v1: &glm::Vec3, v2: &glm::Vec3,
                        orig: &glm::Vec3, dir: &glm::Vec3, tnear: &mut f32, 
                        u: &mut f32, v: &mut f32) -> bool {
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

pub struct Triangle<'a> {
    pub _d          : &'a MeshTriangle,
    pub ind         : u32,
}
impl<'a> Triangle<'a> {
    pub fn new(mesh: &'a MeshTriangle, ind: u32) -> Self{
        Triangle {
            _d: mesh, ind
        }
    }
    
    fn v0(&self) -> &glm::Vec3 { return &self._d.vertices[self.iv0()]; }
    fn v1(&self) -> &glm::Vec3 { return &self._d.vertices[self.iv1()]; }
    fn v2(&self) -> &glm::Vec3 { return &self._d.vertices[self.iv2()]; }
    fn iv0(&self) -> usize { return self._d.indices[self.ind as usize + 0] as usize; }
    fn iv1(&self) -> usize { return self._d.indices[self.ind as usize + 1] as usize; }
    fn iv2(&self) -> usize { return self._d.indices[self.ind as usize + 2] as usize; }

    fn get_surface_properties(&self, P: &nalgebra_glm::Vec3, 
        N: &mut nalgebra_glm::Vec3, 
        st: &mut nalgebra_glm::Vec2) 
    {
        let v0 = self.v0();
        let v1 = self.v1();
        let v2 = self.v2();
        let _d = self._d;

        let e0 = (v1 - v0).normalize();
        let e1 = (v2 - v1).normalize();
        *N = glm::cross(&e0, &e1).normalize();

        let st0 = &_d.st_coordinates[self.iv0()];
        let st1 = &_d.st_coordinates[self.iv1()];
        let st2 = &_d.st_coordinates[self.iv2()];

        *st = st0 * (1.0 - uv.x - uv.y) + st1 * uv.x + st2 * uv.y;
    }


}

impl<'a> ObjectTrait for Triangle<'a> {
    fn get_intersection(&self, ray: &crate::Ray::Ray) -> Option<IntersectData> {
        let mut tnear = 0f32;
        let mut u = 0f32;
        let mut v = 0f32;
        let ok = ray_triangle_intersect(
            self.v0(), self.v1(), self.v2(), 
            &ray.origin, &ray.direction,
            &mut tnear, &mut u, &mut v
        );
        if !ok {
            return None;
        }

        let uv = self._d.
        
        return Some(IntersectData {
            coords: glm::vec3(u, v, 1.0),
            normal: glm::zero(), // todo:
            distance: tnear,
            index: -1,
            m: &self._d.m,
        });
    }

    fn get_bounds(&self) -> crate::Bounds3::Bounds3 {
        let b1 = Bounds3::new(&self.v0(), &self.v1());
        let b2 = Bounds3::new(&self.v0(), &self.v1());
        return Bounds3::intersect(&b1, &b2);
    }
}

pub struct MeshTriangle {
    pub num_triangles: u32,
    pub vertices: Vec<glm::Vec3>,
    pub indices: Vec<u32>,
    pub st_coordinates: Vec<glm::Vec2>,
    pub bounding_box: Bounds3,
    pub m: Material,
}

impl MeshTriangle {
    pub fn new(vertices: Vec<glm::Vec3>, indices: Vec<u32>,
               num_triangles: u32, st_coordinates: Vec<glm::Vec2>) -> MeshTriangle 
    {
        let mut bounding_box = Bounds3::new(&vertices[0], &vertices[1]);   
        for vert in vertices.iter() {
            bounding_box = bounding_box.intersect_p(vert);
        }

        MeshTriangle {
            num_triangles,
            vertices,
            indices,
            st_coordinates,
            bounding_box,
        }
    }

}

impl ObjectTrait for MeshTriangle {
    fn get_surface_properties(&self, P: &nalgebra_glm::Vec3, I: &nalgebra_glm::Vec3, 
        index: &u32, uv: &nalgebra_glm::Vec2, N: &mut nalgebra_glm::Vec3, 
        st: &mut nalgebra_glm::Vec2) 
    {
        let v0 = &self.vertices[self.indices[(index * 3 + 0) as usize] as usize];
        let v1 = &self.vertices[self.indices[(index * 3 + 1) as usize] as usize];
        let v2 = &self.vertices[self.indices[(index * 3 + 2) as usize] as usize];

        let e0 = (v1 - v0).normalize();
        let e1 = (v2 - v1).normalize();
        *N = glm::cross(&e0, &e1).normalize();

        let st0 = &self.st_coordinates[self.indices[(index * 3 + 0) as usize] as usize];
        let st1 = &self.st_coordinates[self.indices[(index * 3 + 1) as usize] as usize];
        let st2 = &self.st_coordinates[self.indices[(index * 3 + 2) as usize] as usize];

        *st = st0 * (1.0 - uv.x - uv.y) + st1 * uv.x + st2 * uv.y;
    }

    fn eval_diffuse_color(&self, st: &glm::Vec2) -> glm::Vec3 {
        let scale = 5.0f32;
        let pattern = ((st.x * scale).rem_euclid(1.0) > 0.5) ^ ((st.y * scale).rem_euclid(1.0) > 0.5);
        let pattern = pattern as u32 as f32;
        return glm::lerp(
            &glm::vec3(0.815, 0.235, 0.031), 
            &glm::vec3(0.937, 0.937, 0.231), 
            pattern
        );
    }

    fn get_bounds(&self) -> Bounds3 {
        return self.bounding_box.clone();
    }

    fn get_intersection(&self, ray: &crate::Ray::Ray) -> Option<IntersectData> {
        todo!()
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