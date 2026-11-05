use std::time::Instant;
use crate::vector3d::Sphere;
use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::bvh::{BV, BVH, Primitive};
use crate::camera::Camera;
use crate::light::Light;
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
mod light;
mod buffer;

fn main() {
    // --- Cornell Box Materials ---
    let white = Vector3d::new(0.73, 0.73, 0.73);
    let red   = Vector3d::new(0.65, 0.45, 0.05);
    let green = Vector3d::new(0.32, 0.45, 0.15);

    // --- Room Vertices ---
    let v_f0 = Vector3d::new(-10.0,  0.0,  0.0);
    let v_f1 = Vector3d::new( 10.0,  0.0,  0.0);
    let v_f2 = Vector3d::new( 10.0,  0.0, 20.0);
    let v_f3 = Vector3d::new(-10.0,  0.0, 20.0);

    let v_c0 = Vector3d::new(-10.0, 20.0,  0.0);
    let v_c1 = Vector3d::new( 10.0, 20.0,  0.0);
    let v_c2 = Vector3d::new( 10.0, 20.0, 20.0);
    let v_c3 = Vector3d::new(-10.0, 20.0, 20.0);

    let mut primitives = Vec::new();

    // Floor (White)
    // --- Room Walls & Roughness Control ---
    let wall_roughness = 0.2; 
    // Helper closure to quickly instantiate and push walls with the target roughness
    let mut push_wall = |a: Vector3d, b: Vector3d, c: Vector3d, col: Vector3d| {
        let mut tri = Triangle3d::new(a, b, c, col);
        tri.roughness = wall_roughness;
        tri.metallic = 0.2;
        primitives.push(Primitive::Triangle(tri));
    };

    // Floor (White)
    push_wall(v_f0, v_f1, v_f2, white);
    push_wall(v_f0, v_f2, v_f3, white);

    // Ceiling (White)
    push_wall(v_c0, v_c2, v_c1, white);
    push_wall(v_c0, v_c3, v_c2, white);

    // Back wall (White)
    push_wall(v_f3, v_f2, v_c2, white);
    push_wall(v_f3, v_c2, v_c3, white);

    // Left wall (Red)
    push_wall(v_f0, v_f3, v_c3, red);
    push_wall(v_f0, v_c3, v_c0, red);

    // Right wall (Green)
    push_wall(v_f1, v_c2, v_f2, green);
    push_wall(v_f1, v_c1, v_c2, green);

    // --- The Cube (Tall block on the right) ---
    let cube_col = Vector3d::new(0.8, 0.8, 0.8);
    let c_min = Vector3d::new(3.0, 0.0, 10.0);
    let c_max = Vector3d::new(8.0, 10.0, 15.0);
    // --- The Cube (Tall block on the right) ---
    let cube_col = Vector3d::new(0.8, 0.8, 0.8);
    let c_min = Vector3d::new(3.0, 0.0, 10.0);
    let c_max = Vector3d::new(8.0, 10.0, 15.0);

    // Cube vertices
    let b0 = Vector3d::new(c_min.x, c_min.y, c_min.z);
    let b1 = Vector3d::new(c_max.x, c_min.y, c_min.z);
    let b2 = Vector3d::new(c_max.x, c_min.y, c_max.z);
    let b3 = Vector3d::new(c_min.x, c_min.y, c_max.z);
    let b4 = Vector3d::new(c_min.x, c_max.y, c_min.z);
    let b5 = Vector3d::new(c_max.x, c_max.y, c_min.z);
    let b6 = Vector3d::new(c_max.x, c_max.y, c_max.z);
    let b7 = Vector3d::new(c_min.x, c_max.y, c_max.z);

    // Front, Back, Left, Right, Top (Skipping bottom since it's on the floor)
    let cube_tris = vec![
        Triangle3d::new(b0, b1, b5, cube_col), Triangle3d::new(b0, b5, b4, cube_col), // Front
        Triangle3d::new(b2, b3, b7, cube_col), Triangle3d::new(b2, b7, b6, cube_col), // Back
        Triangle3d::new(b3, b0, b4, cube_col), Triangle3d::new(b3, b4, b7, cube_col), // Left
        Triangle3d::new(b1, b2, b6, cube_col), Triangle3d::new(b1, b6, b5, cube_col), // Right
        Triangle3d::new(b4, b5, b6, cube_col), Triangle3d::new(b4, b6, b7, cube_col), // Top
    ];

    for mut tri in cube_tris {
        tri.roughness = 0.1;
        tri.metallic = 0.1;
        primitives.push(Primitive::Triangle(tri));
    }

    // --- Spheres ---
    // Large mirror sphere
    let mut sphere1 = Sphere::new(Vector3d::new(-4.0, 4.0, 13.0), 4.0, Vector3d::new(0.9, 0.9, 0.9));
    sphere1.roughness = 0.07;
    sphere1.metallic = 0.75;

    // Small diffuse sphere
    let mut sphere2 = Sphere::new(Vector3d::new(-1.0, 2.5, 6.0), 2.5, Vector3d::new(0.2, 0.3, 0.8));
    sphere2.roughness = 0.9;
    sphere2.metallic = 1.0;

    primitives.push(Primitive::Sphere(sphere1));
    primitives.push(Primitive::Sphere(sphere2));

    // --- Lights ---
    // Main ceiling light
    let light1 = Light::new(Vector3d::new(0.0, 19.9, 10.0), 30.0, 1.0, Vector3d::new(0.8, 0.9, 0.5));
    let light2 = Light::new(Vector3d::new(0.0, 10.9, 10.0), 30.0, 0.0, Vector3d::new(0.8, 0.9, 0.5));
    let lights = vec![light1];

    // --- BVH & Scene Setup ---
    let scene_box = Box3d::new(-15.0, 15.0, -5.0, 25.0, -5.0, 25.0);
    let tree = TreeBranch::new(BV::new(scene_box));
    let mut bvh = BVH { tree, primitives };

    println!("{} total primitives", bvh.primitives.len());
    let start_time = Instant::now();
    bvh.build();
    println!("BVH build finished in {:?} microseconds!", start_time.elapsed().as_micros());

    let scale = 1.0;


    let mut camera = Camera::new(
        Vector3d::new(0.0, 10.0, -10.0),
        70.0 * 0.0174533,
        70.0 * 0.0174533,
        4160.0 * scale,
        1440.0 * scale,
        Vector3d::new(0.0, 0.0, 1.0),
        bvh,
        lights,
    );

    println!("{} total lights", camera.lights.len());
    println!("Starting trace now");

    let trace_time = Instant::now();
    let pixels = camera.render(10); // Or render(10) depending on your setup
    let _ = save_png(&pixels, "render.png");

    println!("{} total pixels, finished in {} seconds", camera.resolutionx * camera.resolutiony, trace_time.elapsed().as_millis() as f32 / 1000.0);
    println!("{} microseconds per pixel", ((trace_time.elapsed().as_micros() as usize)) as f32 / (camera.resolutionx * camera.resolutiony));
}