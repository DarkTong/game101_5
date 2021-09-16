use crate::Bounds3::{Bounds3, Union};
use std::cell::RefCell;
use crate::Object::ObjectTrait;
use crate::Scene::AddObject;
use crate::Ray::Ray;
use crate::Intersection::Intersection;
use std::time::SystemTime;

pub struct BVHBuildNode<'a> {
    pub bounds: Bounds3,
    pub left: Option<Box<BVHBuildNode<'a>>>,
    pub right: Option<Box<BVHBuildNode<'a>>>,
    pub object: Option<&'a Box<dyn ObjectTrait>>

    pub split_axis: i32,
    pub first_prim_offset: i32,
    pub n_primitives: i32,
}

struct BVHPrimitiveInfo {

}

pub enum SplitMethod {
    NAIVE,
    SAH
}

pub struct BVHAccel<'a> {
    pub max_prims_in_node: i32,
    pub split_method: SplitMethod,
    pub primitives: Vec<&'a Box<dyn ObjectTrait>>,
    pub root: Option<Box<BVHBuildNode<'a>>>,
}

impl BVHBuildNode {
    pub fn new() -> BVHBuildNode {
        BVHBuildNode{
            bounds: Bounds3::default(),
            left: None,
            right: None,
            object: None,
            split_axis: 0,
            first_prim_offset: 0,
            n_primitives: 0,
        }
    }
}

impl BVHAccel {
    pub fn new(p: Vec<&Box<dyn ObjectTrait>>,
                        max_prims_in_node: Option<i32>,
                        split_method: Option<SplitMethod>) -> BVHAccel{
        let primitives = p;
        let max_prims_in_node = max_prims_in_node.unwrap_or(1).min(255);
        let split_method = split_method.unwrap_or(SplitMethod::NAIVE);

        let start = SystemTime::now();

        let root = if primitives.is_empty() {
            None
        } else {
            BVHAccel::recursive_build(primitives.clone())
        };

        let stop = SystemTime::now();

        let diff = stop.duration_since(start).unwrap().as_secs();
        let hrs = diff / 3600;
        let mins = diff / 60 - hrs * 60;
        let secs = diff - mins * 60 - hrs * 3600;

        print!("\rBVH Generation complete: \nTime Taken: {} hrs, {} mins, {} secs\n\n", hrs, mins, secs);

        BVHAccel {
            max_prims_in_node,
            split_method,
            primitives,
            root
        }
    }

    pub fn world_bound() -> Bounds3 {

    }

    pub fn intersect(&self, ray: &Ray) -> Intersection {
        return if let Some(&node) = self.root {
            BVHAccel::get_intersection(node, ray)
        }
        else {
            Intersection::new()
        }
    }

    pub fn get_intersection(node: &Box<BVHBuildNode>, ray: &Ray) -> Intersection {
        if node.bounds.intersect_p(ray) {
            let left = if Some(_node) = node.left {
                BVHAccel::get_intersection(_node, ray)
            } else { Intersection::default() };
            let right = if Some(_node) = node.right {
                BVHAccel::get_intersection(_node, ray)
            } else { Intersection::default() };

            return if left.distance < right.distance {
                left
            } else {
                right
            }
        }

        return Intersection::default();
    }

    pub fn recursive_build(mut objects: Vec<&Box<dyn ObjectTrait>>) -> Box<BVHBuildNode> {
        let mut node = Box::new(BVHBuildNode::new());

        if objects.len() == 1 {
            node.bounds = objects[0].get_bounds();
            node.object = Some(objects[0]);
            node.left = None;
            node.right = None;
        }
        else if objects.len() == 2 {
            let mut left = Vec::with_capacity(1);
            left.push(objects[0]);
            let mut right = Vec::with_capacity(1);
            right.push(objects[1]);

            node.left = Some(BVHAccel::recursive_build(left));
            node.right = Some(BVHAccel::recursive_build(right));
            node.bounds = Union(&node.left.unwrap().bounds, &node.right.unwrap().bounds);
        }
        else {
            let mut centroid_bounds = Bounds3::default();
            for i in 0..objects.len() {
                centroid_bounds = Union(&centroid_bounds, &objects[i].get_bounds());
            }
            let dim = centroid_bounds.max_extent();
            objects.sort_by(|f1, f2| {
                f1.get_bounds().centroid()[dim].cmp(f2.get_bounds().centroid()[dim])
            });

            let (mut left, mut right) = objects.split_at_mut(objects.len() / 2);

            node.left = Some(BVHAccel::recursive_build(left.to_vec()));
            node.right = Some(BVHAccel::recursive_build(right.to_vec()));
            node.bounds = Union(&node.left.unwrap().bounds, &node.right.unwrap().bounds);
        }

        return node;
    }
}
