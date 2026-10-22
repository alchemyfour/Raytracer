use crate::intersection3d::Intersection3d;
use crate::ray::Raycast;
use crate::vector3d::Vector3d;

#[derive(Clone)]
pub(crate) struct Pixel<'a> {
    // extremely poor form to use a struct for this, every reference to an attribute requires
    // significant pointer thrashing. This should be ported to use 2d buffers, because this will 
    // TANK GPU performance when ported to wgpu.
    pub bounces: f32,
    pub color: Vector3d,
    pub raycast: Raycast<'a>,
    pub intersections: Vec<Intersection3d>,
    pub x: f32,
    pub y: f32,
}

impl Pixel<'_> {
    pub(crate) fn new(raycast: Raycast, x: f32, y: f32) -> Pixel {
        Pixel {raycast, x, y, color: Vector3d::new(0.0, 0.0, 0.0), intersections: Vec::new(), bounces: 0.0}
    }
}
// Replacement image based buffers
/*
- Skywards
- Normal
- Color
- Metalic
 */