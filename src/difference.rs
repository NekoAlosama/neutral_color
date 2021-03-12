/*
Goals:
 - Figure out and use a good color difference formula
  - HyAB color difference formula is currently used,
    but it originally used CIELAB inputs
*/

extern crate oklab;
use oklab::*;

// HyAB color difference
// |L_1 - L_2| + dist((a_1, b_1), (a_2, b_2))
pub fn hyab(lab: &Oklab, lab2: &Oklab) -> f32 {
    (lab.l - lab2.l).abs() + ((lab.a - lab2.a).powi(2) + (lab.b - lab2.b).powi(2)).sqrt()
}
