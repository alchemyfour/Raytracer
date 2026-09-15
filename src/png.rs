use crate::pixel::Pixel;
use crate::vector3d::Vector3d;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Denoises the image using a Gaussian blur filter of configurable radius.
/// `radius` defines the kernel size of (2 * radius + 1) x (2 * radius + 1).
pub(crate) fn denoise(pixels: &[Pixel<'_>], width: usize, height: usize, radius: usize) -> Vec<Vector3d> {
    if radius == 0 {
        let mut original_colors = vec![Vector3d::new(0.0, 0.0, 0.0); width * height];
        for pixel in pixels {
            if pixel.x >= 0.0 && pixel.y >= 0.0 {
                let px = pixel.x as usize;
                let py = pixel.y as usize;
                if px < width && py < height {
                    original_colors[py * width + px] = pixel.color;
                }
            }
        }
        return original_colors;
    }

    let mut original_colors = vec![Vector3d::new(0.0, 0.0, 0.0); width * height];
    for pixel in pixels {
        if pixel.x >= 0.0 && pixel.y >= 0.0 {
            let px = pixel.x as usize;
            let py = pixel.y as usize;
            if px < width && py < height {
                original_colors[py * width + px] = pixel.color;
            }
        }
    }

    let mut denoised = vec![Vector3d::new(0.0, 0.0, 0.0); width * height];
    let r = radius as isize;
    let sigma = radius as f32;
    let two_sigma_sq = 2.0 * sigma * sigma;

    for y in 0..height {
        for x in 0..width {
            let mut sum = Vector3d::new(0.0, 0.0, 0.0);
            let mut weight_sum = 0.0;
            for dy in -r..=r {
                for dx in -r..=r {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        let weight = (-((dx * dx + dy * dy) as f32) / two_sigma_sq).exp();
                        sum = sum + original_colors[ny as usize * width + nx as usize] * weight;
                        weight_sum += weight;
                    }
                }
            }
            if weight_sum > 0.0 {
                denoised[y * width + x] = sum / weight_sum;
            }
        }
    }
    denoised
}

/// Saves a slice of `Pixel`s to a PNG file after applying the Gaussian denoising algorithm with a default radius of 2.
pub(crate) fn save_png<P: AsRef<Path>>(pixels: &[Pixel<'_>], path: P) -> Result<(), Box<dyn std::error::Error>> {
    // Configurable default radius (change this to adjust the default blur size)
    save_png_with_radius(pixels, path, 2)
}

/// Saves a slice of `Pixel`s to a PNG file after applying the Gaussian denoising algorithm with a configurable radius.
pub(crate) fn save_png_with_radius<P: AsRef<Path>>(pixels: &[Pixel<'_>], path: P, radius: usize) -> Result<(), Box<dyn std::error::Error>> {
    if pixels.is_empty() {
        return Err("No pixels to save".into());
    }

    let width = pixels.iter().map(|p| p.x as usize).max().map(|x| x + 1).unwrap_or(0);
    let height = pixels.iter().map(|p| p.y as usize).max().map(|y| y + 1).unwrap_or(0);

    if width == 0 || height == 0 {
        return Err("Invalid image dimensions".into());
    }

    let mut buffer = vec![0u8; width * height * 3];
    let denoised_colors = denoise(pixels, width, height, radius);

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;
            let color = denoised_colors[y * width + x];
            buffer[idx] = (color.x.clamp(0.0, 1.0) * 255.0) as u8;
            buffer[idx + 1] = (color.y.clamp(0.0, 1.0) * 255.0) as u8;
            buffer[idx + 2] = (color.z.clamp(0.0, 1.0) * 255.0) as u8;
        }
    }

    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&buffer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Raycast;
    use crate::vector3d::{Ray3d, Vector3d};
    use crate::bvh::BVH;
    use crate::bintree::TreeBranch;
    use crate::bvh::BV;
    use crate::box3d::Box3d;

    #[test]
    fn test_denoise_with_radius_1() {
        // Create a 3x3 grid of pixels
        let mut pixels = Vec::new();
        let bvh = BVH {
            tree: TreeBranch::new(BV::new(Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0))),
            primitives: vec![],
        };
        
        for y in 0..3 {
            for x in 0..3 {
                let raycast = Raycast::new(
                    Ray3d::new(Vector3d::new(0.0, 0.0, 0.0), Vector3d::new(0.0, 0.0, 1.0)),
                    &bvh,
                );
                let mut pixel = Pixel::new(raycast, x as f32, y as f32);
                if x == 0 && y == 0 {
                    pixel.color = Vector3d::new(1.0, 1.0, 1.0);
                } else {
                    pixel.color = Vector3d::new(0.0, 0.0, 0.0);
                }
                pixels.push(pixel);
            }
        }

        let denoised = denoise(&pixels, 3, 3, 1);
        
        // At (0,0), itself + 3 neighbors are valid. Only (0,0) is 1.0. 
        // Sum of weight * val = 1.0. Sum of weights of valid pixels = 2.58094.
        // Expected value = 1.0 / 2.58094 = 0.38746.
        let val_00 = denoised[0 * 3 + 0];
        assert!((val_00.x - 0.38746).abs() < 1e-4);
        assert!((val_00.y - 0.38746).abs() < 1e-4);
        assert!((val_00.z - 0.38746).abs() < 1e-4);

        // At (1,1), all 9 pixels are valid. Only (0,0) (diagonal neighbor) is 1.0.
        // Weight of (0,0) from (1,1) is 0.36788. Sum of weights of all 9 pixels = 4.89764.
        // Expected value = 0.36788 / 4.89764 = 0.07511.
        let val_11 = denoised[1 * 3 + 1];
        assert!((val_11.x - 0.07511).abs() < 1e-4);
        assert!((val_11.y - 0.07511).abs() < 1e-4);
        assert!((val_11.z - 0.07511).abs() < 1e-4);
    }

    #[test]
    fn test_denoise_with_radius_0() {
        let mut pixels = Vec::new();
        let bvh = BVH {
            tree: TreeBranch::new(BV::new(Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0))),
            primitives: vec![],
        };
        let raycast = Raycast::new(
            Ray3d::new(Vector3d::new(0.0, 0.0, 0.0), Vector3d::new(0.0, 0.0, 1.0)),
            &bvh,
        );
        let mut pixel = Pixel::new(raycast, 0.0, 0.0);
        pixel.color = Vector3d::new(0.5, 0.5, 0.5);
        pixels.push(pixel);

        let denoised = denoise(&pixels, 1, 1, 0);
        assert_eq!(denoised[0], Vector3d::new(0.5, 0.5, 0.5));
    }
}
