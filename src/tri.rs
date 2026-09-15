use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::bvh::BVH;
use crate::vector3d::{Plane, Vector3d};

#[derive(Clone, Debug)]
pub struct Triangle3d {
    pub a: Vector3d,
    pub b: Vector3d,
    pub c: Vector3d,
    pub color: Vector3d,
}

impl Triangle3d {
    pub fn new(a: Vector3d, b: Vector3d, c: Vector3d, color: Vector3d) -> Triangle3d {
        Triangle3d {a, b, c, color}
    }

    pub fn normal(&self) -> Vector3d {
        let u = self.b - self.a; // get the edge vectors
        let v = self.c - self.a;
        (u.cross(v)).normalize() // and cross and normalize
    }

    pub fn plane(&self) -> Plane {
        let bbox = self.box3d();
        let dx = bbox.xmax - bbox.xmin;
        let dy = bbox.ymax - bbox.ymin;
        let dz = bbox.zmax - bbox.zmin;
        let max_dim = dx.max(dy).max(dz);
        Plane::new(self.normal(), self.a, max_dim, max_dim)
    }

    pub fn box3d(&self) -> Box3d {
        let xmin = self.a.x.min(self.b.x).min(self.c.x);
        let xmax = self.a.x.max(self.b.x).max(self.c.x);
        let ymin = self.a.y.min(self.b.y).min(self.c.y);
        let ymax = self.a.y.max(self.b.y).max(self.c.y);
        let zmin = self.a.z.min(self.b.z).min(self.c.z);
        let zmax = self.a.z.max(self.b.z).max(self.c.z);
        Box3d::new(xmin, xmax, ymin, ymax, zmin, zmax)
    }

    pub fn center(&self) -> Vector3d {
        Vector3d::new(self.a.x + self.b.x, self.a.y + self.b.y, self.a.z + self.b.z) / 3.0
    }
}

pub struct Triangles3d {
    triangles: Vec<Triangle3d>,
    bvh: Option<BVH>
}

impl Triangles3d {
    pub fn new() -> Triangles3d {
        Triangles3d { triangles: Vec::new(), bvh: None }
    }

    pub fn build(&mut self) {
        // no idea how this works
    }
}
