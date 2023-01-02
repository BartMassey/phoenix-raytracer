//! Infrastructure constants and the like for librt.

/// For convenience.
pub use std::f64::consts::PI;

/// A "small number" for various calculations.
pub const TINY: f64 = 0.00001;

/// Degrees to radians.
macro_rules! dtor {
    ($deg:literal) => { PI * $deg / 180.0 };
}

/// Scene viewing angle. XXX This is hardcoded for now.
pub const A: f64 = dtor!(25.0);

/// Scale distance. XXX This is hardcoded for now.
pub const D: f64 = 10.0;
