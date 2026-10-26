use crate::vector3d::Vector3d;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use crate::buffer::ColorBuffer;

const IMPULSE_THRESHOLD: f32 = 0.02;
const BLUR_SIGMA: f32 = 1.3;



pub(crate) fn save_png<P: AsRef<Path>>(buffer: &ColorBuffer, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let width = buffer.x;
    let height = buffer.y;

    if width == 0 || height == 0 {
        return Err("Invalid image dimensions".into());
    }

    let mut img = vec![0f32; width * height * 3];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 3;

            let color = buffer.index(x, y);

            img[idx] = unit(color.x);
            img[idx + 1] = unit(color.y);
            img[idx + 2] = unit(color.z);
        }
    }

    let mut work = vec![0f32; img.len()];

    remove_salt_and_pepper(&img, &mut work, width, height);

    let kernel = gaussian_kernel(BLUR_SIGMA);
    blur_pass(&work, &mut img, width, height, &kernel, true);
    blur_pass(&img, &mut work, width, height, &kernel, false);

    let png_buffer: Vec<u8> = work.iter().map(|&v| (v.clamp(0.0, 1.0) * 255.0).round() as u8).collect();

    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width as u32, height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&png_buffer)?;
    writer.finish()?;

    Ok(())
}

fn unit(v: f32) -> f32 {
    if v.is_nan() { 0.0 } else { v.clamp(0.0, 1.0) }
}

fn remove_salt_and_pepper(src: &[f32], dst: &mut [f32], w: usize, h: usize) {
    let mut win = [0f32; 9];

    for y in 0..h {
        let (y0, y1) = (y.saturating_sub(1), (y + 1).min(h - 1));
        for x in 0..w {
            let (x0, x1) = (x.saturating_sub(1), (x + 1).min(w - 1));
            for c in 0..3 {
                let center = src[(y * w + x) * 3 + c];
                let (mut lo, mut hi, mut n) = (center, center, 0);

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
                if hi - lo > IMPULSE_THRESHOLD && (center <= lo || center >= hi) {
                    let win = &mut win[..n];
                    win.sort_unstable_by(f32::total_cmp);
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

fn blur_pass(src: &[f32], dst: &mut [f32], w: usize, h: usize, kernel: &[f32], horizontal: bool) {
    let r = kernel.len() / 2;

    for y in 0..h {
        for x in 0..w {
            for c in 0..3 {
                let mut acc = 0.0;
                for (i, k) in kernel.iter().enumerate() {
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