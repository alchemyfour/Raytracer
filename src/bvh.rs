use std::ptr::null;
use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::tri::{Triangle3d, Triangles3d};

enum Primitive {
    Triangle(Triangle3d),
    //sphere
    Box3d(Box3d)
}

#[derive()]
pub struct BVH {
    pub tree: TreeBranch,
    pub primitives: Vec<Triangle3d>, // Properly implement center function later, for now primitives is just Triangle3d
}

#[derive(Clone)]
pub struct BV {
    pub box3d: Box3d,
    pub primitives: Option<Vec<Triangle3d>> // Same as last comment
}

impl BV {
    pub fn new(box3d: Box3d) -> BV {
        BV { box3d, primitives: None }
    }
}

impl BVH {
    pub fn new(tree: TreeBranch, primitives: Vec<Primitive>) -> BVH {
        BVH { tree, primitives: vec![] }
    }

    pub fn build_section(section: BV) -> (BV, BV) {
        // if empty then give up
        let x = section.box3d.xmax - section.box3d.xmin;
        let y = section.box3d.ymax - section.box3d.ymin;
        let z = section.box3d.zmax - section.box3d.zmin;

        // find the bounding box's longest axis
        let (mut bv1, mut bv2, axis) = match (x, y, z) {
            (a, b, c) if a >= b && a >= c => {
                // X is biggest or there's a tie
                let center = (section.box3d.xmin + section.box3d.xmax) / 2.0;
                let box1 = Box3d::new(center, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, center, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                (BV::new(box1), BV::new(box2), 'x')
            }
            (_, b, c) if b >= c => {
                // Y is biggest or tied with Z
                let center = (section.box3d.ymin + section.box3d.ymax) / 2.0;
                let box1 = Box3d::new(section.box3d.xmin, section.box3d.xmax, center, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, center, section.box3d.zmin, section.box3d.zmax);
                (BV::new(box1), BV::new(box2), 'y')
            }
            (_, _, _) => {
                // Z is biggest
                let center = (section.box3d.zmin + section.box3d.zmax) / 2.0;
                let box1 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, center, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, center);
                (BV::new(box1), BV::new(box2), 'z')
            }
        };

        if let Some(ref primitives) = section.primitives {
            let mut list1 = Vec::new();
            let mut list2 = Vec::new();

            let center = match axis {
                'x' => (section.box3d.xmin + section.box3d.xmax) / 2.0,
                'y' => (section.box3d.ymin + section.box3d.ymax) / 2.0,
                'z' => (section.box3d.zmin + section.box3d.zmax) / 2.0,
                _ => unreachable!(),
            };

            for triangle in primitives {
                let val = match axis {
                    'x' => triangle.center().x,
                    'y' => triangle.center().y,
                    'z' => triangle.center().z,
                    _ => unreachable!(),
                };

                if val >= center {
                    list1.push(triangle.clone());
                } else {
                    list2.push(triangle.clone());
                }
            }

            bv1.primitives = Some(list1);
            bv2.primitives = Some(list2);
        }

        (bv1, bv2)
    }

    fn build_node(node: &mut TreeBranch, threshold: usize) {
        let primitives_len = match &node.data.primitives {
            Some(prims) => prims.len(),
            None => 0,
        };

        // Terminate recursion if threshold is met
        if primitives_len <= threshold {
            return;
        }

        // Subdivide
        let (bv1, bv2) = Self::build_section(node.data.clone());

        let mut left_branch = TreeBranch::new(bv2);
        let mut right_branch = TreeBranch::new(bv1);

        // Guard against infinite recursion when no primitives can be partitioned further
        let left_len = left_branch.data.primitives.as_ref().map_or(0, |p| p.len());
        let right_len = right_branch.data.primitives.as_ref().map_or(0, |p| p.len());
        if left_len == primitives_len || right_len == primitives_len {
            return;
        }

        // Recursively build children
        Self::build_node(&mut left_branch, threshold);
        Self::build_node(&mut right_branch, threshold);

        // Link parent to children
        node.left = Some(Box::new(left_branch) as Box<dyn std::any::Any>);
        node.right = Some(Box::new(right_branch) as Box<dyn std::any::Any>);

        // Parents do not need to keep the list of primitives once subdivided
        node.data.primitives = None;
    }

    pub fn build(&mut self) {
        // find the smallest possible shape that all primitives can be contained in
        let mut xmin = 0.0;
        let mut xmax = 1.0;
        let mut ymin = 0.0;
        let mut ymax = 1.0;
        let mut zmin = 0.0;
        let mut zmax = 1.0;

        for primitive in &self.primitives {
            let bounding = primitive.box3d();
            if bounding.xmin < xmin { xmin = bounding.xmin }
            if bounding.xmax > xmax { xmax = bounding.xmax }
            if bounding.ymin < ymin { ymin = bounding.ymin }
            if bounding.ymax > ymax { ymax = bounding.ymax }
            if bounding.zmin < zmin { zmin = bounding.zmin }
            if bounding.zmax > zmax { zmax = bounding.zmax }
        }

        let scene_box = Box3d::new(xmin, xmax, ymin, ymax, zmin, zmax);
        self.tree = TreeBranch::new(BV::new(scene_box)); // make the root tree branch a BV with all primitives
        self.tree.data.primitives = Option::from(self.primitives.clone());
        let triangle_threshold = 5;

        // Recursive building starting from the root of the tree
        Self::build_node(&mut self.tree, triangle_threshold);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vector3d::Vector3d;

    #[test]
    fn test_bvh_build() {
        // Create 6 triangles at different positions to trigger subdivision (threshold is 5)
        let t1 = Triangle3d::new(
            Vector3d::new(0.1, 0.1, 0.1),
            Vector3d::new(0.2, 0.1, 0.1),
            Vector3d::new(0.15, 0.2, 0.1),
        );
        let t2 = Triangle3d::new(
            Vector3d::new(0.8, 0.8, 0.8),
            Vector3d::new(0.9, 0.8, 0.8),
            Vector3d::new(0.85, 0.9, 0.8),
        );
        let t3 = Triangle3d::new(
            Vector3d::new(0.11, 0.11, 0.11),
            Vector3d::new(0.21, 0.11, 0.11),
            Vector3d::new(0.16, 0.21, 0.11),
        );
        let t4 = Triangle3d::new(
            Vector3d::new(0.81, 0.81, 0.81),
            Vector3d::new(0.91, 0.81, 0.81),
            Vector3d::new(0.86, 0.91, 0.81),
        );
        let t5 = Triangle3d::new(
            Vector3d::new(0.12, 0.12, 0.12),
            Vector3d::new(0.22, 0.12, 0.12),
            Vector3d::new(0.17, 0.22, 0.12),
        );
        let t6 = Triangle3d::new(
            Vector3d::new(0.82, 0.82, 0.82),
            Vector3d::new(0.92, 0.82, 0.82),
            Vector3d::new(0.87, 0.92, 0.82),
        );

        let primitives = vec![t1, t2, t3, t4, t5, t6];
        let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let tree = TreeBranch::new(BV::new(scene_box));
        let mut bvh = BVH { tree, primitives };

        bvh.build();

        // After build, the root node should have subdivided since 6 > 5 (threshold)
        assert!(bvh.tree.left.is_some());
        assert!(bvh.tree.right.is_some());
        assert!(bvh.tree.data.primitives.is_none());
    }
}
