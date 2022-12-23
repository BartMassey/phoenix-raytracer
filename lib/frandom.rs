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
        if matches!(STATE.compare_exchange(current, new, SeqCst, SeqCst), Ok(current)) {
            return new;
        }
    }
}

/// Produce a pseudo-random floating point number in the
/// range [0..1].
pub fn frandom() -> f64 {
    random() as f64 / (!0u64 as f64)
}
