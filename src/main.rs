/*
Goals:
- Use a uniform color space better than CIELAB like Oklab
- Figure out and use a good color difference formula
  - HyAB color difference formula is currently used,
    but it originally used CIELAB inputs
*/

extern crate oklab;
use oklab::*;

use std::time::SystemTime;

fn diff(lab: &Oklab, lab2: &Oklab) -> f32 {
    (lab.l - lab2.l).abs() + ((lab.a - lab2.a).powi(2) + (lab.b - lab2.b).powi(2)).sqrt()
}

fn main() {
    let start = SystemTime::now();

    let mut saved_max: f32 = 1000.0;
    let mut saved_rgb: RGB<u8> = RGB::new(0, 0, 0);

    let error_margin = std::f32::EPSILON;

    // '1' tests 0, 255; '2' tests 0, 128, 255; '3' tests 0, 85, 170, 255;
    // '4' tests 0, 64, 128, 192, 255; etc.
    let quantize: f32 = 1_f32;

    // Iterate through all sRGB colors
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let lab = srgb_to_oklab(RGB::new(r, g, b));

                // Lightness restriction for speedup
                if lab.l < 0.4 || lab.l > 0.6 {
                    continue;
                };

                let mut local_max: f32 = 0.0;

                // Instead of iterating through the whole sRGB color space per color,
                // we just iterate though a few select ones
                for r2 in 0..=quantize as usize {
                    for g2 in 0..=quantize as usize {
                        for b2 in 0..=quantize as usize {
                            let lab2 = srgb_to_oklab(RGB::new(
                                (r2 as f32 * 255.0 / quantize).ceil() as u8,
                                (g2 as f32 * 255.0 / quantize).ceil() as u8,
                                (b2 as f32 * 255.0 / quantize).ceil() as u8,
                            ));

                            // Get the difference and save if it's the max
                            let delta = diff(&lab, &lab2);
                            if delta > local_max {
                                local_max = delta;
                            }
                        }
                    }
                }

                // If this color's max is lower than the overall max, then save it
                if (local_max - saved_max).abs() < error_margin {
                    // Used if the differences are equal
                    println!("Equivalent delta detected!");
                    saved_max = local_max;
                    saved_rgb = RGB::new(r, g, b);
                }
                if local_max < saved_max {
                    saved_max = local_max;
                    saved_rgb = RGB::new(r, g, b);
                }
            }
        }
    }

    // Print results
    println!("RGB Color: {:?}", saved_rgb);
    println!("Oklab Color: {:?}", srgb_to_oklab(saved_rgb));
    println!("Color difference of: {:?}", saved_max);

    // Print runtime
    let end = SystemTime::now();
    let runtime = end.duration_since(start).expect("bad time");
    println!("{:?}", runtime);
}
