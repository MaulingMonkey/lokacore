//! Intrinsics for the [x86_64](https://en.wikipedia.org/wiki/X86-64) processor family.

use super::*;

use core::arch::x86_64::*;

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

/// As [_rdtsc](core::arch::x86_64::_rdtsc), just marked safe since it's always safe.
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { _rdtsc() }
}

/// A 128-bit SIMD register, always used as `f32x4`
#[derive(Clone, Copy)]
#[cfg_attr(not(target_feature = "sse"), derive(Debug))]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128(pub __m128);

#[cfg(target_feature = "sse")]
impl core::fmt::Debug for m128 {
  /// Formats in set/store order: high index lane to low index lane.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "m128({}, {}, {}, {})", a[3], a[2], a[1], a[0])
  }
}

#[test]
#[cfg(target_feature = "sse")]
fn test_m128_debug() {
  extern crate std;
  let m = m128::set(5.0, 6.0, 7.0, 8.5);
  assert_eq!(&std::format!("{:?}", m), "m128(5, 6, 7, 8.5)");
}

/// A 128-bit SIMD register, the integer lanes depend on the operation used.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_feature = "sse2"), derive(Debug))]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128i(pub __m128i);

#[cfg(target_feature = "sse2")]
impl core::fmt::Debug for m128i {
  /// Formats in set/store order: high index lane to low index lane.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let u = self.to_u128();
    write!(f, "m128i({})", u)
  }
}

#[test]
#[cfg(target_feature = "sse")]
fn test_m128i_debug() {
  extern crate std;
  let m = m128i::set_i32(-1, 0, 1, 15);
  let expected = (-1i32 as u32 as u128) << 96 | 1 << 32 | 15;
  assert_eq!(
    &std::format!("{:?}", m),
    &std::format!("m128i({})", expected)
  );
}

/// A 128-bit SIMD register, always used as `f64x2`
#[derive(Clone, Copy)]
#[cfg_attr(not(target_feature = "sse2"), derive(Debug))]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128d(pub __m128d);

#[cfg(target_feature = "sse2")]
impl core::fmt::Debug for m128d {
  /// Formats in set/store order: high index lane to low index lane.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "m128d({}, {})", a[1], a[0])
  }
}

#[test]
#[cfg(target_feature = "sse")]
fn test_m128d_debug() {
  extern crate std;
  let m = m128d::set(5.0, 6.5);
  assert_eq!(&std::format!("{:?}", m), "m128d(5, 6.5)");
}
