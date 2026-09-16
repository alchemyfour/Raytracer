use crate::bvh::{BV, BVH, Primitive};
use crate::vector3d::{Ray3d, Vector3d};
use crate::bintree::TreeBranch;

#[derive(Clone, Copy)]
pub struct Raycast<'a> {
    pub ray3d: Ray3d,
    pub bvh: &'a BVH,
}

pub enum Resolvable<'a> {
    Primitive(&'a Primitive),
    BV(&'a BV),
    Branch(&'a TreeBranch),
}

impl<'a> Resolvable<'a> {
    pub fn check_intersection(&self, ray: &Ray3d) -> Option<Vector3d> {
        match self {
            Resolvable::Primitive(primitive) => {
                match primitive {
                    Primitive::Triangle(triangle) => {
                        let e1 = triangle.b - triangle.a;
                        let e2 = triangle.c - triangle.a;
                        let h = ray.direction.cross(e2);
                        let a_det = e1.dot(h);
                        if a_det.abs() < 1e-6 {
                            return None;
                        }
                        let f = 1.0 / a_det;
                        let s = ray.origin - triangle.a;
                        let u = f * s.dot(h);
                        if u < 0.0 || u > 1.0 {
                            return None;
                        }
                        let q = s.cross(e1);
                        let v = f * ray.direction.dot(q);
                        if v < 0.0 || u + v > 1.0 {
                            return None;
                        }
                        let t = f * e2.dot(q);
                        if t > 1e-6 {
                            Some(ray.t(&t))
                        } else {
                            None
                        }
                    }
                    Primitive::Box(box3d) => {
                        ray.intersecting_box(box3d)
                    }
                    Primitive::Sphere(sphere) => {
                        let oc = ray.origin - sphere.center;
                        let b = 2.0 * ray.direction.dot(oc);
                        let c = oc.dot(oc) - sphere.radius * sphere.radius;
                        let discriminant = b * b - 4.0 * c;
                        if discriminant < 0.0 {
                            None
                        } else {
                            let t1 = (-b - discriminant.sqrt()) / 2.0;
                            let t2 = (-b + discriminant.sqrt()) / 2.0;
                            let t = if t1 >= 0.0 {
                                t1
                            } else if t2 >= 0.0 {
                                t2
                            } else {
                                return None;
                            };
                            Some(ray.t(&t))
                        }
                    }
                    Primitive::Plane(plane) => {
                        ray.intersecting_plane(plane)
                    }
                }
            }
            Resolvable::BV(bv) => {
                ray.intersecting_box(&bv.box3d)
            }
            Resolvable::Branch(branch) => {
                ray.intersecting_box(&branch.data.box3d)
            }
        }
    }
}

impl<'a> Raycast<'a> {
    pub fn new(ray3d: Ray3d, bvh: &'a BVH) -> Self {
        Raycast { ray3d, bvh }
    }

    pub fn resolve(self: Raycast<'a>) -> Option<(Vector3d, &'a Primitive)> {
        let mut resolve_stack: std::vec::Vec<Resolvable<'a>> = Vec::new();
        resolve_stack.push(Resolvable::Branch(&self.bvh.tree));

        const EPSILON: f32 = 0.001;
        let mut closest_hit: Option<(Vector3d, &'a Primitive)> = None;
        let mut min_t = f32::INFINITY;

        while let Some(item) = resolve_stack.pop() {
            if let Some(hit_point) = item.check_intersection(&self.ray3d) {
                let t = (hit_point - self.ray3d.origin).magnitude();
                if t >= min_t {
                    continue;
                }

                match item {
                    Resolvable::Branch(node) => {
                        if let Some(ref primitives) = node.data.primitives {
                            for primitive in primitives {
                                resolve_stack.push(Resolvable::Primitive(primitive));
                            }
                        }
                        if let Some(ref right_any) = node.right {
                            if let Some(right_branch) = right_any.downcast_ref::<TreeBranch>() {
                                resolve_stack.push(Resolvable::Branch(right_branch));
                            }
                        }
                        if let Some(ref left_any) = node.left {
                            if let Some(left_branch) = left_any.downcast_ref::<TreeBranch>() {
                                resolve_stack.push(Resolvable::Branch(left_branch));
                            }
                        }
                    }
                    Resolvable::Primitive(primitive) => {
                        if t > EPSILON {
                            min_t = t;
                            closest_hit = Some((hit_point, primitive));
                        }
                    }
                    Resolvable::BV(bv) => {
                        if let Some(ref primitives) = bv.primitives {
                            for primitive in primitives {
                                resolve_stack.push(Resolvable::Primitive(primitive));
                            }
                        }
                    }
                }
            }
        }

        closest_hit
    }
}
