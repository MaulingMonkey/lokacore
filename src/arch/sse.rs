#![cfg(target_feature="sse")]

use super::*;

/// A 128-bit SIMD value. Always used as `f32x4`.
///
/// * The convention for SIMD data is that, similar to a `u128` or `i128`, the
///   0th bit is on the far right, and bits count up as you move left.
/// * This type always treats the bits as if they were four `f32` values in a
///   row. Each `f32` is a "lane". Just like with bit numbering, the 0th (low)
///   lane is on the right and lane index values go up as you move left. This is
///   the opposite of how you're usually told to think about arrayed data, but
///   that's just the convention.
/// * There's both unary and binary "lanewise" operations, which cause each lane
///   to do the operation on its own.
/// * There's also operations with a `_low` suffix, which use only the 0th lane.
///   The other lanes are either copied forward from `self` (methods) or set to
///   `0.0` (constructor functions).
/// * There's "rounding" operations, which work according to the current
///   thread's rounding mode. See [`set_rounding_mode`].
#[derive(Clone, Copy)]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128(pub __m128);

unsafe impl Zeroable for m128 {}
unsafe impl Pod for m128 {}

impl core::fmt::Debug for m128 {
  /// Formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::Display for m128 {
  /// Formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({}, {}, {}, {})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::LowerExp for m128 {
  /// Formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:e}, {:e}, {:e}, {:e})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::UpperExp for m128 {
  /// Formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32;4] = cast(self.0);
    write!(f, "m128({:E}, {:E}, {:E}, {:E})", a[0], a[1], a[2], a[3])
  }
}
