#![cfg(target_feature="sse")]

use super::*;
use core::ops::*;

/// A 128-bit SIMD value. Always used as `f32x4`.
#[derive(Clone, Copy)]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128(pub __m128);

unsafe impl Zeroable for m128 {}
unsafe impl Pod for m128 {}

impl core::fmt::Debug for m128 {
  /// Debug formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::Display for m128 {
  /// Display formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({}, {}, {}, {})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::LowerExp for m128 {
  /// LowerExp formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:e}, {:e}, {:e}, {:e})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::UpperExp for m128 {
  /// UpperExp formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:E}, {:E}, {:E}, {:E})", a[0], a[1], a[2], a[3])
  }
}

impl Add for m128 {
  type Output = Self;
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_ps(self.0, rhs.0) })
  }
}
impl AddAssign for m128 {
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_add_ps(self.0, rhs.0) };
  }
}

impl m128 {
  /// Adds the 0th lanes without affecting the other lanes of `self.
  #[inline(always)]
  pub fn add0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_ss(self.0, rhs.0) })
  }
}
