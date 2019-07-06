//! Intrinsics for the [x86_64](https://en.wikipedia.org/wiki/X86-64) processor family.

use core::arch::x86_64::*;

/// As [_rdtsc](core::arch::x86_64::_rdtsc), just marked safe since it's always safe.
#[inline]
pub fn rdtsc() -> u64 {
  unsafe {
    _rdtsc()
  }
}

#[derive(Debug, Clone, Copy)]
pub struct m128(pub __m128);

#[derive(Debug, Clone, Copy)]
pub struct m128i(pub __m128i);

#[derive(Debug, Clone, Copy)]
pub struct m128d(pub __m128d);
