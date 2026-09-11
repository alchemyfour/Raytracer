use std::any::Any;
use crate::bvh::{BV, BVH};

pub struct TreeBranch {
    pub left: Option<Box<dyn Any>>,
    pub right: Option<Box<dyn Any>>,
    pub parent: Option<Box<TreeBranch>>,
    pub data: BV
}

impl TreeBranch {
    pub fn new(data: BV) -> TreeBranch {
        TreeBranch {data, parent: None, left: None, right: None}
    }
    
}

