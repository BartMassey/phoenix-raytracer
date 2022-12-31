//! Infrastructure constants and the like for librt.

/// For convenience.
pub use std::f64::consts::PI;

/// A "small number" for various calculations.
pub const TINY: f64 = 0.00001;

/// Scene viewing angle. XXX This is hardcoded for now.
pub const A: f64 = 25.0 * PI / 180.0;

/// Scene viewing distance. XXX This is hardcoded for now.
pub const D: f64 = 10.0;
