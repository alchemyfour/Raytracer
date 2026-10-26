use crate::vector3d::Vector3d;

#[derive(Clone)]
pub struct ByteBuffer {
    pub x: usize,
    pub y: usize,
    pub elements: Vec<u8>,
}

impl ByteBuffer {
    pub fn new(x: usize, y: usize) -> ByteBuffer {
        let zeros = vec![0u8; x * y];
        ByteBuffer { x, y, elements: zeros }
    }

    // Changed to &self so it doesn't consume the buffer
    pub fn index(&self, x: usize, y: usize) -> u8 {
        self.elements[y * self.x + x]
    }

    // Changed to &mut self
    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        self.elements[y * self.x + x] = value;
    }

    pub fn increment(&mut self, x: usize, y: usize) {
        let idx = y * self.x + x;
        self.elements[idx] = self.elements[idx].wrapping_add(1);
    }
}

pub struct ColorBuffer {
    pub x: usize,
    pub y: usize,
    pub r: ByteBuffer,
    pub g: ByteBuffer,
    pub b: ByteBuffer,
}

impl ColorBuffer {
    pub fn new(x: usize, y: usize) -> ColorBuffer {
        ColorBuffer {
            x, y,
            r: ByteBuffer::new(x, y),
            g: ByteBuffer::new(x, y),
            b: ByteBuffer::new(x, y)
        }
    }

    pub fn write(&mut self, x: usize, y: usize, color: Vector3d) {
        self.r.write(x, y, (color.x.clamp(0.0, 1.0) * 255.0) as u8);
        self.g.write(x, y, (color.y.clamp(0.0, 1.0) * 255.0) as u8);
        self.b.write(x, y, (color.z.clamp(0.0, 1.0) * 255.0) as u8);
    }

    pub fn index(&self, x: usize, y: usize) -> Vector3d {

        Vector3d::new(
            (self.r.index(x, y) as f32) / 255.0,
            (self.g.index(x, y) as f32) / 255.0,
            (self.b.index(x, y) as f32) / 255.0
        )
    }

    pub fn lerp(&mut self, x: usize, y: usize, val: Vector3d, factor: f32) {
        self.r.write(x, y, ((val.x.clamp(0.0, 1.0) * 255.0)*factor+(self.r.index(x, y) as f32)*(1.0-factor)) as u8);
        self.g.write(x, y, ((val.y.clamp(0.0, 1.0) * 255.0)*factor+(self.g.index(x, y) as f32)*(1.0-factor)) as u8);
        self.b.write(x, y, ((val.z.clamp(0.0, 1.0) * 255.0)*factor+(self.b.index(x, y) as f32)*(1.0-factor)) as u8);
    }
}
