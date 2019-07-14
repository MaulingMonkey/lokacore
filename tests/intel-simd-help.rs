//!
//! Module for test helpers common to all Intel SIMD testing.
//! 

#[cfg(target_arch="x86")]
pub use core::arch::x86::*;
#[cfg(target_arch="x86_64")]
pub use core::arch::x86_64::*;

#[cfg(target_arch="x86")]
pub use lokacore::arch::x86::*;
#[cfg(target_arch="x86_64")]
pub use lokacore::arch::x86_64::*;

pub use core::mem::*;
pub use lokacore::*;
