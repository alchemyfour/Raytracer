use crate::intersection3d::Intersection3d;
use crate::vector3d::Vector3d;

struct Pixel {
    bounces: f64,
    color: Option<Vector3d>,
    intersections: Vec<Intersection3d>,
    x: f64,
    y: f64,
}