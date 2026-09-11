use std::any::Any;
use crate::bvh::BVH;

pub struct TreeBranch {
    pub left: Box<dyn Any>,
    pub right: Box<dyn Any>
}

