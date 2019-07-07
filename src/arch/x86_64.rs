//! Intrinsics for the [x86_64](https://en.wikipedia.org/wiki/X86-64) processor family.

use core::arch::x86_64::*;

#[cfg(target_feature = "sse")]
mod sse;
#[cfg(target_feature = "sse")]
pub use sse::*;

#[cfg(target_feature = "sse2")]
mod sse2;
#[cfg(target_feature = "sse2")]
pub use sse2::*;

/// As [_rdtsc](core::arch::x86_64::_rdtsc), just marked safe since it's always safe.
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { _rdtsc() }
}

/// A 128-bit SIMD register, `f32x4`
#[derive(Debug, Clone, Copy)]
#[allow(bad_style)]
pub struct m128(pub __m128);

/// A 128-bit SIMD register, integral data, layout based on the op.
#[derive(Debug, Clone, Copy)]
#[allow(bad_style)]
pub struct m128i(pub __m128i);

/// A 128-bit SIMD register, `f64x2`
#[derive(Debug, Clone, Copy)]
#[allow(bad_style)]
pub struct m128d(pub __m128d);
