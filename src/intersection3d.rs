use crate::vector3d::Vector3d;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection3d {
    hit: bool,
    incidence: Vector3d,
    location: Vector3d,
    normal: Vector3d
}

impl Intersection3d {
    pub fn new(incidence: Vector3d, location: Vector3d, normal: Vector3d) -> Intersection3d {
        Intersection3d { hit: true, incidence, location, normal }
    }
}