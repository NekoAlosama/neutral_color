/*
Goals:
 - Figure out the ranges for Oklab values
   - CIELAB ranges: from 0 to 100, -128/-127 to 127/128, -128/-127 to 127/128
   - No real idea what Oklab ranges are
 - Figure out and use a good color difference formula
   - The fact that the colors generated are somewhat similar (except for lightness) gives some credit to Oklab?
   - Additional notes added below
 - Experiment with Oklch
   - Needs a different crate like tincture (unless the oklab crate gets updated)
Excluded formulas:
 - Euclidian distance for all axes
   - Does not work well for Oklab.L distances
*/

extern crate oklab;
use oklab::*;

#[allow(dead_code)]
pub fn hyab(lab: &Oklab, lab2: &Oklab) -> f32 {
    /*
    HyAB (Hybrid of absolute value and Euclidian distance): |L_1 - L_2| + dist((a_1, b_1), (a_2, b_2))
    An alternative to CIEDE2000 since it doesn't work with differences >10 and this is simpler
    Originally used with CIELAB inputs as it was created before Oklab (http://markfairchild.org/PDFs/PAP40.pdf)
    Color generated: (109, 110, 66) */
    (lab.l - lab2.l).abs() + ((lab.a - lab2.a).powi(2) + (lab.b - lab2.b).powi(2)).sqrt()
}

#[allow(dead_code)]
pub fn absolute(lab: &Oklab, lab2: &Oklab) -> f32 {
    /*
    Absolute value: |L_1 - L_2| + |a_1 - a_2| + |b_1 - b_2|
    Also known as city-block or Manhattan distance, due to their use for calculating walking distances in cities
    Color generated: (102, 131, 112) */
    (lab.l - lab2.l).abs() + (lab.a - lab2.a).abs() + (lab.b - lab2.b).abs()
}
