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

    let ground_col = Vector3d::new(0.8, 0.8, 0.8);
    let g_a = Vector3d::new(-200.0, -3.0, -100.0);
    let g_b = Vector3d::new( 400.0, -3.0, -100.0);
    let g_c = Vector3d::new( 400.0, -3.0,  600.0);
    let g_d = Vector3d::new(-200.0, -3.0,  600.0);
    let mut ground1 = Triangle3d::new(g_a, g_c, g_b, ground_col);
    let mut ground2 = Triangle3d::new(g_a, g_d, g_c, ground_col);
    ground1.roughness = 1.0;
    ground2.roughness = 1.0;

    let mut sphere1 = Sphere::new(Vector3d::new( 2.0,  0.0, 18.0), 3.0, Vector3d::new(0.1, 0.6, 0.2));
    let mut sphere2 = Sphere::new(Vector3d::new(14.0,  1.0, 27.0), 4.0, Vector3d::new(0.7, 0.15, 0.15));
    let mut sphere3 = Sphere::new(Vector3d::new(-3.0, -1.0, 12.0), 2.0, Vector3d::new(0.2, 0.3, 0.8));
    sphere1.roughness = 0.05;
    sphere2.roughness = 0.5;
    sphere3.roughness = 0.9;
    sphere2.metallic = 0.0;

    let p0   = Vector3d::new( 5.0, -3.0, 20.0);
    let p1   = Vector3d::new(11.0, -3.0, 20.0);
    let p2   = Vector3d::new( 8.0, -3.0, 25.0);
    let apex = Vector3d::new( 8.0,  1.5, 21.7);
    let gold = Vector3d::new(0.9, 0.7, 0.1);
    let mut face1 = Triangle3d::new(p0, p1, apex, gold);
    face1.roughness = 0.2;
    let mut face2 = Triangle3d::new(p1, p2, apex, gold);
    face2.roughness = 0.2;
    let mut face3 = Triangle3d::new(p2, p0, apex, gold);
    face3.roughness = 0.2;
    let base  = Triangle3d::new(p0, p2, p1, gold);

    let primitives = vec![
        Primitive::Triangle(ground1),
        Primitive::Triangle(ground2),
        Primitive::Triangle(face1),
        Primitive::Triangle(face2),
        Primitive::Triangle(face3),
        Primitive::Triangle(base),
        Primitive::Sphere(sphere1),
        Primitive::Sphere(sphere2),
        Primitive::Sphere(sphere3),
    ];

    let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
    let tree = TreeBranch::new(BV::new(scene_box));
    let mut bvh = BVH { tree, primitives };
    bvh.build();
    let scale = 1.0;

    let mut camera = Camera::new(
        Vector3d::new(0.0, 0.0, -50.0),
        50.0/0.0174533,
        60.0/0.0174533,
        4160.0 * scale,
        1440.0 * scale,
        Vector3d::new(0.0, 0.0, 1.0),
        bvh
    );
    let pixels = camera.fire();
    let _ = save_png(&*pixels, "pixels.png");
}
// we're so much cuter than this guy, we don't have a brow ridge (Referring to a skull)