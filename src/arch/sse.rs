#![cfg(target_feature="sse")]

use super::*;
use core::ops::*;

/// A 128-bit SIMD value. Always used as `f32x4`.
///
/// * This documentation numbers the lanes sensibly: based on the index you'd
///   need to use to access that lane if the value were cast to an array.
/// * This is also the way that the type is printed out using
///   [`Debug`](core::fmt::Debug), [`Display`](core::fmt::Display),
///   [`LowerExp`](core::fmt::LowerExp), and [`UpperExp`](core::fmt::UpperExp).
/// * **Please note** that Intel hates you, so the normal [`set`](m128::set),
///   [`load`](m128::load), and [`store`](m128::store) order is _opposite_ of
///   sensible, index based ordering. The "unaligned" version also uses the
///   normal order. There are "reverse" order operations, but not "reverse
///   unaligned" versions.
/// * Most operations work per-lane, "lanewise".
/// * Some operations work using lane 0 only. When appropriate, these have the
///   same name as the lanewise version but with a `0` on the end. Eg: `cmp_eq`
///   and `cmp_eq0`. The other lanes are simply copied forward from `self`.
/// * Comparisons give "bool-ish" output, where all bits 1 in a lane is true,
///   and all bits 0 in a lane is false. Unfortunately, all bits 1 with a float
///   is one of the `NaN` values, so that part can make it harder to work with.
#[derive(Clone, Copy)]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128(pub __m128);

unsafe impl Zeroable for m128 {}
unsafe impl Pod for m128 {}

impl core::fmt::Debug for m128 {
  /// Debug formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32; 4] = cast(self.0);
    write!(f, "m128({:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::Display for m128 {
  /// Display formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32; 4] = cast(self.0);
    write!(f, "m128({}, {}, {}, {})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::LowerExp for m128 {
  /// LowerExp formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32; 4] = cast(self.0);
    write!(f, "m128({:e}, {:e}, {:e}, {:e})", a[0], a[1], a[2], a[3])
  }
}

impl core::fmt::UpperExp for m128 {
  /// UpperExp formats in offset order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f32; 4] = cast(self.0);
    write!(f, "m128({:E}, {:E}, {:E}, {:E})", a[0], a[1], a[2], a[3])
  }
}

impl Add for m128 {
  type Output = Self;
  /// Lanewise addition.
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_ps(self.0, rhs.0) })
  }
}
impl AddAssign for m128 {
  /// Lanewise addition.
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_add_ps(self.0, rhs.0) };
  }
}

impl BitAnd for m128 {
  type Output = Self;
  /// Bitwise AND.
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    Self(unsafe { _mm_and_ps(self.0, rhs.0) })
  }
}
impl BitAndAssign for m128 {
  /// Bitwise AND.
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_ps(self.0, rhs.0) };
  }
}

impl Div for m128 {
  type Output = Self;
  /// Lanewise division.
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    Self(unsafe { _mm_div_ps(self.0, rhs.0) })
  }
}
impl DivAssign for m128 {
  /// Lanewise division.
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_div_ps(self.0, rhs.0) };
  }
}

