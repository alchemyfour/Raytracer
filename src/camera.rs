use crate::bvh::BVH;
use crate::intersection3d::Intersection3d;
use crate::ray::Raycast;
use crate::pixel::Pixel;
use crate::vector3d::{Ray3d, Vector3d};

struct Camera {
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

    pub fn fire(&mut self) -> Vec<Pixel>{
        let mut resolve = Vec::new();
        let mut misses: Vec<Pixel> = Vec::new();
        for y in 0..self.resolutiony as usize { // push all the raycasts to the resolve vec
            for x in 0..self.resolutionx as usize {
                resolve.push(Pixel::new(
                    Raycast::new( Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh),
                    x as f32,
                    y as f32
                )

                )
            }
        }

        for mut pixel in resolve.clone() {
            let resolved = pixel.raycast.resolve();
            match resolved {
                None => {
                    misses.push(pixel);
                },
                Some((hit_point, primitive)) => {
                    let normal = primitive.normal(hit_point);
                    let intersection = Intersection3d::new(pixel.raycast.ray3d.direction, hit_point, normal);
                    pixel.raycast = Raycast::new(Ray3d::new(hit_point, pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(normal)) * normal), pixel.raycast.bvh);
                    pixel.intersections.push(intersection);
                    resolve.push(pixel);
                }
            }
        }

        for mut pixel in misses.clone() {
            if pixel.intersections.is_empty() {
                pixel.color = Vector3d::new(0.0, 0.0, 0.0);
            } else {
                pixel.color = Vector3d::new(0.0, 0.0, 0.0);
            }
        }

        misses

        /*
        should iteratively fire every X and Y ray by taking the direction and accounting for FOV
        takes each one of these rays and sticks ones that hit into a hits Vec<Pixel>, and a misses Vec<Pixel>
        once done, refires all pixels in the hits section by calculating their reflection (doesn't account for roughness ATM)
        adds hits and misses to their respective Vec
        repeats until all hits are in the misses field, or repeats more than some set ammount of times (we'll say 10)
        returns a vec with all pixels in it, to have their colors computed seprately based on their hits
        */
    }
}
