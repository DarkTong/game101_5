use std::slice::from_raw_parts_mut;
use crate::Ray::*;

pub struct Bounds3 {
    pub p_min: glm::Vec3,
    pub p_max: glm::Vec3,
}

impl Bounds3 {
    pub fn default() -> Bounds3 {
        let max_f32 = f32::INFINITY;
        let min_f32 = f32::NEG_INFINITY;
        Bounds3 {
            p_min: glm::vec3(max_f32, max_f32, max_f32),
            p_max: glm::vec3(min_f32, min_f32, min_f32),
        }
    }

    pub fn new1(p1: &glm::Vec3, p2: &glm::Vec3) -> Bounds3 {
        let p_min = glm::vec3(
          f32::min(p1.x, p2.x),
          f32::min(p1.y, p2.y),
          f32::min(p1.z, p2.z),
        );
        let p_max = glm::vec3(
            f32::max(p1.x, p2.x),
            f32::max(p1.y, p2.y),
            f32::max(p1.z, p2.z),
        );
        Bounds3 {
            p_min,
            p_max
        }
    }

    pub fn diagonal(&self) -> glm::Vec3 {
        return self.p_max - self.p_min;
    }

    pub fn max_extent(&self) -> i32 {
        let d = self.diagonal();
        return if d.x > d.y && d.x > d.z {
            0
        } else if d.y > d.z {
            1
        } else {
            2
        }
    }

    pub fn surface_area(&self) -> f32 {
        let d = self.diagonal();
        return 2.0 * (d.x*d.y + d.x*d.z + d.y * d.z);
    }

    pub fn centroid(&self) -> glm::Vec3 {
        return 0.5 * self.p_min + 0.5 * self.p_max;
    }

    pub fn intersect(&self, b: &Bounds3) -> Bounds3{
        Bounds3::new1(
            &glm::max2(&self.p_min, &b.p_min),
            &glm::min2(&self.p_max, &b.p_max)
        )
    }

    pub fn offset(&self, p: &glm::Vec3) -> glm::Vec3 {
        let mut o = p - self.p_min;
        for i in 0..3 as usize {
            if self.p_max[i] > self.p_min[i] {
                o.x /= self.p_max[i] - self.p_min[i];
            }
        }
        return o;
    }

    pub fn overlaps(b1: &Bounds3, b2: &Bounds3) -> bool{
        let mut r = [false; 3];
        for i in 0..3usize {
            r[i] = (b1.p_max[i] >= b2.p_max[i]) && (b1.p_min[i] <= b2.p_min[i]);
        }
        return r[0] && r[1] && r[2];
    }

    pub fn inside(p: &glm::Vec3, b: &Bounds3) -> bool{
        return glm::min2(&b.p_min, &p) == b.p_min &&
            glm::max2(&p, &b.p_max) == b.p_max;
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        let _min = (self.p_min - ray.origin) * ray.direction_inv;
        let _max = (self.p_max - ray.origin) * ray.direction_inv;
        let _t = _max - _min;
        if _min.x < 0. || _min.y < 0. || _min.z < 0. ||
            _max.x < 0. || _max.y < 0. || _max.z < 0. ||
            _t.x < 0. || _t.y < 0. || _t.z < 0. {
            return false;
        }
        return true;
    }

}

pub fn Union(b1: &Bounds3, b2: &Bounds3) -> Bounds3{
    let p_min = glm::min2(&b1.p_min, &b2.p_min);
    let p_max = glm::min2(&b1.p_max, &b2.p_max);
    return Bounds3 { p_min, p_max }
}

pub fn Union1(b: &Bounds3, p: &glm::Vec3) -> Bounds3{
    let p_min = glm::min2(&b1.p_min, p);
    let p_max = glm::min2(&b1.p_max, p);
    return Bounds3 { p_min, p_max }
}
