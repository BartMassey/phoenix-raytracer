// Copyright © 2017 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.
    
//! Terrible LCG PRNG because Rust and global state.
//! http://nuclear.llnl.gov/CNP/rng/rngman/node4.html

/// Produce a pseudo-random integer.
pub fn random() -> u64 {
    unsafe {
        static mut STATE: u64 = 0x123456789abcdef0u64;
        let tmp =
          STATE.wrapping_mul(2862933555777941757u64) + 3037000493u64;
        STATE = tmp;
        tmp
    }
}

/// Produce a pseudo-random floating point number in the
/// range [0..1] with 32 bits of precision.
pub fn frandom() -> f64 {
    (random() & 0xffffffffu64) as f64 / (0xffffffffu64 as f64)
}
