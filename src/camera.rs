use crate::bvh::BVH;
use crate::intersection3d::Intersection3d;
use crate::ray::Raycast;
use crate::pixel::Pixel;
use crate::vector3d::{Ray3d, Vector3d};

pub struct Camera {
    origin: Vector3d,
    fovy: f32,
    fovx: f32,
    resolutionx: f32,
    resolutiony: f32,
    dir: Vector3d,
    bvh: BVH,
}

impl Camera {
    pub fn new(origin: Vector3d, fovy: f32, fovx: f32, resolutionx: f32, resolutiony: f32, dir: Vector3d, bvh: BVH) -> Self {
        Camera {origin, fovy, fovx, resolutionx, resolutiony, dir, bvh}
    }

    pub fn ray_direction(&self, x: f32, y: f32) -> Vector3d {
        let w = self.dir.normalize();
        let helper = if w.x.abs() > 0.9 {
            Vector3d::new(0.0, 1.0, 0.0)
        } else {
            Vector3d::new(1.0, 0.0, 0.0)
        };
        let u = w.cross(helper).normalize();
        let v = w.cross(u).normalize();

        let px = ((x + 0.5) / self.resolutionx) * 2.0 - 1.0;
        let py = 1.0 - ((y + 0.5) / self.resolutiony) * 2.0;

        let scale_x = (self.fovx / 2.0).tan();
        let scale_y = (self.fovy / 2.0).tan();

        let dir_x = u * (px * scale_x) as f32;
        let dir_y = v * (py * scale_y) as f32;

        (w + dir_x + dir_y).normalize()
    }

    pub fn fire(&mut self) -> Vec<Pixel<'_>> {
        let mut resolve = Vec::new();
        let mut misses: Vec<Pixel> = Vec::new();
        for y in 0..self.resolutiony as usize { // push all the raycasts to the resolve vec
            for x in 0..self.resolutionx as usize {
                resolve.push(Pixel::new(
                    Raycast::new( Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh),
                    x as f32,
                    y as f32
                ))
            }
        }

        let mut bounce_limit = 10;
        while !resolve.is_empty() && bounce_limit > 0 {
            let mut next_resolve = Vec::new();
            for mut pixel in resolve {
                let resolved = pixel.raycast.resolve();
                match resolved {
                    None => {
                        misses.push(pixel);
                    }
                    Some((hit_point, primitive)) => {
                        let normal = primitive.normal(hit_point);
                        let intersection = Intersection3d::new(pixel.raycast.ray3d.direction, hit_point, normal, pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(normal)) * normal, Vector3d::new(0.5, 0.5, 0.5));
                        pixel.raycast = Raycast::new(
                            Ray3d::new(hit_point, pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(normal)) * normal),
                            pixel.raycast.bvh
                        );
                        pixel.intersections.push(intersection);
                        pixel.bounces += 1.0;
                        if pixel.bounces < 10.0 {
                            next_resolve.push(pixel);
                        } else {
                            misses.push(pixel);
                        }
                    }
                }
            }
            resolve = next_resolve;
            bounce_limit -= 1;
        }

        // Apply background/hit coloring to the pixels in misses

        for pixel in &mut misses {
            let sky_color = Vector3d::new(
                pixel.raycast.ray3d.direction.x*4.00_f32.clamp(0.01, 0.5),
                pixel.raycast.ray3d.direction.x*8.00_f32.clamp(0.1, 0.8),
                pixel.raycast.ray3d.direction.x*12.0_f32.clamp(0.1, 0.98)
            );
            if pixel.intersections.is_empty() {
                pixel.color = sky_color
                // skycolor formula
            } else {
                // take the final bounce and figure out how much it pointed skywards
                let sky_factor = pixel.raycast.ray3d.direction.x;
                let mut basecolor = Vector3d::new(0.5, 0.5, 0.5);
                for intersection in &pixel.intersections {
                    basecolor = intersection.color + basecolor;
                }
                let finalcolor = (basecolor+sky_color*sky_factor)/(pixel.intersections.len()+1) as f32;

                pixel.color = finalcolor;
            }
        }

        misses
    }
}