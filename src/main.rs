use crate::vector3d::Sphere;
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
        Vector3d::new(0.0, 0.0, 1.0)
    );


    let sphere1 = Sphere::new(Vector3d::new(0.0, 16.0, 16.0), 5.0, Vector3d::new(0.0, 1.0, 0.0));

    let sphere2 = Sphere::new(Vector3d::new(0.0, 8.0, 16.0), 7.0, Vector3d::new(9.0, 0.0, 0.0));

    let primitives = vec![
        Primitive::Triangle(t_mirror1),
        Primitive::Sphere(sphere1),
        Primitive::Sphere(sphere2),
    ];

    let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
    let tree = TreeBranch::new(BV::new(scene_box));
    let mut bvh = BVH { tree, primitives };
    bvh.build();
    let scale = 1.0;

    let mut camera = Camera::new(
        Vector3d::new(-25.0, 0.0, -50.0),
        50.0,
        70.0,
        4160.0*scale,
        1440.0*scale,
        Vector3d::new(0.5, 0.0, 1.0),
        bvh
    );
    let pixels = camera.fire();
    let _ = save_png(&*pixels, "pixels.png");
}
