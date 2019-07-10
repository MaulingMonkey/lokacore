//! Intrinsics for the [x86](https://en.wikipedia.org/wiki/X86) processor family.

use super::*;

use core::arch::x86::*;

#[cfg(target_feature = "sse")]
#[path = "sse.rs"]
mod sse;
#[cfg(target_feature = "sse")]
pub use sse::*;

#[cfg(target_feature = "sse2")]
#[path = "sse2.rs"]
mod sse2;
#[cfg(target_feature = "sse2")]
pub use sse2::*;

/// As [`_rdtsc`](core::arch::x86::_rdtsc).
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { _rdtsc() }
}
