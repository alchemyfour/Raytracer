use std::ops::{Add, BitXor, Mul};
use crate::bvh::BVH;
use crate::intersection3d::Intersection3d;
use crate::ray::Raycast;
use crate::pixel::Pixel;
use crate::vector3d::{Ray3d, Vector3d};
use rand::{random, RngExt};
use crate::light::Light;

pub struct Camera {
    origin: Vector3d,
    fovy: f32,
    fovx: f32,
    resolutionx: f32,
    resolutiony: f32,
    dir: Vector3d,
    bvh: BVH,
    pub lights: Vec<Light>
}

impl Add<f32> for Vector3d {
    type Output = ();

    fn add(self, rhs: f32) -> Self::Output {
        todo!()
    }
}



impl Camera {
    pub fn new(origin: Vector3d, fovy: f32, fovx: f32, resolutionx: f32, resolutiony: f32, dir: Vector3d, bvh: BVH, lights: Vec<Light>) -> Self {
        Camera {origin, fovy, fovx, resolutionx, resolutiony, dir, bvh, lights}
    }

    pub fn ray_direction(&self, x: f32, y: f32) -> Vector3d {
        let w = self.dir.normalize();
        let helper = if w.x.abs() > 0.9 {
            Vector3d::new(0.0, 1.0, 0.0)
        } else {
            Vector3d::new(0.0, 1.0, 0.0)
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
        for x in 0..self.resolutionx as usize { // push all the raycasts to the resolve vec
            for y in 0..self.resolutiony as usize {
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
                        let intersection = Intersection3d::new(pixel.raycast.ray3d.direction, hit_point, normal, pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(normal)) * normal, primitive.color(hit_point), primitive.metallic(hit_point));
                        let rand: Vector3d = Vector3d::new(random::<f32>(), random::<f32>(), random::<f32>());
                        pixel.raycast = Raycast::new(
                            Ray3d::new(hit_point, (pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(normal)) * normal) + (rand)*primitive.roughness(hit_point)),
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

        for pixel in &mut misses { // This ought to be rewritten with the image class but this does work for now
            let sky_change = 0.0;
            let sky_color = Vector3d::new(
                (pixel.raycast.ray3d.direction.y+sky_change)*4.00_f32.clamp(0.01, 0.4),
                (pixel.raycast.ray3d.direction.y+sky_change)*8.00_f32.clamp(0.1, 0.8),
                (pixel.raycast.ray3d.direction.y+sky_change)*12.0_f32.clamp(0.1, 0.98)
            );
            if pixel.intersections.is_empty() {
                pixel.color = sky_color
                // skycolor formula
            } else {
                // take the final bounce and figure out how much it pointed skywards
                let sky_factor = pixel.raycast.ray3d.direction.y;
                let mut basecolor = Vector3d::new(0.0, 0.0, 0.0);
                let mut brightness: f32 = 1.0;
                    for intersection in &pixel.intersections {
                        basecolor = intersection.color + basecolor;
                        brightness = brightness * intersection.color.magnitude();

                }
                let reflection_color = (basecolor+sky_color*sky_factor*1.0)/(pixel.intersections.len() as f32+(1.0/sky_factor)*1.0);
                let albedo: Vector3d = (pixel.intersections[0].color);
                // let cheap_lit: Vector3d = Vector3d::new(pixel.intersections[0].color.x + pixel.intersections[0].normal.x, pixel.intersections[0].color.y + pixel.intersections[0].normal.x, pixel.intersections[0].color.z + pixel.intersections[0].normal.x);
                let cheap_lit = Vector3d::new(pixel.raycast.ray3d.direction.y, pixel.raycast.ray3d.direction.y, pixel.raycast.ray3d.direction.y);
                let reflections: Vector3d = Vector3d::new(pixel.bounces - 1.0, pixel.bounces - 1.0, pixel.bounces - 1.0);
                let final_lerp = sky_factor;
                let final_color = (albedo*(pixel.intersections[0].normal.y+1.0)/2.0);
                let specular = ((pixel.raycast.ray3d.direction - 2.0 * (pixel.raycast.ray3d.direction.dot(pixel.intersections[0].normal)) * pixel.intersections[0].normal).dot(self.dir));
                let metallic = (((basecolor*sky_factor.clamp(0.2,1.0))+sky_color*0.5)/(pixel.bounces + 0.01)) * specular.powf(3.0);
                let metallic_factor = pixel.intersections[0].metallic;
                let mut lit: f32 = 0.0;
                let mut lit_color = Vector3d::new(0.0, 0.0, 0.0);
                for light in self.lights.clone() {
                    for intersection in &pixel.intersections {
                        let light_dir = (light.position - intersection.location).normalize();
                        let light_dist = (light.position - intersection.location).magnitude();
                        let brightness: f32 = ((light_dir.dot(intersection.normal).abs()) * light.brightness)/(light_dist/light.radius.powf(1.0));
                        lit += brightness;
                        lit_color = lit_color + light.color/(light_dist/light.radius.powf(1.0));    
                    }
                }
                if lit_color == Vector3d::new(0.0, 0.0, 0.0) {
                    lit_color = Vector3d::new(1.0, 1.0, 1.0);
                }
                lit_color.normalize();
                let final_vals = ((final_color*specular.clamp(0.01, 1.0) * (1.0 - metallic_factor) + metallic * metallic_factor)* lit);

                // pixel.color = Vector3d::new(final_vals.x * lit_color.x, final_vals.y * lit_color.y, final_vals.z * lit_color.z);
                pixel.color = lit_color;
                // pixel.color = sky_color;
            }
        }

        misses
    }
}