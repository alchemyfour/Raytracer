use crate::bintree::BinaryTree;
use crate::box3d::Box3d;
use crate::bvh::BVH;
use crate::vector3d::{Plane, Vector3d};

pub struct Triangle3d {
    a: Vector3d,
    b: Vector3d,
    c: Vector3d,
}

impl Triangle3d {
    pub fn new(a: Vector3d, b: Vector3d, c: Vector3d) -> Triangle3d {
        Triangle3d {a, b, c}
    }

    pub fn normal(&self) -> Vector3d {
        let u = self.b - self.a; // get the edge vectors
        let v = self.c - self.a;
        (u.cross(v)).normalize() // and cross and normalize
    }

    pub fn plane(&self) -> Plane {
        Plane::new(self.a, self.normal())
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