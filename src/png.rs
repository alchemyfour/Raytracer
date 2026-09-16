use crate::pixel::Pixel;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// A sample only counts as salt/pepper if it's the min or max of its 3×3
/// neighbourhood AND sits further than this from the neighbourhood median.
/// 0.0 = replace every local extreme. Raise it to be more conservative.
const IMPULSE_THRESHOLD: f32 = 0.02;

/// Std-dev (in pixels) of the Gaussian applied after denoising. 0.0 = off.
const BLUR_SIGMA: f32 = 1.3;

/// Saves a slice of `Pixel`s to a PNG file.
///
/// Dimensions come from the maximum x and y coordinates among the pixels.
/// Before encoding, the image gets a switching 3×3 median (salt-and-pepper
/// removal) followed by a light separable Gaussian blur.
pub(crate) fn save_png<P: AsRef<Path>>(pixels: &[Pixel<'_>], path: P) -> Result<(), Box<dyn std::error::Error>> {
    if pixels.is_empty() {
        return Err("No pixels to save".into());
    }

    let width = pixels.iter().map(|p| p.x as usize).max().map(|x| x + 1).unwrap_or(0);
    let height = pixels.iter().map(|p| p.y as usize).max().map(|y| y + 1).unwrap_or(0);

    if width == 0 || height == 0 {
        return Err("Invalid image dimensions".into());
    }

    // Float RGB in [0, 1]. Pixels that were never written stay black.
    let mut img = vec![0f32; width * height * 3];

    for pixel in pixels {
        if pixel.x >= 0.0 && pixel.y >= 0.0 {
            let px = pixel.x as usize;
            let py = pixel.y as usize;
            if px < width && py < height {
                let idx = (py * width + px) * 3;
                img[idx] = unit(pixel.color.x as f32);
                img[idx + 1] = unit(pixel.color.y as f32);
                img[idx + 2] = unit(pixel.color.z as f32);
            }
        }
    }

    // Ping-pong between two buffers: img -> work -> img -> work.
    let mut work = vec![0f32; img.len()];
    remove_salt_and_pepper(&img, &mut work, width, height);
    let kernel = gaussian_kernel(BLUR_SIGMA);
    blur_pass(&work, &mut img, width, height, &kernel, true);
    blur_pass(&img, &mut work, width, height, &kernel, false);

    let buffer: Vec<u8> = work.iter().map(|&v| (v * 255.0).round() as u8).collect();

    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    writer.write_image_data(&buffer)?;
    // Writes IEND and flushes. Relying on Drop silently swallows I/O errors.
    writer.finish()?;
    Ok(())
}

/// Clamp to [0, 1]. NaN becomes 0 (and then gets treated as pepper).
fn unit(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

/// Switching median filter, per channel.
///
/// Only samples that look like impulses are replaced with their 3×3 median, so
/// flat areas, gradients, anti-aliased edges and thick lines pass through
/// untouched. A plain median would soften all of them.
fn remove_salt_and_pepper(src: &[f32], dst: &mut [f32], w: usize, h: usize) {
    let mut win = [0f32; 9];

    for y in 0..h {
        let (y0, y1) = (y.saturating_sub(1), (y + 1).min(h - 1));
        for x in 0..w {
            let (x0, x1) = (x.saturating_sub(1), (x + 1).min(w - 1));
            for c in 0..3 {
                let center = src[(y * w + x) * 3 + c];
                let (mut lo, mut hi, mut n) = (center, center, 0);

                // In-bounds neighbours only: 9 inside, 6 on an edge, 4 in a corner.
                for sy in y0..=y1 {
                    for sx in x0..=x1 {
                        let v = src[(sy * w + sx) * 3 + c];
                        win[n] = v;
                        n += 1;
                        lo = lo.min(v);
                        hi = hi.max(v);
                    }
                }

                let mut out = center;
                // If the whole window spans <= threshold, nothing here can qualify.
                if hi - lo > IMPULSE_THRESHOLD && (center <= lo || center >= hi) {
                    let win = &mut win[..n];
                    win.sort_unstable_by(f32::total_cmp);
                    // Even-sized border windows: take the middle value on the side
                    // away from the impulse. Same index as usual when n == 9.
                    let median = if center >= hi { win[(n - 1) / 2] } else { win[n / 2] };
                    if (center - median).abs() > IMPULSE_THRESHOLD {
                        out = median;
                    }
                }
                dst[(y * w + x) * 3 + c] = out;
            }
        }
    }
}

/// Normalised 1-D Gaussian with radius ceil(3σ). σ <= 0 gives the identity.
fn gaussian_kernel(sigma: f32) -> Vec<f32> {
    if sigma <= 0.0 {
        return vec![1.0];
    }
    let r = (3.0 * sigma).ceil() as usize;
    let mut k: Vec<f32> = (0..=2 * r)
        .map(|i| {
            let d = i as f32 - r as f32;
            (-(d * d) / (2.0 * sigma * sigma)).exp()
        })
        .collect();
    let sum: f32 = k.iter().sum();
    k.iter_mut().for_each(|v| *v /= sum);
    k
}

/// One direction of the separable blur, edges clamped.
fn blur_pass(src: &[f32], dst: &mut [f32], w: usize, h: usize, kernel: &[f32], horizontal: bool) {
    let r = kernel.len() / 2;

    for y in 0..h {
        for x in 0..w {
            for c in 0..3 {
                let mut acc = 0.0;
                for (i, k) in kernel.iter().enumerate() {
                    // Tap offset is i - r, clamped into the image.
                    let (sx, sy) = if horizontal {
                        ((x + i).saturating_sub(r).min(w - 1), y)
                    } else {
                        (x, (y + i).saturating_sub(r).min(h - 1))
                    };
                    acc += k * src[(sy * w + sx) * 3 + c];
                }
                dst[(y * w + x) * 3 + c] = acc;
            }
        }
    }
}