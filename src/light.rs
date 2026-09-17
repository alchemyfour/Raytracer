use crate::vector3d::Vector3d;

#[derive(Clone)]
pub struct Light {
    pub position: Vector3d,
    pub radius: f32,
    pub brightness: f32,
    pub color: Vector3d,
}

impl Light {
    pub fn new(position: Vector3d, radius: f32, brightness: f32, color: Vector3d) -> Light {
        Light { position, radius, brightness, color }
    }
}