impl m128 {
  /// Adds the 0th lanes without affecting the other lanes of `self.
  #[inline(always)]
  pub fn add0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_ss(self.0, rhs.0) })
  }

  /// Bitwise `(!self) & rhs`
  #[inline(always)]
  pub fn andnot(self, rhs: Self) -> Self {
    Self(unsafe { _mm_andnot_ps(self.0, rhs.0) })
  }

  /// Lanewise `self == rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpeq_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self == rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_eq0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpeq_ss(self.0, rhs.0) })
  }

  /// Lanewise `self >= rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_ge(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpge_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self >= rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_ge0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpge_ss(self.0, rhs.0) })
  }

  /// Lanewise `self > rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_gt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpgt_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self > rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_gt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpgt_ss(self.0, rhs.0) })
  }

  /// Lanewise `self <= rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_le(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmple_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self <= rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_le0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmple_ss(self.0, rhs.0) })
  }

  /// Lanewise `self < rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_lt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmplt_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self < rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_lt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmplt_ss(self.0, rhs.0) })
  }

  /// Lanewise `self != rhs` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_ne(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpneq_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self != rhs`, bool-ish output.
  #[inline(always)]
  pub fn cmp_ne0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpneq_ss(self.0, rhs.0) })
  }

  /// Lanewise `!(self >= rhs)` check, bool-ish output.
  ///
  /// Also, this triggers 3rd Impact.
  #[inline(always)]
  pub fn cmp_nge(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnge_ps(self.0, rhs.0) })
  }

  /// Lane 0: `!(self >= rhs)`, bool-ish output.
  #[inline(always)]
  pub fn cmp_nge0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnge_ss(self.0, rhs.0) })
  }

  /// Lanewise `!(self > rhs)` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_ngt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpngt_ps(self.0, rhs.0) })
  }

  /// Lane 0: `!(self > rhs)`, bool-ish output.
  #[inline(always)]
  pub fn cmp_ngt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpngt_ss(self.0, rhs.0) })
  }

  /// Lanewise `!(self <= rhs)` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_nle(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnle_ps(self.0, rhs.0) })
  }

  /// Lane 0: `!(self <= rhs)`, bool-ish output.
  #[inline(always)]
  pub fn cmp_nle0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnle_ss(self.0, rhs.0) })
  }

  /// Lanewise `!(self < rhs)` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_nlt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_ps(self.0, rhs.0) })
  }

  /// Lane 0: `!(self < rhs)`, bool-ish output.
  #[inline(always)]
  pub fn cmp_nlt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_ss(self.0, rhs.0) })
  }

  /// Lanewise `self.not_nan() & rhs.not_nan()` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_ordinary(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpord_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self.not_nan() & rhs.not_nan()`, bool-ish output.
  #[inline(always)]
  pub fn cmp_ordinary0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpord_ss(self.0, rhs.0) })
  }

  /// Lanewise `self.is_nan() | rhs.is_nan()` check, bool-ish output.
  #[inline(always)]
  pub fn cmp_nan(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpunord_ps(self.0, rhs.0) })
  }

  /// Lane 0: `self.is_nan() | rhs.is_nan()`, bool-ish output.
  #[inline(always)]
  pub fn cmp_nan0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpunord_ss(self.0, rhs.0) })
  }

  /// Lane 0: `self == rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_eq0(self, rhs: Self) -> i32 {
    unsafe { _mm_comieq_ss(self.0, rhs.0) }
  }

  /// Lane 0: `self >= rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_ge0(self, rhs: Self) -> i32 {
    unsafe { _mm_comige_ss(self.0, rhs.0) }
  }

  /// Lane 0: `self > rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_gt0(self, rhs: Self) -> i32 {
    unsafe { _mm_comigt_ss(self.0, rhs.0) }
  }

  /// Lane 0: `self <= rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_le0(self, rhs: Self) -> i32 {
    unsafe { _mm_comile_ss(self.0, rhs.0) }
  }

  /// Lane 0: `self < rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_lt0(self, rhs: Self) -> i32 {
    unsafe { _mm_comilt_ss(self.0, rhs.0) }
  }

  /// Lane 0: `self != rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_ne0(self, rhs: Self) -> i32 {
    unsafe { _mm_comineq_ss(self.0, rhs.0) }
  }

  /// Round the `i32` to `f32` and replace lane 0.
  ///
  /// Subject to the current thread's [rounding
  /// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#rounding-mode)
  #[inline(always)]
  pub fn round_replace0_i32(self, rhs: i32) -> Self {
    Self(unsafe { _mm_cvt_si2ss(self.0, rhs) })
  }

  /// Round lane 0 to `i32` and return.
  ///
  /// Subject to the current thread's [rounding
  /// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#rounding-mode)
  #[inline(always)]
  pub fn round_extract0_i32(self) -> i32 {
    unsafe { _mm_cvt_ss2si(self.0) }
  }

  /// Round the `i64` to `f32` and replace lane 0.
  ///
  /// Subject to the current thread's [rounding
  /// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#rounding-mode)
  /// 
  /// Not available to `x86`
  #[inline(always)]
  #[cfg(target_arch="x86_64")]
  pub fn round_replace0_i64(self, rhs: i64) -> Self {
    Self(unsafe { _mm_cvtsi64_ss(self.0, rhs) })
  }

  /// Directly extracts lane 0 as `f32`.
  #[inline(always)]
  pub fn extract0_f32(self) -> f32 {
    unsafe { _mm_cvtss_f32(self.0) }
  }

  /// Round lane 0 to `i64` and return.
  ///
  /// Subject to the current thread's [rounding
  /// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#rounding-mode)
  #[inline(always)]
  pub fn round_extract0_i64(self) -> i64 {
    unsafe { _mm_cvtss_si64(self.0) }
  }

  /// Truncate lane 0 to `i32` and return.
  #[inline(always)]
  pub fn truncate_extract0_i32(self) -> i32 {
    unsafe { _mm_cvtt_ss2si(self.0) }
  }

  /// Truncate lane 0 to `i64` and return.
  #[inline(always)]
  pub fn truncate_extract0_i64(self) -> i64 {
    unsafe { _mm_cvttss_si64(self.0) }
  }

  /// Divides the 0th lanes without affecting the other lanes of `self.
  #[inline(always)]
  pub fn div0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_div_ss(self.0, rhs.0) })
  }
}

