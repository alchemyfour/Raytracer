use crate::pixel::Pixel;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Saves a slice of `Pixel`s to a PNG file.
///
/// It determines the image dimensions from the maximum x and y coordinates
/// found among the pixels.
pub(crate) fn save_png<P: AsRef<Path>>(pixels: &[Pixel<'_>], path: P) -> Result<(), Box<dyn std::error::Error>> {
    if pixels.is_empty() {
        return Err("No pixels to save".into());
    }

    let width = pixels.iter().map(|p| p.x as usize).max().map(|x| x + 1).unwrap_or(0);
    let height = pixels.iter().map(|p| p.y as usize).max().map(|y| y + 1).unwrap_or(0);

    if width == 0 || height == 0 {
        return Err("Invalid image dimensions".into());
    }

    let mut buffer = vec![0u8; width * height * 3];

    for pixel in pixels {
        if pixel.x >= 0.0 && pixel.y >= 0.0 {
            let px = pixel.x as usize;
            let py = pixel.y as usize;
            if px < width && py < height {
                let idx = (py * width + px) * 3;
                buffer[idx] = (pixel.color.x.clamp(0.0, 1.0) * 255.0) as u8;
                buffer[idx + 1] = (pixel.color.y.clamp(0.0, 1.0) * 255.0) as u8;
                buffer[idx + 2] = (pixel.color.z.clamp(0.0, 1.0) * 255.0) as u8;
            }
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
    use crate::vector3d::{Vector3d, Ray3d};
    use crate::box3d::Box3d;
    use crate::bvh::{BVH, BV};
    use crate::bintree::TreeBranch;
    use crate::ray::Raycast;

    #[test]
    fn test_save_png() {
        let scene_box = Box3d::new(0.0, 1.0, 0.0, 1.0, 0.0, 1.0);
        let tree = TreeBranch::new(BV::new(scene_box));
        let bvh = BVH { tree, primitives: Vec::new() };
        let ray = Ray3d::new(Vector3d::new(0.0, 0.0, 0.0), Vector3d::new(0.0, 0.0, 1.0));
        let raycast = Raycast::new(ray, &bvh);

        let pixels = vec![
            Pixel {
                bounces: 0.0,
                color: Vector3d::new(1.0, 0.0, 0.0), // Red
                raycast,
                intersections: Vec::new(),
                x: 0.0,
                y: 0.0,
            },
            Pixel {
                bounces: 0.0,
                color: Vector3d::new(0.0, 1.0, 0.0), // Green
                raycast,
                intersections: Vec::new(),
                x: 1.0,
                y: 0.0,
            },
            Pixel {
                bounces: 0.0,
                color: Vector3d::new(0.0, 0.0, 1.0), // Blue
                raycast,
                intersections: Vec::new(),
                x: 0.0,
                y: 1.0,
            },
            Pixel {
                bounces: 0.0,
                color: Vector3d::new(1.0, 1.0, 1.0), // White
                raycast,
                intersections: Vec::new(),
                x: 1.0,
                y: 1.0,
            },
        ];

        let path = std::env::temp_dir().join("test_output.png");
        let result = save_png(&pixels, &path);
        assert!(result.is_ok());
        assert!(path.exists());
        let _ = std::fs::remove_file(path);
    }
}
