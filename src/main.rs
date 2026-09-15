use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::bvh::{BV, BVH, Primitive};
use crate::camera::Camera;
use crate::png::save_png;
use crate::tri::Triangle3d;
use crate::vector3d::{Ray3d, Vector3d};

mod vector3d;
mod tri;
mod box3d;
mod intersection3d;
mod bvh;
mod bintree;
mod ray;
mod camera;
mod pixel;
mod png;

fn main() {
    // 1. Mirror 1 (Tilted at 45 degrees in the center, reflects camera rays to the right)
    let t_mirror1 = Triangle3d::new(
        Vector3d::new(0.5, -1.0, 0.5),
        Vector3d::new(-0.5, -1.0, -0.5),
        Vector3d::new(0.0, 1.0, 0.0),
    );

    // 2. Target 1 (Located on the right side to intercept the reflected rays from Mirror 1)
    let t_target1 = Triangle3d::new(
        Vector3d::new(2.0, -1.0, -1.0),
        Vector3d::new(2.0, -1.0, 1.0),
        Vector3d::new(2.0, 1.0, 0.0),
    );

    // 3. Mirror 2 (Tilted at 45 degrees, reflects camera rays to the left)
    let t_mirror2 = Triangle3d::new(
        Vector3d::new(-1.5, -1.0, -0.5),
        Vector3d::new(-2.5, -1.0, 0.5),
        Vector3d::new(-2.0, 1.0, 0.0),
    );

    // 4. Target 2 (Located on the left side to intercept the reflected rays from Mirror 2)
    let t_target2 = Triangle3d::new(
        Vector3d::new(-4.0, -1.0, 1.0),
        Vector3d::new(-4.0, -1.0, -1.0),
        Vector3d::new(-4.0, 1.0, 0.0),
    );

    // 5 & 6. Background Wall (Located far behind to catch rays that miss the mirrors)
    let t_bg1 = Triangle3d::new(
        Vector3d::new(-5.0, -3.0, 4.0),
        Vector3d::new(5.0, -3.0, 4.0),
        Vector3d::new(0.0, 5.0, 4.0),
    );
    let t_bg2 = Triangle3d::new(
        Vector3d::new(-5.0, 5.0, 4.0),
        Vector3d::new(-5.0, -3.0, 4.0),
        Vector3d::new(5.0, 5.0, 4.0),
    );

    let primitives = vec![
        Primitive::Triangle(t_mirror1),
        Primitive::Triangle(t_target1),
        Primitive::Triangle(t_mirror2),
        Primitive::Triangle(t_target2),
        Primitive::Triangle(t_bg1),
        Primitive::Triangle(t_bg2),
    ];

    let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
    let tree = TreeBranch::new(BV::new(scene_box));
    let mut bvh = BVH { tree, primitives };
    bvh.build();

    let mut camera = Camera::new(
        Vector3d::new(0.0, -50.0, -50.0),
        50.0,
        70.0,
        1380.0,
        620.0,
        Vector3d::new(-0.0, 1.0, 1.0),
        bvh
    );
    let pixels = camera.fire();
    let _ = save_png(&*pixels, "pixels.png");
}
