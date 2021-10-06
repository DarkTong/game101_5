use nalgebra_glm::left_handed;

use crate::Intersection::IntersectData;
use crate::Object::ObjectTrait;
use crate::Bounds3::Bounds3;
use crate::Ray::Ray;

pub enum SplitMethod {
    NAIVE,
    SAH,
}

pub struct BVHBuildNode<'a> {
    pub bounds: Bounds3,
    pub left: Option<Box<BVHBuildNode<'a>>>,
    pub right: Option<Box<BVHBuildNode<'a>>>,
    pub object: Option<&'a dyn ObjectTrait>,
}

impl<'a> Default for BVHBuildNode<'a> {
    fn default() -> Self {
        BVHBuildNode {
            bounds: Bounds3::default(),
            left: None,
            right: None,
            object: None,
        }
    }
}

pub struct BVHAccel<'a> {
    pub root: Option<Box<BVHBuildNode<'a>>>,
    pub max_prims_in_node: u32,
    pub split_method: SplitMethod,
    pub primitives: Vec<&'a dyn ObjectTrait>,
}

impl<'a> BVHAccel<'a> {
    pub fn new(
        p: Vec<&'a dyn ObjectTrait>, 
        max_prims_in_node: u32,
        split_method: SplitMethod
    ) -> Self {
        let root = BVHAccel::recursive_build(&mut p[..]);
        BVHAccel {
            root,
            max_prims_in_node,
            split_method,
            primitives: p,
        }
    }

    pub fn recursive_build(objects: &mut [&'a dyn ObjectTrait]) 
        -> Option<Box<BVHBuildNode<'a>>>
    {
        if objects.len() == 0 {
            return None;
        }
        let mut node = Box::new(BVHBuildNode::default());
        let mut bounds = Bounds3::default();
        for obj in objects.iter() {
            bounds = Bounds3::union(&bounds, &obj.get_bounds());
        }
        node.bounds = bounds;

        if objects.len() == 0 {
            node.object = Some(objects[0]);
        }
        else {
            let mut centroid_bounds = Bounds3::default();
            for obj in objects {
                centroid_bounds = centroid_bounds.union_p(&obj.get_bounds().centroid());
            }
            let dim = centroid_bounds.max_extent();
            objects.sort_by(
                |&a, &b| {
                    a.get_bounds().centroid()[dim]
                        .partial_cmp(&b.get_bounds().centroid()[dim])
                        .unwrap()
                }
            );
            let mid = objects.len() / 2;

            node.left  = BVHAccel::recursive_build(&mut objects[0..mid+1]);
            node.right = BVHAccel::recursive_build(&mut objects[mid..]);
        }

        return Some(node)
    }

    pub fn get_intersection(&self, ray: &Ray) -> Option<IntersectData> {
        return self._get_intersection(&self.root, ray);
    }

    fn _get_intersection(&self, node: &Option<Box<BVHBuildNode<'a>>>, ray: &Ray
    ) -> Option<IntersectData> {
        // no node
        if node.is_none() {
            return None;
        }
        let node_data = &node.unwrap();
        // check bound failed
        if !node_data.bounds.intersect_ray(ray) {
            return None;
        }
        // is leaf node
        if node_data.object.is_some() {
            return node_data.object.unwrap().get_intersection(ray);
        }
        // check left node
        let left_data = self._get_intersection(&node_data.left, ray);
        // check right node
        let right_data = self._get_intersection(&node_data.right, ray);

        return if left_data.is_none() {
            right_data
        }
        else if right_data.is_none() {
            left_data
        }
        // get neatest intersection data
        else if left_data.unwrap().distance < right_data.unwrap().distance {
            left_data
        }
        else {
            right_data
        }
    }
}


