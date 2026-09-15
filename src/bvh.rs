use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::tri::Triangle3d;
use crate::vector3d::{Vector3d, Plane, Sphere};

#[derive(Clone, Debug)]
pub enum Primitive {
    Triangle(Triangle3d),
    Box(Box3d),
    Sphere(Sphere),
    Plane(Plane),
}

impl Primitive {

    pub fn color(&self, point: Vector3d) -> Vector3d {
        //Vector3d { x: 0.5, y: 0.5, z: 0.5}
        match self {
            Primitive::Triangle(t) => t.normal(),
            Primitive::Box(b) => b.normal(point),
            Primitive::Sphere(s) => s.normal(point),
            Primitive::Plane(p) => p.normal(),
        }
    }

    pub fn normal(&self, point: Vector3d) -> Vector3d {
        match self {
            Primitive::Triangle(t) => t.normal(),
            Primitive::Box(b) => b.normal(point),
            Primitive::Sphere(s) => s.normal(point),
            Primitive::Plane(p) => p.normal(),
        }
    }
    
    pub fn center(&self) -> Vector3d {
        match self {
            Primitive::Triangle(t) => t.center(),
            Primitive::Box(b) => b.center(),
            Primitive::Sphere(s) => s.center,
            Primitive::Plane(p) => p.origin,
        }
    }

    pub fn box3d(&self) -> Box3d {
        match self {
            Primitive::Triangle(t) => t.box3d(),
            Primitive::Box(b) => *b,
            Primitive::Sphere(s) => Box3d::new(
                s.center.x - s.radius, s.center.x + s.radius,
                s.center.y - s.radius, s.center.y + s.radius,
                s.center.z - s.radius, s.center.z + s.radius,
            ),
            Primitive::Plane(p) => {
                let (lx, ly) = p.local_axes();
                let half_x = lx * (p.x_size / 2.0);
                let half_y = ly * (p.y_size / 2.0);

                let corners = [
                    p.origin + half_x + half_y,
                    p.origin + half_x - half_y,
                    p.origin - half_x + half_y,
                    p.origin - half_x - half_y,
                ];

                let mut xmin = f32::INFINITY;
                let mut xmax = f32::NEG_INFINITY;
                let mut ymin = f32::INFINITY;
                let mut ymax = f32::NEG_INFINITY;
                let mut zmin = f32::INFINITY;
                let mut zmax = f32::NEG_INFINITY;

                for c in &corners {
                    if c.x < xmin { xmin = c.x; }
                    if c.x > xmax { xmax = c.x; }
                    if c.y < ymin { ymin = c.y; }
                    if c.y > ymax { ymax = c.y; }
                    if c.z < zmin { zmin = c.z; }
                    if c.z > zmax { zmax = c.z; }
                }

                Box3d::new(xmin, xmax, ymin, ymax, zmin, zmax)
            }
        }
    }
}

pub struct BVH {
    pub tree: TreeBranch,
    pub primitives: Vec<Primitive>,
}

#[derive(Clone, Debug)]
pub struct BV {
    pub box3d: Box3d,
    pub primitives: Option<Vec<Primitive>>,
}

impl BV {
    pub fn new(box3d: Box3d) -> BV {
        BV { box3d, primitives: None }
    }
}

impl BVH {
    pub fn new(tree: TreeBranch, primitives: Vec<Primitive>) -> BVH {
        BVH { tree, primitives }
    }

    pub fn build_section(section: BV) -> (BV, BV) {
        let x = section.box3d.xmax - section.box3d.xmin;
        let y = section.box3d.ymax - section.box3d.ymin;
        let z = section.box3d.zmax - section.box3d.zmin;

        let (mut bv1, mut bv2, axis) = match (x, y, z) {
            (a, b, c) if a >= b && a >= c => {
                let center = (section.box3d.xmin + section.box3d.xmax) / 2.0;
                let box1 = Box3d::new(center, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, center, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                (BV::new(box1), BV::new(box2), 'x')
            }
            (_, b, c) if b >= c => {
                let center = (section.box3d.ymin + section.box3d.ymax) / 2.0;
                let box1 = Box3d::new(section.box3d.xmin, section.box3d.xmax, center, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, center, section.box3d.zmin, section.box3d.zmax);
                (BV::new(box1), BV::new(box2), 'y')
            }
            (_, _, _) => {
                let center = (section.box3d.zmin + section.box3d.zmax) / 2.0;
                let box1 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, center, section.box3d.zmax);
                let box2 = Box3d::new(section.box3d.xmin, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, center);
                (BV::new(box1), BV::new(box2), 'z')
            }
        };

        if let Some(ref primitives) = section.primitives {
            let mut list1 = Vec::new();
            let mut list2 = Vec::new();

            let center = match axis {
                'x' => (section.box3d.xmin + section.box3d.xmax) / 2.0,
                'y' => (section.box3d.ymin + section.box3d.ymax) / 2.0,
                'z' => (section.box3d.zmin + section.box3d.zmax) / 2.0,
                _ => unreachable!(),
            };

            for primitive in primitives {
                let val = match axis {
                    'x' => primitive.center().x,
                    'y' => primitive.center().y,
                    'z' => primitive.center().z,
                    _ => unreachable!(),
                };

                if val >= center {
                    list1.push(primitive.clone());
                } else {
                    list2.push(primitive.clone());
                }
            }

            bv1.primitives = Some(list1);
            bv2.primitives = Some(list2);
        }

        (bv1, bv2)
    }

    fn build_node(node: &mut TreeBranch, threshold: usize) {
        let primitives_len = match &node.data.primitives {
            Some(prims) => prims.len(),
            None => 0,
        };

        if primitives_len <= threshold {
            return;
        }

        let (bv1, bv2) = Self::build_section(node.data.clone());

        let mut left_branch = TreeBranch::new(bv2);
        let mut right_branch = TreeBranch::new(bv1);

        let left_len = left_branch.data.primitives.as_ref().map_or(0, |p| p.len());
        let right_len = right_branch.data.primitives.as_ref().map_or(0, |p| p.len());
        if left_len == primitives_len || right_len == primitives_len {
            return;
        }

        Self::build_node(&mut left_branch, threshold);
        Self::build_node(&mut right_branch, threshold);

        node.left = Some(Box::new(left_branch) as Box<dyn std::any::Any>);
        node.right = Some(Box::new(right_branch) as Box<dyn std::any::Any>);

        node.data.primitives = None;
    }

    pub fn build(&mut self) {
        let mut xmin = 0.0;
        let mut xmax = 1.0;
        let mut ymin = 0.0;
        let mut ymax = 1.0;
        let mut zmin = 0.0;
        let mut zmax = 1.0;

        for primitive in &self.primitives {
            let bounding = primitive.box3d();
            if bounding.xmin < xmin { xmin = bounding.xmin }
            if bounding.xmax > xmax { xmax = bounding.xmax }
            if bounding.ymin < ymin { ymin = bounding.ymin }
            if bounding.ymax > ymax { ymax = bounding.ymax }
            if bounding.zmin < zmin { zmin = bounding.zmin }
            if bounding.zmax > zmax { zmax = bounding.zmax }
        }

        let scene_box = Box3d::new(xmin, xmax, ymin, ymax, zmin, zmax);
        self.tree = TreeBranch::new(BV::new(scene_box));
        self.tree.data.primitives = Option::from(self.primitives.clone());
        let threshold = 5;

        Self::build_node(&mut self.tree, threshold);
    }
}
