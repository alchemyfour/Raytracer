use crate::vector3d::Vector3d;

pub struct ColorBuffer {
    pub x: usize,
    pub y: usize,
    pub elements: Vec<Vector3d>,
}

impl ColorBuffer {
    pub fn new(x: usize, y: usize, elements: Vec<Vector3d>) -> ColorBuffer {
        ColorBuffer {x, y, elements}
        // not panicking if 0 because I'm lazy and it's an edge case
    }

    pub fn index(self, x: usize, y: usize) -> Vector3d {
        self.elements[y * self.x + x]
    }
}