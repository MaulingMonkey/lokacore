//! Intrinsics for the [x86](https://en.wikipedia.org/wiki/X86) processor family.

use super::*;

use core::arch::x86::*;

unsafe impl Zeroable for __m128i {}
unsafe impl Zeroable for __m128 {}
unsafe impl Zeroable for __m128d {}
unsafe impl Zeroable for __m256i {}
unsafe impl Zeroable for __m256 {}
unsafe impl Zeroable for __m256d {}

unsafe impl Pod for __m128i {}
unsafe impl Pod for __m128 {}
unsafe impl Pod for __m128d {}
unsafe impl Pod for __m256i {}
unsafe impl Pod for __m256 {}
unsafe impl Pod for __m256d {}

// Note(Lokathor): THE SAFETY OF THE SSE AND SSE2 MODULES DEPENDS UPON COMPILE
// TIME FEATURE SETTINGS. THEY DO NOT PERFORM ANY RUNTIME FEATURE DETECTION. YOU
// SHOULD NOT REMOVE THE TARGET FEATURE ATTRIBUTES FOR ANY REASON.

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

#[cfg(target_feature = "sse3")]
#[path = "sse3.rs"]
mod sse3;
#[cfg(target_feature = "sse3")]
pub use sse3::*;

/// As [`_rdtsc`](https://doc.rust-lang.org/core/arch/x86/fn._rdtsc.html).
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { _rdtsc() }
}
