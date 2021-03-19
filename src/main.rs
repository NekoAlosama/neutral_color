/*
Goals:
 - Use a uniform color space better than CIELAB like Oklab
   - Done, but remember to update with an even better one
 - Additional goals in difference.rs
*/

extern crate oklab;
use oklab::*;

mod difference;

use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();

    let mut saved_max: f32 = 1000.0;
    let mut saved_rgb: RGB<u8> = RGB::new(0, 0, 0);

    let quantize = 255;

    // Iterate through all sRGB colors
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let lab: Oklab = srgb_to_oklab(RGB::new(r, g, b));

                /* Lightness restriction for speedup and to get rid of really bright or dark ones
                if lab.l < 0.4 || lab.l > 0.6 {
                    continue;
                };*/

                let mut local_max: f32 = 0.0;

                // Instead of iterating through the whole sRGB color space per color,
                // we just iterate though a few select ones
                // It's probably fine to just use 0 or 255, but decrease quantize for completeness
                for r2 in (0..=255).step_by(quantize) {
                    for g2 in (0..=255).step_by(quantize) {
                        for b2 in (0..=255).step_by(quantize) {
                            let lab2: Oklab = srgb_to_oklab(RGB::new(r2, g2, b2));

                            // Get the difference and save if it's the max
                            let delta = difference::hyab(&lab, &lab2);
                            if delta > local_max {
                                local_max = delta;
                            }
                        }
                    }
                }

                // If this color's max is lower than the overall max, then save it
                if local_max < saved_max {
                    // Used if the differences are equal
                    if (local_max - saved_max).abs() < f32::EPSILON {
                        println!("\nEquivalent delta detected! saved_max: {}", saved_max);
                        println!("Previous RGB: {:?}", saved_rgb);
                        println!("Current RGB: {:?}", RGB::new(r, g, b));
                    }

                    saved_max = local_max;
                    saved_rgb = RGB::new(r, g, b);
                }
            }
        }
    }

    // Print results
    println!(
        "\nRGB Color: ({}, {}, {})",
        saved_rgb.r, saved_rgb.g, saved_rgb.b
    );
    println!(
        "Oklab Color: ({}, {}, {})",
        srgb_to_oklab(saved_rgb).l,
        srgb_to_oklab(saved_rgb).a,
        srgb_to_oklab(saved_rgb).b
    );
    println!("Color difference: {:?}", saved_max);

    // Print runtime
    let end = SystemTime::now();
    let runtime = end.duration_since(start).expect("bad time");
    println!("Runtime: {:?}", runtime);
}
