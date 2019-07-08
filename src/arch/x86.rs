//! Intrinsics for the [x86](https://en.wikipedia.org/wiki/X86) processor family.

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

/// As [_rdtsc](core::arch::x86::_rdtsc), just marked safe since it's always safe.
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { core::arch::x86::_rdtsc() }
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
