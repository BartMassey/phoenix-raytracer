// Copyright © 2017 Bart Massey
// [This program is licensed under the "3-clause ('new') BSD License"]
// Please see the file COPYING in the source
// distribution of this software for license terms.
    
//! Terrible LCG PRNG because Rust and global state.
//! http://nuclear.llnl.gov/CNP/rng/rngman/node4.html

use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

/// Produce a pseudo-random integer. Will likely be slow in
/// the presence of contention.
pub fn random() -> u64 {
    static STATE: AtomicU64 = AtomicU64::new(0x123456789abcdef0u64);
    loop {
        let current = STATE
            .load(SeqCst);
        let new = current
            .wrapping_mul(2862933555777941757u64)
            .wrapping_add(3037000493u64);
        loop {
            if STATE.compare_exchange(current, new, SeqCst, SeqCst).is_ok() {
                return new;
            }
        }

    }
}

const RES: u64 = 100_000;

/// Produce a pseudo-random floating point number in the
/// range [0..1], with the given resolution.
pub fn frandom() -> f64 {
    let n = random() >> 3;
    (n % (RES + 1)) as f64 / RES as f64
}
