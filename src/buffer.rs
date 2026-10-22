use pixels::wgpu::wgc::command::AttachmentErrorLocation::Color;
use crate::vector3d::Vector3d;

pub struct ByteBuffer {
    pub x: usize,
    pub y: usize,
    pub elements: Vec<u8>,

}

impl ByteBuffer {
    pub fn new(x: usize, y: usize) -> ByteBuffer {
        let zeros = vec![0u8; x*y];
        ByteBuffer {x, y, elements:zeros}
    }

    pub fn index(self, x: usize, y: usize) -> u8 {
        self.elements[y * self.x + x]
    }

    pub fn write(mut self, x: usize, y: usize, value: u8) {
        self.elements[y * self.x + x] = value;
    }
}

pub struct ColorBuffer {
    /*
    Avoids writing a vec of pointers to the stack (As opposed to just storing the Vector3d class)
     */
    pub x: usize,
    pub y: usize,
    pub r: ByteBuffer,
    pub g: ByteBuffer,
    pub b: ByteBuffer,
}

impl ColorBuffer {
    pub fn new(x: usize, y: usize) -> ColorBuffer {
        ColorBuffer {x, y, r: ByteBuffer::new(x,y), g: ByteBuffer::new(x,y), b: ByteBuffer::new(x,y) }
    }

    pub fn write(mut self, x: usize, y: usize, color: Vector3d) {
        self.r.write(x, y, (color.x*255.0)as u8);
        self.g.write(x, y, (color.y*255.0)as u8);
        self.b.write(x, y, (color.z*255.0)as u8);
    }

    pub fn index(self, x: usize, y: usize) -> Vector3d {
        Vector3d::new((self.r.index(x, y) / 255) as f32, (self.g.index(x, y) / 255) as f32, (self.b.index(x, y) / 255) as f32)
    }
}