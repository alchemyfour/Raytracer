use crate::bintree::TreeBranch;
use crate::box3d::Box3d;
use crate::tri::{Triangle3d, Triangles3d};

enum Primitive {
    Triangle(Triangle3d),
    //sphere
    Box3d(Box3d)
}

#[derive()]
pub struct BVH {
    pub tree: TreeBranch,
    pub primitives: Vec<Triangle3d>, // Properly implement center function later, for now primitives is just Triangle3d
}

pub struct BV {
    pub box3d: Box3d,
    pub primitives: Vec<Triangle3d> // Same as last comment

}

impl BV {
    pub fn new(box3d: Box3d, primitives: Vec<Triangle3d>) -> BV {
        BV { box3d, primitives }
    }

}

impl BVH {
    pub fn new(tree: TreeBranch, primitives: Vec<Primitive>) -> BVH {
        BVH { tree, primitives: vec![] }
    }
    pub fn build(&mut self) {
        // find the smallest possible shape that all primitives can be contained in
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
        self.tree = TreeBranch::new(BV::new(scene_box, self.primitives())); // make the root tree branch a BV with all primitives
        let mut current_branch = &self.tree;
        let triangle_threshold = 5;
        while true {
            break
        }


        // 1. iterate through each branch (starting from the left) and split box along the longest axis, favouring x, y, then z respectfully
        // 2. sort through every triangle and append it onto either the left or right primitive vec (left being < coord along the split axis than right)
        // 3. check each, if it contains no triangles or less than the triangle threshold, then move rightwards. Otherwise move upwards until moving rightwards is possible
        // 4. if unable to move rightwards, move up.






        // split primitives into 2 boxes along longest axis
        // continue until either subdivision does not contain any primitives, or contains just one
        // cull tree seprately

    }

}