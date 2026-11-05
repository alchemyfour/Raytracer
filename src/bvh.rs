use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::tri::Triangle3d;
use crate::vector3d::{Vector3d, Plane, Sphere};

#[derive(Clone, Debug)]
pub enum Primitive {
    Triangle(Triangle3d),
    Box(Box3d),
    Sphere(Sphere),
}

impl Primitive {

    pub fn color(&self, point: Vector3d) -> Vector3d {
        //Vector3d { x: 0.5, y: 0.5, z: 0.5}
        match self {
            Primitive::Triangle(t) => t.color,
            Primitive::Box(b) => b.normal(point),
            Primitive::Sphere(s) => s.color,
        }
    }

    pub fn metallic(&self, point: Vector3d) -> f32 {
        //Vector3d { x: 0.5, y: 0.5, z: 0.5}
        match self {
            Primitive::Triangle(t) => t.metallic,
            Primitive::Box(b) => 0.0,
            Primitive::Sphere(s) => s.metallic,
        }
    }

    pub fn roughness(&self, point: Vector3d) -> f32 {
        //Vector3d { x: 0.5, y: 0.5, z: 0.5}
        match self {
            Primitive::Triangle(t) => t.roughness,
            Primitive::Box(b) => 0.02,
            Primitive::Sphere(s) => s.roughness,
        }
    }

    pub fn normal(&self, point: Vector3d) -> Vector3d {
        match self {
            Primitive::Triangle(t) => t.normal(),
            Primitive::Box(b) => b.normal(point),
            Primitive::Sphere(s) => s.normal(point),
        }
    }
    
    pub fn center(&self) -> Vector3d {
        match self {
            Primitive::Triangle(t) => t.center(),
            Primitive::Box(b) => b.center(),
            Primitive::Sphere(s) => s.center,
        }
    }

    pub fn box3d(&self) -> Box3d { // deprecated since... A plane is just two triangles?
        match self {
            Primitive::Triangle(t) => t.box3d(),
            Primitive::Box(b) => *b,
            Primitive::Sphere(s) => Box3d::new(
                s.center.x - s.radius, s.center.x + s.radius,
                s.center.y - s.radius, s.center.y + s.radius,
                s.center.z - s.radius, s.center.z + s.radius,
            ),
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

    // Helper to calculate a tight, padded bounding box for a list of primitives
    pub fn calculate_bounds(primitives: &[Primitive]) -> Box3d {
        if primitives.is_empty() {
            return Box3d::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        }

        let mut xmin = f32::INFINITY;
        let mut xmax = f32::NEG_INFINITY;
        let mut ymin = f32::INFINITY;
        let mut ymax = f32::NEG_INFINITY;
        let mut zmin = f32::INFINITY;
        let mut zmax = f32::NEG_INFINITY;

        for p in primitives {
            let bounding = p.box3d();
            if bounding.xmin < xmin { xmin = bounding.xmin }
            if bounding.xmax > xmax { xmax = bounding.xmax }
            if bounding.ymin < ymin { ymin = bounding.ymin }
            if bounding.ymax > ymax { ymax = bounding.ymax }
            if bounding.zmin < zmin { zmin = bounding.zmin }
            if bounding.zmax > zmax { zmax = bounding.zmax }
        }

        // Epsilon padding prevents float-precision misses on perfectly flat 2D planes
        let epsilon = 0.01;
        Box3d::new(
            xmin - epsilon, xmax + epsilon,
            ymin - epsilon, ymax + epsilon,
            zmin - epsilon, zmax + epsilon
        )
    }

    pub fn build_section(mut section: BV) -> (BV, BV) {
        let x = section.box3d.xmax - section.box3d.xmin;
        let y = section.box3d.ymax - section.box3d.ymin;
        let z = section.box3d.zmax - section.box3d.zmin;

        let axis = match (x, y, z) {
            (a, b, c) if a >= b && a >= c => 'x',
            (_, b, c) if b >= c => 'y',
            _ => 'z',
        };

        let mut primitives = section.primitives.take().unwrap_or_default();

        // Object Split: Sort primitives by their centroid along the longest axis
        primitives.sort_by(|a, b| {
            let val_a = match axis { 'x' => a.center().x, 'y' => a.center().y, 'z' => a.center().z, _ => unreachable!() };
            let val_b = match axis { 'x' => b.center().x, 'y' => b.center().y, 'z' => b.center().z, _ => unreachable!() };
            val_a.partial_cmp(&val_b).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Split the list exactly in half
        let mid = primitives.len() / 2;
        let list1 = primitives[mid..].to_vec();
        let list2 = primitives[..mid].to_vec();

        // Calculate the TRUE bounds of the primitives in each half
        let box1 = Self::calculate_bounds(&list1);
        let box2 = Self::calculate_bounds(&list2);

        let mut bv1 = BV::new(box1);
        let mut bv2 = BV::new(box2);

        bv1.primitives = Some(list1);
        bv2.primitives = Some(list2);

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

        Self::build_node(&mut left_branch, threshold);
        Self::build_node(&mut right_branch, threshold);

        node.left = Some(Box::new(left_branch) as Box<dyn std::any::Any + Send + Sync>);
        node.right = Some(Box::new(right_branch) as Box<dyn std::any::Any + Send + Sync>);

        node.data.primitives = None;
    }

    pub fn build(&mut self) {
        // Calculate the root bounding box tightly using the new helper
        let scene_box = Self::calculate_bounds(&self.primitives);

        self.tree = TreeBranch::new(BV::new(scene_box));
        self.tree.data.primitives = Option::from(self.primitives.clone());
        let threshold = 5;

        Self::build_node(&mut self.tree, threshold);
    }
}

