use crate::vector3d::Vector3d;

#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn normal(&self, point: Vector3d) -> Vector3d {
        let epsilon = 1e-4;
        if (point.x - self.xmin).abs() < epsilon {
            Vector3d::new(-1.0, 0.0, 0.0)
        } else if (point.x - self.xmax).abs() < epsilon {
            Vector3d::new(1.0, 0.0, 0.0)
        } else if (point.y - self.ymin).abs() < epsilon {
            Vector3d::new(0.0, -1.0, 0.0)
        } else if (point.y - self.ymax).abs() < epsilon {
            Vector3d::new(0.0, 1.0, 0.0)
        } else if (point.z - self.zmin).abs() < epsilon {
            Vector3d::new(0.0, 0.0, -1.0)
        } else if (point.z - self.zmax).abs() < epsilon {
            Vector3d::new(0.0, 0.0, 1.0)
        } else {
            let dx1 = (point.x - self.xmin).abs();
            let dx2 = (point.x - self.xmax).abs();
            let dy1 = (point.y - self.ymin).abs();
            let dy2 = (point.y - self.ymax).abs();
            let dz1 = (point.z - self.zmin).abs();
            let dz2 = (point.z - self.zmax).abs();

            let min_d = dx1.min(dx2).min(dy1).min(dy2).min(dz1).min(dz2);
            if min_d == dx1 {
                Vector3d::new(-1.0, 0.0, 0.0)
            } else if min_d == dx2 {
                Vector3d::new(1.0, 0.0, 0.0)
            } else if min_d == dy1 {
                Vector3d::new(0.0, -1.0, 0.0)
            } else if min_d == dy2 {
                Vector3d::new(0.0, 1.0, 0.0)
            } else if min_d == dz1 {
                Vector3d::new(0.0, 0.0, -1.0)
            } else {
                Vector3d::new(0.0, 0.0, 1.0)
            }
        }
    }
}