/// A bit set of [exception
/// flags](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#exception-flags).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionMask(u32);
impl ExceptionMask {
  /// The raw `u32` mask value.
  pub const fn to_raw(self) -> u32 {
    self.0
  }
  /// Turns a raw `u32` value into an exception mask without any checks.
  /// 
  /// ## Safety
  /// 
  /// You must not pass a `u32` with invalid bits set.
  pub const unsafe fn from_raw_unchecked(val: u32) -> Self {
    Self(val)
  }
  /// Check the `_MM_EXCEPT_INVALID` bit.
  pub fn invalid(self) -> bool {
    (self.0 & _MM_EXCEPT_INVALID) > 0
  }
  /// Set the `_MM_EXCEPT_INVALID` bit.
  pub fn set_invalid(&mut self, invalid: bool) {
    self.0 ^= (u32::from(invalid).wrapping_neg() ^ self.0) & _MM_EXCEPT_INVALID;
  }
  /// Check the `_MM_EXCEPT_DENORM` bit.
  pub fn denorm(self) -> bool {
    (self.0 & _MM_EXCEPT_DENORM) > 0
  }
  /// Set the `_MM_EXCEPT_DENORM` bit.
  pub fn set_denorm(&mut self, denorm: bool) {
    self.0 ^= (u32::from(denorm).wrapping_neg() ^ self.0) & _MM_EXCEPT_DENORM;
  }
  /// Check the `_MM_EXCEPT_DIV_ZERO` bit.
  pub fn div_zero(self) -> bool {
    (self.0 & _MM_EXCEPT_DIV_ZERO) > 0
  }
  /// Set the `_MM_EXCEPT_DIV_ZERO` bit.
  pub fn set_div_zero(&mut self, div_zero: bool) {
    self.0 ^= (u32::from(div_zero).wrapping_neg() ^ self.0) & _MM_EXCEPT_DIV_ZERO;
  }
  /// Check the `_MM_EXCEPT_OVERFLOW` bit.
  pub fn overflow(self) -> bool {
    (self.0 & _MM_EXCEPT_OVERFLOW) > 0
  }
  /// Set the `_MM_EXCEPT_OVERFLOW` bit.
  pub fn set_overflow(&mut self, overflow: bool) {
    self.0 ^= (u32::from(overflow).wrapping_neg() ^ self.0) & _MM_EXCEPT_OVERFLOW;
  }
  /// Check the `_MM_EXCEPT_UNDERFLOW` bit.
  pub fn underflow(self) -> bool {
    (self.0 & _MM_EXCEPT_UNDERFLOW) > 0
  }
  /// Set the `_MM_EXCEPT_UNDERFLOW` bit.
  pub fn set_underflow(&mut self, underflow: bool) {
    self.0 ^= (u32::from(underflow).wrapping_neg() ^ self.0) & _MM_EXCEPT_UNDERFLOW;
  }
  /// Check the `_MM_EXCEPT_INEXACT` bit.
  pub fn inexact(self) -> bool {
    (self.0 & _MM_EXCEPT_INEXACT) > 0
  }
  /// Set the `_MM_EXCEPT_INEXACT` bit.
  pub fn set_inexact(&mut self, inexact: bool) {
    self.0 ^= (u32::from(inexact).wrapping_neg() ^ self.0) & _MM_EXCEPT_INEXACT;
  }
}

/// Calls to [`_MM_GET_EXCEPTION_MASK`](https://doc.rust-lang.org/core/arch/x86_64/fn._MM_GET_EXCEPTION_MASK.html), with newtype'd output.
pub fn exception_mask() -> ExceptionMask {
  ExceptionMask(unsafe { _MM_GET_EXCEPTION_MASK() })
}
