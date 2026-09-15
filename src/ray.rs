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
                        if t < min_t {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bintree::TreeBranch;
    use crate::box3d::Box3d;
    use crate::bvh::BV;
    use crate::vector3d::Vector3d;

    #[test]
    fn test_raycast_creation() {
        let origin = Vector3d::new(0.0, 0.0, 0.0);
        let direction = Vector3d::new(0.0, 0.0, 1.0);
        let ray = Ray3d::new(origin, direction);

        let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let tree = TreeBranch::new(BV::new(scene_box));
        let bvh = BVH { tree, primitives: vec![] };

        let raycast = Raycast::new(ray, &bvh);
        assert_eq!(raycast.bvh.primitives.len(), 0);
    }

    #[test]
    fn test_resolvable_intersection_sphere() {
        use crate::vector3d::Sphere;
        let origin = Vector3d::new(0.0, 0.0, -5.0);
        let direction = Vector3d::new(0.0, 0.0, 1.0);
        let ray = Ray3d::new(origin, direction);

        let sphere = Sphere::new(Vector3d::new(0.0, 0.0, 0.0), 1.0);
        let prim = Primitive::Sphere(sphere);
        let resolvable = Resolvable::Primitive(&prim);

        let hit = resolvable.check_intersection(&ray);
        assert!(hit.is_some());
        let pt = hit.unwrap();
        assert!((pt.z - (-1.0)).abs() < 1e-4);
    }

    #[test]
    fn test_resolvable_intersection_triangle() {
        use crate::tri::Triangle3d;
        let origin = Vector3d::new(0.0, 0.0, -5.0);
        let direction = Vector3d::new(0.0, 0.0, 1.0);
        let ray = Ray3d::new(origin, direction);

        let tri = Triangle3d::new(
            Vector3d::new(-1.0, -1.0, 0.0),
            Vector3d::new(1.0, -1.0, 0.0),
            Vector3d::new(0.0, 1.0, 0.0),
        );
        let prim = Primitive::Triangle(tri);
        let resolvable = Resolvable::Primitive(&prim);

        let hit = resolvable.check_intersection(&ray);
        assert!(hit.is_some());
        let pt = hit.unwrap();
        assert!((pt.z - 0.0).abs() < 1e-4);
    }

    #[test]
    fn test_raycast_resolve_bvh() {
        use crate::tri::Triangle3d;
        let origin = Vector3d::new(0.0, 0.0, -5.0);
        let direction = Vector3d::new(0.0, 0.0, 1.0);
        let ray = Ray3d::new(origin, direction);

        let tri1 = Triangle3d::new(
            Vector3d::new(-1.0, -1.0, 2.0),
            Vector3d::new(1.0, -1.0, 2.0),
            Vector3d::new(0.0, 1.0, 2.0),
        );
        let tri2 = Triangle3d::new(
            Vector3d::new(-1.0, -1.0, 1.0),
            Vector3d::new(1.0, -1.0, 1.0),
            Vector3d::new(0.0, 1.0, 1.0),
        );

        let primitives = vec![
            Primitive::Triangle(tri1),
            Primitive::Triangle(tri2),
        ];

        let mut bvh = BVH {
            tree: TreeBranch::new(BV::new(Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0))),
            primitives,
        };
        bvh.build();

        let raycast = Raycast::new(ray, &bvh);
        let resolve_result = raycast.resolve();
        assert!(resolve_result.is_some());
        let (hit_pt, hit_prim) = resolve_result.unwrap();
        assert!((hit_pt.z - 1.0).abs() < 1e-4);
        
        match hit_prim {
            Primitive::Triangle(t) => {
                assert_eq!(t.a.z, 1.0);
            }
            _ => panic!("Expected triangle hit"),
        }
    }

    #[test]
    fn test_ray_starts_inside_bvh() {
        use crate::tri::Triangle3d;
        let origin = Vector3d::new(0.5, 0.5, 0.5);
        let direction = Vector3d::new(0.0, 0.0, 1.0);
        let ray = Ray3d::new(origin, direction);

        let tri = Triangle3d::new(
            Vector3d::new(0.4, 0.4, 0.8),
            Vector3d::new(0.6, 0.4, 0.8),
            Vector3d::new(0.5, 0.6, 0.8),
        );

        let primitives = vec![
            Primitive::Triangle(tri),
        ];

        let mut bvh = BVH {
            tree: TreeBranch::new(BV::new(Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0))),
            primitives,
        };
        bvh.build();

        let raycast = Raycast::new(ray, &bvh);
        let resolve_result = raycast.resolve();
        assert!(resolve_result.is_some());
        let (hit_pt, _) = resolve_result.unwrap();
        assert!((hit_pt.z - 0.8).abs() < 1e-4);
    }

    #[test]
    fn test_primitive_normals() {
        use crate::vector3d::{Sphere, Plane};
        use crate::tri::Triangle3d;
        use crate::box3d::Box3d;

        // 1. Triangle Normal
        let tri = Triangle3d::new(
            Vector3d::new(0.0, 0.0, 0.0),
            Vector3d::new(1.0, 0.0, 0.0),
            Vector3d::new(0.0, 1.0, 0.0),
        );
        let tri_prim = Primitive::Triangle(tri);
        let normal_tri = tri_prim.normal(Vector3d::new(0.2, 0.2, 0.0));
        assert_eq!(normal_tri, Vector3d::new(0.0, 0.0, 1.0));

        // 2. Sphere Normal
        let sphere = Sphere::new(Vector3d::new(0.0, 0.0, 0.0), 2.0);
        let sphere_prim = Primitive::Sphere(sphere);
        let normal_sphere = sphere_prim.normal(Vector3d::new(2.0, 0.0, 0.0));
        assert_eq!(normal_sphere, Vector3d::new(1.0, 0.0, 0.0));

        // 3. Plane Normal
        let plane = Plane::new(Vector3d::new(0.0, 1.0, 0.0), Vector3d::new(0.0, 0.0, 0.0), 10.0, 10.0);
        let plane_prim = Primitive::Plane(plane);
        let normal_plane = plane_prim.normal(Vector3d::new(5.0, 0.0, 5.0));
        assert_eq!(normal_plane, Vector3d::new(0.0, 1.0, 0.0));

        // 4. Box Normal
        let box3d = Box3d::new(-1.0, 1.0, -1.0, 1.0, -1.0, 1.0);
        let box_prim = Primitive::Box(box3d);
        // Test right face (+x)
        assert_eq!(box_prim.normal(Vector3d::new(1.0, 0.0, 0.0)), Vector3d::new(1.0, 0.0, 0.0));
        // Test left face (-x)
        assert_eq!(box_prim.normal(Vector3d::new(-1.0, 0.0, 0.0)), Vector3d::new(-1.0, 0.0, 0.0));
        // Test top face (+y)
        assert_eq!(box_prim.normal(Vector3d::new(0.0, 1.0, 0.0)), Vector3d::new(0.0, 1.0, 0.0));
    }
}
