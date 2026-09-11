use crate::bintree::TreeBranch;

pub struct BVH {
    tree: TreeBranch
}

impl BVH {
    pub fn new(tree: TreeBranch) -> BVH {
        BVH {tree}
    }
}