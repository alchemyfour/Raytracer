use crate::bintree::BinaryTree;

pub struct BVH {
    tree: BinaryTree
}

impl BVH {
    pub fn new(tree: BinaryTree) -> BVH {
        BVH {tree}
    }
}