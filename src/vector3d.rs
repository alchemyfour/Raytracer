use std::ops::Mul;
use std::ops::Div;
use std::ops::Add;
use std::ops::Sub;
use crate::box3d::Box3d;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3d {
        Vector3d { x, y, z }
    }
}

impl Add<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<&f32> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &f32) -> Vector3d {
        Vector3d::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Add<&Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<&Vector3d> for &Vector3d {
    type Output = Vector3d;

    fn sub(self, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Div<f32> for Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Vector3d {
        Vector3d::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}


impl Div<f32> for &Vector3d {
    type Output = Vector3d;

    fn div(self, rhs: f32) -> Vector3d {
        Vector3d::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}


impl Div<Vector3d> for f32 {
    type Output = Vector3d;

    fn div(self, rhs: Vector3d) -> Vector3d {
        rhs / self
    }
}


impl Div<&Vector3d> for f32 {
    type Output = Vector3d;

    fn div(self, rhs: &Vector3d) -> Vector3d {
        rhs / self
    }
}


impl Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Vector3d {
        Vector3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3d> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Vector3d {
        Vector3d::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}


impl Mul<f32> for &Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Vector3d {
        Vector3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}


impl Mul<Vector3d> for f32 {
    type Output = Vector3d;

    fn mul(self, rhs: Vector3d) -> Vector3d {
        rhs * self
    }
}


impl Mul<&Vector3d> for f32 {
    type Output = Vector3d;

    fn mul(self, rhs: &Vector3d) -> Vector3d {
        rhs * self
    }
}

impl Vector3d {

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3d {
        let m = self.magnitude();
        let x = self.x / m;
        let y = self.y / m;
        let z = self.z / m;
        Vector3d::new(x, y, z)
    }

    pub fn dot(&self, other: Vector3d) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vector3d) -> Vector3d {
        Vector3d::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }


    pub fn sub(&self, other: Vector3d) -> Vector3d {
        Vector3d::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub normal: Vector3d,
    pub origin: Vector3d,
    pub x_size: f32,
    pub y_size: f32,
}

impl Plane {
    pub fn new(normal: Vector3d, origin: Vector3d, x_size: f32, y_size: f32) -> Plane {
        Plane { normal, origin, x_size, y_size }
    }

    pub fn local_axes(&self) -> (Vector3d, Vector3d) {
        let n = self.normal.normalize();
        let helper = if n.x.abs() > 0.9 {
            Vector3d::new(0.0, 1.0, 0.0)
        } else {
            Vector3d::new(1.0, 0.0, 0.0)
        };
        let local_x = n.cross(helper).normalize();
        let local_y = n.cross(local_x).normalize();
        (local_x, local_y)
    }

    pub fn normal(&self) -> Vector3d {
        self.normal.normalize()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Vector3d,
    pub radius: f32,
    pub color: Vector3d,
    pub roughness: f32,
    pub metallic: f32
}

impl Sphere {
    pub fn new(center: Vector3d, radius: f32, color: Vector3d) -> Sphere {
        Sphere { center, radius, color, roughness: 0.02, metallic: 1.0 }
    }

    pub fn normal(&self, point: Vector3d) -> Vector3d {
        (point - self.center).normalize()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray3d {
    pub origin: Vector3d,
    pub direction: Vector3d,
}

impl Ray3d {
    pub fn new(origin: Vector3d, direction: Vector3d) -> Ray3d {
        Ray3d { origin, direction: direction.normalize() }
    }

    pub fn t(&self, t: &f32) -> Vector3d {
        Vector3d::new(self.direction.x * t + self.origin.x, self.direction.y * t + self.origin.y, self.direction.z * t + self.origin.z)
    }

    pub fn intersecting_plane(&self, plane: &Plane) -> Option<Vector3d> {
        let denom = self.direction.dot(plane.normal);

        if denom.abs() < 1e-6 {
            return None;
        }

        let t = plane.origin.sub(self.origin).dot(plane.normal) / denom;

        if t < 0.0 {
            return None;
        }

        Some(self.origin.add(self.direction.mul(t)))
    }

    pub fn intersecting_box(&self, box3d: &Box3d) -> Option<f32> {
        let inv_dir_x = 1.0 / self.direction.x;
        let inv_dir_y = 1.0 / self.direction.y;
        let inv_dir_z = 1.0 / self.direction.z;

        let t0_x = (box3d.xmin - self.origin.x) * inv_dir_x;
        let t1_x = (box3d.xmax - self.origin.x) * inv_dir_x;
        let tmin_x = t0_x.min(t1_x);
        let tmax_x = t0_x.max(t1_x);

        let t0_y = (box3d.ymin - self.origin.y) * inv_dir_y;
        let t1_y = (box3d.ymax - self.origin.y) * inv_dir_y;
        let tmin_y = t0_y.min(t1_y);
        let tmax_y = t0_y.max(t1_y);

        let t0_z = (box3d.zmin - self.origin.z) * inv_dir_z;
        let t1_z = (box3d.zmax - self.origin.z) * inv_dir_z;
        let tmin_z = t0_z.min(t1_z);
        let tmax_z = t0_z.max(t1_z);

        let tmin = tmin_x.max(tmin_y).max(tmin_z);
        let tmax = tmax_x.min(tmax_y).min(tmax_z);

        if tmax >= tmin && tmax >= 0.0 {
            Some(tmin.max(0.0))
        } else {
            None
        }
    }
}
