use crate::vector3d::Vector3d;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection3d {
    pub hit: bool,
    pub incidence: Vector3d,
    pub location: Vector3d,
    pub normal: Vector3d,
    pub color: Vector3d,
    pub metallic: f32
}

impl Intersection3d {
    pub fn new(incidence: Vector3d, location: Vector3d, normal: Vector3d, d: Vector3d, color: Vector3d, metallic: f32) -> Intersection3d {
        Intersection3d { hit: true, incidence, location, normal, color, metallic}
    }
}