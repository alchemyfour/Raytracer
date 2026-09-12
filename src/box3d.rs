use crate::vector3d::Vector3d;

#[derive(Clone, Copy)]
pub struct Box3d {
    pub(crate) xmin: f32,
    pub(crate) xmax: f32,
    pub(crate) ymin: f32,
    pub(crate) ymax: f32,
    pub(crate) zmin: f32,
    pub(crate) zmax: f32,
}

impl Box3d {
    pub fn new(xmin: f32, xmax: f32, ymin: f32, ymax: f32, zmin: f32, zmax: f32) -> Self {
        Box3d {xmin, xmax, ymin, ymax, zmin, zmax}
    }
    
    pub fn center(&self) -> Vector3d {
        Vector3d::new((self.xmax + self.xmin)/2.0, (self.ymax + self.ymin)/2.0, (self.zmax + self.zmin)/2.0)
    }
    
}