use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::bvh::{BV, BVH};
use crate::tri::Triangle3d;
use crate::vector3d::Vector3d;

mod vector3d;
mod tri;
mod box3d;
mod intersection3d;
mod bvh;
mod bintree;
mod ray;

fn main() {
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
}