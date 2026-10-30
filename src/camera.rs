use std::ops::{Add, BitXor, Mul};
use crate::buffer::{BitBuffer, ByteBuffer, ColorBuffer};
use crate::bvh::BVH;
use crate::intersection3d::Intersection3d;
use crate::vector3d::{Ray3d, Vector3d};
use crate::light::Light;
use rayon::prelude::*;
use crate::ray::Raycast;

pub struct Camera {
    origin: Vector3d,
    fovy: f32,
    fovx: f32,
    pub(crate) resolutionx: f32,
    pub(crate) resolutiony: f32,
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

    pub fn fire(&mut self, seed: u32) -> ColorBuffer {
        let mut resolve: Vec<Raycast> = Vec::new();
        // let mut misses: Vec<Raycast> = Vec::new(); // Unused in active code

        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                resolve.push(Raycast::new(Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh, x, y, 0, 0.0))
            }
        }

        let mut bounce_buffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut depth_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut metallic_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut color_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut normal_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut reflections_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);

        let mut bounce_limit = 10;

        while !(resolve.is_empty() || bounce_limit <= 0) {
            // Borrow properties to move into the Rayon closure
            let origin = self.origin;
            let lights = &self.lights;

            // Map threads over rays, returning the new rays and buffer updates safely
            let (next_resolve, updates): (Vec<_>, Vec<_>) = resolve.into_par_iter().filter_map(|raycast: Raycast| {
                let resolved = raycast.clone().resolve();

                match resolved {
                    None => None,
                    Some((hit_point, primitive)) => {
                        let normal = primitive.normal(hit_point);
                        let intersection = Intersection3d::new(
                            raycast.ray3d.direction,
                            hit_point,
                            normal,
                            raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal,
                            primitive.color(hit_point),
                            primitive.metallic(hit_point)
                        );

                        let hash = |mut s: u32| -> f32 {
                            s ^= s >> 16;
                            s = s.wrapping_mul(0x7feb352d);
                            s ^= s >> 15;
                            s = s.wrapping_mul(0x846ca68b);
                            s ^= s >> 16;
                            (s as f32) / (u32::MAX as f32)
                        };

                        let x_bits = hit_point.x.to_bits();
                        let y_bits = hit_point.y.to_bits();
                        let z_bits = hit_point.z.to_bits();
                        let b_bits: u32 = seed;

                        let seed_base = x_bits
                            ^ y_bits.wrapping_mul(0x9E3779B9)
                            ^ z_bits.wrapping_mul(0x85EBCA6B)
                            ^ b_bits.wrapping_mul(0xC2B2AE35);

                        let rand: Vector3d = Vector3d::new(
                            hash(seed_base.wrapping_add(1)),
                            hash(seed_base.wrapping_add(2)),
                            hash(seed_base.wrapping_add(3))
                        );

                        let new_raycast = Raycast::new(
                            Ray3d::new(
                                hit_point,
                                (raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal) + (rand * primitive.roughness(hit_point))
                            ),
                            raycast.bvh,
                            raycast.x,
                            raycast.y,
                            raycast.bounces + 1,
                            primitive.metallic(intersection.location)


                        );

                        let mut lit = 0.0;
                        let mut lit_color = Vector3d::new(0.0, 0.0, 0.0);

                        for light in lights.iter() {
                            let light_dir = (light.position - intersection.location).normalize();
                            let light_dist = (light.position - intersection.location).magnitude();
                            let brightness: f32 = ((light_dir.dot(intersection.normal).abs()) * light.brightness) / (light_dist / light.radius.powf(1.0));

                            let check = Raycast::new(Ray3d::new(intersection.location + light_dir, light_dir), raycast.bvh, raycast.x, raycast.y, 0, 0.0);
                            let checked = check.resolve();

                            if checked.is_none() {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(1.0));
                            } else if (checked.unwrap().0 - intersection.location).magnitude() > light_dist + 0.1 {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(1.0));
                            }
                        }

                        let depth_val = ((intersection.location - origin).magnitude() / 4.0) as u8;
                        let final_color = intersection.color * lit;

                        // Package the calculations and the next raycast
                        let update_data = (raycast.x, raycast.y, depth_val, intersection.normal, intersection.color, final_color, raycast.bounces, intersection.location, raycast.metallic);



                        Some((new_raycast, update_data))
                    }
                }
            }).unzip();


            resolve = next_resolve;
            for (x, y, depth, normal, raw_color, final_color, bounces, location, metallic) in updates {
                bounce_buffer.increment(x, y);
                (&mut depth_buffer).write(x, y, depth);
                normal_buffer.write(x, y, normal);

                (&mut reflections_buffer).lerp(x, y, raw_color, 0.0);


                //color_buffer.write(x, y, location/4.0);



                if bounces <= 0 {
                    color_buffer.write(x, y, final_color);
                } else if bounces >= 1 {
                    color_buffer.lerp(x, y, final_color, (0.0 + (metallic *(1.0/bounces as f32))) as f32);
                }


            }

            bounce_limit -= 1;
        }

        let mut alpha = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        alpha.r = bounce_buffer.clone();
        alpha.b = depth_buffer.clone();

        let mut final_buffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        final_buffer.r = depth_buffer.clone();
        final_buffer.g = depth_buffer.clone();
        final_buffer.b = depth_buffer.clone();
        color_buffer
    }

    pub fn render(&mut self, samples: u8) -> ColorBuffer {
        let mut passes = Vec::new();
        for pass in 0..samples {
            passes.push(self.fire(pass as u32));
            println!("{}% done!", ((pass as f32 + 1.0)/samples as f32)*100.0);
        }
        let mut final_buffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                let mut pixel_color = Vector3d::new(0.0, 0.0, 0.0);
                for pass in passes.iter() {
                    pixel_color = pixel_color + pass.index(x, y)
                }
                final_buffer.write(x, y, pixel_color/samples as f32);
            }
        }
        final_buffer
    }

    pub fn pos_fire(&mut self, seed: u32) -> ColorBuffer {
        let mut resolve: Vec<Raycast> = Vec::new();
        // let mut misses: Vec<Raycast> = Vec::new(); // Unused in active code

        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                resolve.push(Raycast::new(Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh, x, y, 0, 0.0))
            }
        }

        let mut bounce_buffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut depth_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut metallic_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut color_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut normal_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut reflections_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);

        let mut bounce_limit = 10;

        while !(resolve.is_empty() || bounce_limit <= 0) {
            // Borrow properties to move into the Rayon closure
            let origin = self.origin;
            let lights = &self.lights;

            // Map threads over rays, returning the new rays and buffer updates safely
            let (next_resolve, updates): (Vec<_>, Vec<_>) = resolve.into_par_iter().filter_map(|raycast: Raycast| {
                let resolved = raycast.clone().resolve();

                match resolved {
                    None => None,
                    Some((hit_point, primitive)) => {
                        let normal = primitive.normal(hit_point);
                        let intersection = Intersection3d::new(
                            raycast.ray3d.direction,
                            hit_point,
                            normal,
                            raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal,
                            primitive.color(hit_point),
                            primitive.metallic(hit_point)
                        );

                        let hash = |mut s: u32| -> f32 {
                            s ^= s >> 16;
                            s = s.wrapping_mul(0x7feb352d);
                            s ^= s >> 15;
                            s = s.wrapping_mul(0x846ca68b);
                            s ^= s >> 16;
                            (s as f32) / (u32::MAX as f32)
                        };

                        let x_bits = hit_point.x.to_bits();
                        let y_bits = hit_point.y.to_bits();
                        let z_bits = hit_point.z.to_bits();
                        let b_bits: u32 = seed;

                        let seed_base = x_bits
                            ^ y_bits.wrapping_mul(0x9E3779B9)
                            ^ z_bits.wrapping_mul(0x85EBCA6B)
                            ^ b_bits.wrapping_mul(0xC2B2AE35);

                        let rand: Vector3d = Vector3d::new(
                            hash(seed_base.wrapping_add(1)),
                            hash(seed_base.wrapping_add(2)),
                            hash(seed_base.wrapping_add(3))
                        );

                        let new_raycast = Raycast::new(
                            Ray3d::new(
                                hit_point,
                                (raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal) + (rand * primitive.roughness(hit_point))
                            ),
                            raycast.bvh,
                            raycast.x,
                            raycast.y,
                            raycast.bounces + 1,
                            primitive.metallic(intersection.location)


                        );

                        let mut lit = 0.0;
                        let mut lit_color = Vector3d::new(0.0, 0.0, 0.0);

                        for light in lights.iter() {
                            let light_dir = (light.position - intersection.location).normalize();
                            let light_dist = (light.position - intersection.location).magnitude();
                            let brightness: f32 = ((light_dir.dot(intersection.normal).abs()) * light.brightness) / (light_dist / light.radius.powf(1.0));

                            let check = Raycast::new(Ray3d::new(intersection.location + light_dir, light_dir), raycast.bvh, raycast.x, raycast.y, 0, 0.0);
                            let checked = check.resolve();

                            if checked.is_none() {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(2.0));
                            } else if (checked.unwrap().0 - intersection.location).magnitude() > light_dist + 0.1 {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(2.0));
                            }
                        }

                        let depth_val = ((intersection.location - origin).magnitude() / 4.0) as u8;
                        let final_color = intersection.color * lit;

                        // Package the calculations and the next raycast
                        let update_data = (raycast.x, raycast.y, depth_val, intersection.normal, intersection.color, final_color, raycast.bounces, intersection.location, raycast.metallic);



                        Some((new_raycast, update_data))
                    }
                }
            }).unzip();


            resolve = next_resolve;
            for (x, y, depth, normal, raw_color, final_color, bounces, location, metallic) in updates {
                bounce_buffer.increment(x, y);
                (&mut depth_buffer).write(x, y, depth);
                normal_buffer.write(x, y, normal);

                (&mut reflections_buffer).lerp(x, y, raw_color, 0.0);

                if bounces <= 0 {
                    color_buffer.write(x, y, location/4.0);
                } else if bounces >= 1 {
                    color_buffer.lerp(x, y, location/4.0, (0.0 + (metallic *(1.0/bounces as f32))) as f32);
                }


            }

            bounce_limit -= 1;
        }
        color_buffer
    }

    pub fn selective_fire(&mut self, seed: u32, mask: BitBuffer) -> ColorBuffer {
        let mut resolve: Vec<Raycast> = Vec::new();
        // let mut misses: Vec<Raycast> = Vec::new(); // Unused in active code

        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                if mask.index(x, y) {
                    resolve.push(Raycast::new(Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh, x, y, 0, 0.0))
                }
            }
        }

        let mut bounce_buffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut depth_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut metallic_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut color_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut normal_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut reflections_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);

        let mut bounce_limit = 10;

        while !(resolve.is_empty() || bounce_limit <= 0) {
            // Borrow properties to move into the Rayon closure
            let origin = self.origin;
            let lights = &self.lights;

            // Map threads over rays, returning the new rays and buffer updates safely
            let (next_resolve, updates): (Vec<_>, Vec<_>) = resolve.into_par_iter().filter_map(|raycast: Raycast| {
                let resolved = raycast.clone().resolve();

                match resolved {
                    None => None,
                    Some((hit_point, primitive)) => {
                        let normal = primitive.normal(hit_point);
                        let intersection = Intersection3d::new(
                            raycast.ray3d.direction,
                            hit_point,
                            normal,
                            raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal,
                            primitive.color(hit_point),
                            primitive.metallic(hit_point)
                        );

                        let hash = |mut s: u32| -> f32 {
                            s ^= s >> 16;
                            s = s.wrapping_mul(0x7feb352d);
                            s ^= s >> 15;
                            s = s.wrapping_mul(0x846ca68b);
                            s ^= s >> 16;
                            (s as f32) / (u32::MAX as f32)
                        };

                        let x_bits = hit_point.x.to_bits();
                        let y_bits = hit_point.y.to_bits();
                        let z_bits = hit_point.z.to_bits();
                        let b_bits: u32 = seed;

                        let seed_base = x_bits
                            ^ y_bits.wrapping_mul(0x9E3779B9)
                            ^ z_bits.wrapping_mul(0x85EBCA6B)
                            ^ b_bits.wrapping_mul(0xC2B2AE35);

                        let rand: Vector3d = Vector3d::new(
                            hash(seed_base.wrapping_add(1)),
                            hash(seed_base.wrapping_add(2)),
                            hash(seed_base.wrapping_add(3))
                        );

                        let new_raycast = Raycast::new(
                            Ray3d::new(
                                hit_point,
                                (raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal) + (rand * primitive.roughness(hit_point))
                            ),
                            raycast.bvh,
                            raycast.x,
                            raycast.y,
                            raycast.bounces + 1,
                            primitive.metallic(intersection.location)


                        );

                        let mut lit = 0.0;
                        let mut lit_color = Vector3d::new(0.0, 0.0, 0.0);

                        for light in lights.iter() {
                            let light_dir = (light.position - intersection.location).normalize();
                            let light_dist = (light.position - intersection.location).magnitude();
                            let brightness: f32 = ((light_dir.dot(intersection.normal).abs()) * light.brightness) / (light_dist / light.radius.powf(1.0));

                            let check = Raycast::new(Ray3d::new(intersection.location + light_dir, light_dir), raycast.bvh, raycast.x, raycast.y, 0, 0.0);
                            let checked = check.resolve();

                            if checked.is_none() {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(1.0));
                            } else if (checked.unwrap().0 - intersection.location).magnitude() > light_dist + 0.1 {
                                lit += brightness;
                                lit_color = lit_color + light.color / (light_dist / light.radius.powf(1.0));
                            }
                        }

                        let depth_val = ((intersection.location - origin).magnitude() / 4.0) as u8;
                        let final_color = intersection.color * lit;

                        // Package the calculations and the next raycast
                        let update_data = (raycast.x, raycast.y, depth_val, intersection.normal, intersection.color, final_color, raycast.bounces, intersection.location, raycast.metallic);



                        Some((new_raycast, update_data))
                    }
                }
            }).unzip();


            resolve = next_resolve;
            for (x, y, depth, normal, raw_color, final_color, bounces, location, metallic) in updates {
                bounce_buffer.increment(x, y);
                (&mut depth_buffer).write(x, y, depth);
                normal_buffer.write(x, y, normal);

                (&mut reflections_buffer).lerp(x, y, raw_color, 0.0);


                //color_buffer.write(x, y, location/4.0);



                if bounces <= 0 {
                    color_buffer.write(x, y, final_color);
                } else if bounces >= 1 {
                    color_buffer.lerp(x, y, final_color, (0.0 + (metallic *(1.0/bounces as f32))) as f32);
                }


            }

            bounce_limit -= 1;
        }
        
        color_buffer
    }
    pub fn roughness_fire(&mut self, seed: u32) -> ByteBuffer {
        let mut resolve: Vec<Raycast> = Vec::new();
        // let mut misses: Vec<Raycast> = Vec::new(); // Unused in active code

        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                resolve.push(Raycast::new(Ray3d::new(self.origin, self.ray_direction(x as f32, y as f32)), &self.bvh, x, y, 0, 0.0))
            }
        }

        let mut bounce_buffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut depth_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut metallic_buffer: ByteBuffer = ByteBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut color_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut normal_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        let mut reflections_buffer: ColorBuffer = ColorBuffer::new(self.resolutionx as usize, self.resolutiony as usize);

        let mut bounce_limit = 10;

        while !(resolve.is_empty() || bounce_limit <= 0) {
            // Borrow properties to move into the Rayon closure
            let origin = self.origin;
            let lights = &self.lights;

            // Map threads over rays, returning the new rays and buffer updates safely
            let (next_resolve, updates): (Vec<_>, Vec<_>) = resolve.into_par_iter().filter_map(|raycast: Raycast| {
                let resolved = raycast.clone().resolve();

                match resolved {
                    None => None,
                    Some((hit_point, primitive)) => {
                        let normal = primitive.normal(hit_point);
                        let intersection = Intersection3d::new(
                            raycast.ray3d.direction,
                            hit_point,
                            normal,
                            raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal,
                            primitive.color(hit_point),
                            primitive.roughness(hit_point)
                        );

                        let hash = |mut s: u32| -> f32 {
                            s ^= s >> 16;
                            s = s.wrapping_mul(0x7feb352d);
                            s ^= s >> 15;
                            s = s.wrapping_mul(0x846ca68b);
                            s ^= s >> 16;
                            (s as f32) / (u32::MAX as f32)
                        };

                        let x_bits = hit_point.x.to_bits();
                        let y_bits = hit_point.y.to_bits();
                        let z_bits = hit_point.z.to_bits();
                        let b_bits: u32 = seed;

                        let seed_base = x_bits
                            ^ y_bits.wrapping_mul(0x9E3779B9)
                            ^ z_bits.wrapping_mul(0x85EBCA6B)
                            ^ b_bits.wrapping_mul(0xC2B2AE35);

                        let rand: Vector3d = Vector3d::new(
                            hash(seed_base.wrapping_add(1)),
                            hash(seed_base.wrapping_add(2)),
                            hash(seed_base.wrapping_add(3))
                        );

                        let new_raycast = Raycast::new(
                            Ray3d::new(
                                hit_point,
                                (raycast.ray3d.direction - 2.0 * (raycast.ray3d.direction.dot(normal)) * normal) + (rand * primitive.roughness(hit_point))
                            ),
                            raycast.bvh,
                            raycast.x,
                            raycast.y,
                            raycast.bounces + 1,
                            primitive.metallic(intersection.location)


                        );
                        

                        // Package the calculations and the next raycast
                        let update_data = (raycast.x, raycast.y, 0, intersection.normal, intersection.color, raycast.bounces, intersection.location, raycast.metallic);



                        Some((new_raycast, update_data))
                    }
                }
            }).unzip();


            resolve = next_resolve;
            for (x, y, depth, normal, raw_color, bounces, location, metallic) in updates {



                //color_buffer.write(x, y, location/4.0);



                if bounces <= 0 {
                    metallic_buffer.write(x, y, (metallic * 256.0) as u8);
                } else if bounces >= 1 {
                    metallic_buffer.lerp(x, y, metallic as u8, (0.0 + (metallic *(1.0/bounces as f32))) as f32);
                }


            }

            bounce_limit -= 1;
        }

        metallic_buffer
    }
    pub fn lazy_fire(&mut self, seed: u32) {
        let mask = self.roughness_fire(seed);
        let mut bitmask = BitBuffer::new(self.resolutionx as usize, self.resolutiony as usize);
        for x in 0..self.resolutionx as usize {
            for y in 0..self.resolutiony as usize {
                bitmask.write(x, y, (mask.index(x, y) > 64))
            }
        }
        self.selective_fire(seed, bitmask);
    }
}







