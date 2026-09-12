use std::ptr::null;
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
    pub primitives: Option<Vec<Triangle3d>> // Same as last comment

}

impl BV {
    pub fn new(box3d: Box3d) -> BV {
        BV { box3d, primitives: None }
    }

}

impl BVH {
    pub fn new(tree: TreeBranch, primitives: Vec<Primitive>) -> BVH {
        BVH { tree, primitives: vec![] }
    }

    pub fn build_section(section: BV) {
        // if empty then give up
        if section.primitives.expect("Fuck you").len() > 0 {

        }
        let x = section.box3d.xmax - section.box3d.xmin;
        let y = section.box3d.ymax - section.box3d.ymin;
        let z = section.box3d.zmax - section.box3d.zmin;

        // find the bounding box's longest axis
        match (x, y, z) {

            (a, b, c) if a >= b && a >= c => {
                // X is biggest or there's a tie
                let center = (section.box3d.xmin + section.box3d.xmax)/2.0;
                let box1: Box3d = Box3d::new(center, section.box3d.xmax, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let mut bv1 = BV::new(box1);
                let box2: Box3d = Box3d::new( section.box3d.xmin, center, section.box3d.ymin, section.box3d.ymax, section.box3d.zmin, section.box3d.zmax);
                let mut bv2 = BV::new(box2);
                // Create two new boxes that have their min/max set at the center respectfully
                for triangle in section.primitives.iter() { // THIS SHOULDN'T PASS JUST TRIANGLES BUT IT DOES
                    if triangle.center().x > center { // This should probably be a match but I'm too lazy
                        bv1.primitives.unwrap().append(triangle);
                    }
                    if triangle.center().x < center {

                    }
                }




            }

            (_, b, c) if b >= c => {
                // Y is biggest or tied with Z

            }

            (_, _, c) => {
                // Z is biggest

            }
        }


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
        self.tree = TreeBranch::new(BV::new(scene_box)); // make the root tree branch a BV with all primitives
        self.tree.data.primitives = Option::from(self.primitives.clone());
        let mut current_branch = &self.tree;
        let triangle_threshold = 5;


        // TO BE IMPLEMENTED AS A RECURSIVE FUNCTION
        // 1. go through each branch (starting from the left) and split box along the longest axis, favouring x, y, then z respectfully
        // 2. sort through every triangle and append it onto either the left or right primitive vec (left being < coord along the split axis than right)
        // 3. check each, if it contains no triangles or less than the triangle threshold, then move rightwards. Otherwise move upwards until moving rightwards is possible
        // 4. if unable to move rightwards, move up.






        // split primitives into 2 boxes along longest axis
        // continue until either subdivision does not contain any primitives, or contains just one
        // cull tree seprately

    }

}