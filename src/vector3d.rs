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

impl Add<&Vector3d> for Vector3d {
    type Output = Vector3d;

    fn add(self, rhs: &Vector3d) -> Vector3d {
        Vector3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
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
        self / rhs
    }
}


impl Mul<f32> for Vector3d {
    type Output = Vector3d;

    fn mul(self, rhs: f32) -> Vector3d {
        Vector3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
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


pub struct Plane {
    normal: Vector3d,
    origin: Vector3d,
}

impl Plane {
    pub fn new(normal: Vector3d, origin: Vector3d) -> Plane {
        Plane { normal, origin }
    }
}
pub struct Ray3d {
    origin: Vector3d,
    direction: Vector3d,
}

impl Ray3d {
    pub fn new(origin: Vector3d, direction: Vector3d) -> Ray3d {
        Ray3d {origin, direction: direction.normalize()}
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

    pub fn intersecting_box(&self, box3d: &Box3d) -> Option<Vector3d> {
        let mut tmin = f32::NEG_INFINITY;
        let mut tmax = f32::INFINITY;
        
        if self.direction.x.abs() < 1e-6 {
            if self.origin.x < box3d.xmin || self.origin.x > box3d.xmax  {
                return None;
            }
        } else {
            let inv_dir = 1.0 / self.direction.x;
            let mut t1 = (box3d.xmin - self.origin.x) * inv_dir;
            let mut t2 = (box3d.xmax - self.origin.x) * inv_dir;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return None;
            }
        }
        
        if self.direction.y.abs() < 1e-6 {
            if self.origin.y < box3d.ymin || self.origin.y > box3d.ymax {
                return None;
            }
        } else {
            let inv_dir = 1.0 / self.direction.y;
            let mut t1 = (box3d.ymin - self.origin.y) * inv_dir;
            let mut t2 = (box3d.ymax - self.origin.y) * inv_dir;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return None;
            }
        }
        
        if self.direction.z.abs() < 1e-6 {
            if self.origin.z < box3d.zmin || self.origin.z > box3d.zmax {
                return None;
            }
        } else {
            let inv_dir = 1.0 / self.direction.z;
            let mut t1 = (box3d.zmin - self.origin.z) * inv_dir;
            let mut t2 = (box3d.zmax - self.origin.z) * inv_dir;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
            }
            tmin = tmin.max(t1);
            tmax = tmax.min(t2);

            if tmin > tmax {
                return None;
            }
        }
        
        if tmax < 0.0 {
            return None;
        }
        
        let t = if tmin >= 0.0 { tmin } else { tmax };

        Some(self.t(&t))
    }
}


