use crate::bvh::BVH;
use crate::vector3d::Ray3d;

pub struct Raycast<'a> {
    pub ray3d: Ray3d,
    pub bvh: &'a BVH,
}

impl<'a> Raycast<'a> {
    pub fn new(ray3d: Ray3d, bvh: &'a BVH) -> Self {
        Raycast { ray3d, bvh }
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
}