use super::*;
use core::ops::*;

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
  /// Formats in set/store order.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a = self.to_array();
    write!(f, "m128({}, {}, {}, {})", a[3], a[2], a[1], a[0])
  }
}

impl Add for m128 {
  type Output = Self;
  /// f32x4 lanewise addition
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_ps(self.0, rhs.0) })
  }
}

impl Div for m128 {
  type Output = Self;
  /// f32x4 lanewise division
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    Self(unsafe { _mm_div_ps(self.0, rhs.0) })
  }
}

impl Mul for m128 {
  type Output = Self;
  /// f32x4 lanewise multiplication
  #[inline(always)]
  fn mul(self, rhs: Self) -> Self {
    Self(unsafe { _mm_mul_ps(self.0, rhs.0) })
  }
}

impl Sub for m128 {
  type Output = Self;
  /// f32x4 lanewise subtraction
  #[inline(always)]
  fn sub(self, rhs: Self) -> Self {
    Self(unsafe { _mm_sub_ps(self.0, rhs.0) })
  }
}

impl Neg for m128 {
  type Output = Self;
  /// lanewise unary negation (`0.0 - self`)
  #[inline(always)]
  fn neg(self) -> Self {
    Self(unsafe { _mm_sub_ps(_mm_setzero_ps(), self.0) })
  }
}

impl AddAssign for m128 {
  /// f32x4 lanewise addition then assignment
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_add_ps(self.0, rhs.0) };
  }
}

impl DivAssign for m128 {
  /// f32x4 lanewise division then assignment
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_div_ps(self.0, rhs.0) };
  }
}

impl MulAssign for m128 {
  /// f32x4 lanewise multiplication then assignment
  #[inline(always)]
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_mul_ps(self.0, rhs.0) };
  }
}

impl SubAssign for m128 {
  /// f32x4 lanewise subtraction then assignment
  #[inline(always)]
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_sub_ps(self.0, rhs.0) };
  }
}

impl BitAnd for m128 {
  type Output = Self;
  /// bitwise `&`
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    Self(unsafe { _mm_and_ps(self.0, rhs.0) })
  }
}

impl BitOr for m128 {
  type Output = Self;
  /// bitwise `|`
  #[inline(always)]
  fn bitor(self, rhs: Self) -> Self {
    Self(unsafe { _mm_or_ps(self.0, rhs.0) })
  }
}

impl BitXor for m128 {
  type Output = Self;
  /// bitwise `XOR`
  #[inline(always)]
  fn bitxor(self, rhs: Self) -> Self {
    Self(unsafe { _mm_xor_ps(self.0, rhs.0) })
  }
}

impl BitAndAssign for m128 {
  /// bitwise `&` then assignment
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_ps(self.0, rhs.0) };
  }
}

impl BitOrAssign for m128 {
  /// bitwise `|` then assignment
  #[inline(always)]
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_or_ps(self.0, rhs.0) };
  }
}

impl BitXorAssign for m128 {
  /// bitwise `XOR` then assignment
  #[inline(always)]
  fn bitxor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_xor_ps(self.0, rhs.0) };
  }
}

/// # SSE Operations
impl m128 {
  /// Sets the `f32` values into lanes from high to low.
  #[inline(always)]
  pub fn set(e3: f32, e2: f32, e1: f32, e0: f32) -> Self {
    Self(unsafe { _mm_set_ps(e3, e2, e1, e0) })
  }

  /// Sets the `f32` values into lanes with reverse order, from to low high.
  #[inline(always)]
  pub fn set_reverse(e0: f32, e1: f32, e2: f32, e3: f32) -> Self {
    Self(unsafe { _mm_setr_ps(e0, e1, e2, e3) })
  }

  /// Sets the `f32` as the value for all lanes.
  #[inline(always)]
  pub fn splat(f: f32) -> Self {
    Self(unsafe { _mm_set1_ps(f) })
  }

  /// Sets the `f32` as the low lane, other lanes zero.
  #[inline(always)]
  pub fn set_low(f: f32) -> Self {
    Self(unsafe { _mm_set_ss(f) })
  }

  /// Returns a value with all lanes zero.
  #[inline(always)]
  pub fn zeroed() -> Self {
    Self(unsafe { _mm_setzero_ps() })
  }

  /// Loads the `f32`s in so that the index matches the lane.
  #[inline(always)]
  pub fn load(addr: &Align16<[f32; 4]>) -> Self {
    let p = addr as *const Align16<[f32; 4]> as *const f32;
    Self(unsafe { _mm_load_ps(p) })
  }

  /// Loads the `f32`s reversed, index opposite of lane.
  #[inline(always)]
  pub fn load_reverse(addr: &Align16<[f32; 4]>) -> Self {
    let p = addr as *const Align16<[f32; 4]> as *const f32;
    Self(unsafe { _mm_loadr_ps(p) })
  }

  /// As [load](m128::load), but no alignment requirement.
  #[inline(always)]
  pub fn load_unaligned(addr: &[f32; 4]) -> Self {
    let p = addr as *const [f32; 4] as *const f32;
    Self(unsafe { _mm_loadu_ps(p) })
  }

  /// Loads the `f32` referenced into all lanes.
  #[allow(clippy::trivially_copy_pass_by_ref)]
  #[inline(always)]
  pub fn load_splat(addr: &f32) -> Self {
    Self(unsafe { _mm_load1_ps(addr) })
  }

  /// Loads the `f32` referenced into the lowest lane, others are 0.
  #[allow(clippy::trivially_copy_pass_by_ref)]
  #[inline(always)]
  pub fn load_low(addr: &f32) -> Self {
    Self(unsafe { _mm_load_ss(addr) })
  }

  /// Stores the `f32`s in so that the index matches the lane.
  #[inline(always)]
  pub fn store(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store_ps(p, self.0) };
  }

  /// Stores the `f32`s reversed, index opposite of lane.
  #[inline(always)]
  pub fn store_reverse(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storer_ps(p, self.0) };
  }

  /// As [store](m128::store), but no alignment requirement.
  #[inline(always)]
  pub fn store_unaligned(self, addr: &mut [f32; 4]) {
    let p = addr as *mut [f32; 4] as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storeu_ps(p, self.0) };
  }

  /// Store the lowest lane to all slots in the array.
  #[inline(always)]
  pub fn store_splat(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store1_ps(p, self.0) };
  }

  /// Store the lowest lane to the address.
  #[inline(always)]
  pub fn store_low(self, addr: &mut f32) {
    unsafe { _mm_store_ss(addr, self.0) };
  }

  /// As [store](m128::store), but makes a new array and returns it for you.
  #[inline(always)]
  pub fn to_array(self) -> [f32; 4] {
    let mut a = Align16([0.0_f32; 4]);
    self.store(&mut a);
    a.0
  }

  /// f32x4 lanewise reciprocal approximation: `1.0/self[n]`
  ///
  /// Maximum relative error for the approximation is `1.5*2^-12`.
  #[inline(always)]
  pub fn reciprocal(self) -> Self {
    Self(unsafe { _mm_rcp_ps(self.0) })
  }

  /// f32x4 lanewise reciprocal square root approximation: `1.0/sqrt(self[n])`
  ///
  /// Maximum relative error for the approximation is `1.5*2^-12`.
  #[inline(always)]
  pub fn reciprocal_sqrt(self) -> Self {
    Self(unsafe { _mm_rsqrt_ps(self.0) })
  }

  /// f32x4 lanewise square root.
  #[inline(always)]
  pub fn sqrt(self) -> Self {
    Self(unsafe { _mm_sqrt_ps(self.0) })
  }

  /// low lane is `self+other`, other lanes are just `self`.
  #[inline(always)]
  pub fn add_low(self, other: Self) -> Self {
    Self(unsafe { _mm_add_ss(self.0, other.0) })
  }

  /// low lane is `self/other`, other lanes are `self`.
  #[inline(always)]
  pub fn div_low(self, other: Self) -> Self {
    Self(unsafe { _mm_div_ss(self.0, other.0) })
  }

  /// low lane is `self*other`, other lanes are `self`.
  #[inline(always)]
  pub fn mul_low(self, other: Self) -> Self {
    Self(unsafe { _mm_mul_ss(self.0, other.0) })
  }

  /// As [reciprocal](m128::reciprocal) in the low lane, other lanes unchanged.
  #[inline(always)]
  pub fn reciprocal_low(self) -> Self {
    Self(unsafe { _mm_rcp_ss(self.0) })
  }

  /// As [reciprocal_sqrt](m128::reciprocal_sqrt) in the low lane, other lanes
  /// unchanged.
  #[inline(always)]
  pub fn reciprocal_sqrt_low(self) -> Self {
    Self(unsafe { _mm_rsqrt_ss(self.0) })
  }

  /// square root in the low lane, other lanes unchanged.
  #[inline(always)]
  pub fn sqrt_low(self) -> Self {
    Self(unsafe { _mm_sqrt_ss(self.0) })
  }

  /// `a-b` in the low lane, other lanes unchanged.
  #[inline(always)]
  pub fn sub_low(self, other: Self) -> Self {
    Self(unsafe { _mm_sub_ss(self.0, other.0) })
  }

  /// bitwise `!self & other`.
  #[inline(always)]
  pub fn andnot(self, other: Self) -> Self {
    Self(unsafe { _mm_andnot_ps(self.0, other.0) })
  }

  /// lanewise `self == other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_eq(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpeq_ps(self.0, other.0) })
  }

  /// as [cmp_eq](m128::cmp_eq), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_eq_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpeq_ss(self.0, other.0) })
  }

  /// lanewise `self >= other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_ge(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpge_ps(self.0, other.0) })
  }

  /// as [cmp_ge](m128::cmp_ge), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_ge_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpge_ss(self.0, other.0) })
  }

  /// lanewise `self > other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_gt(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpgt_ps(self.0, other.0) })
  }

  /// as [cmp_gt](m128::cmp_gt), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_gt_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpgt_ss(self.0, other.0) })
  }

  /// lanewise `self <= other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_le(self, other: Self) -> Self {
    Self(unsafe { _mm_cmple_ps(self.0, other.0) })
  }

  /// as [cmp_le](m128::cmp_le), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_le_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmple_ss(self.0, other.0) })
  }

  /// lanewise `self < other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_lt(self, other: Self) -> Self {
    Self(unsafe { _mm_cmplt_ps(self.0, other.0) })
  }

  /// as [cmp_lt](m128::cmp_lt), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_lt_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmplt_ss(self.0, other.0) })
  }

  /// lanewise `self != other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_neq(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpneq_ps(self.0, other.0) })
  }

  /// as [cmp_neq](m128::cmp_neq), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_neq_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpneq_ss(self.0, other.0) })
  }

  /// lanewise `!(self >= other)`, 0 for `false`, all bits for `true`.
  ///
  /// Also this intrinsic triggers 3rd Impact.
  #[inline(always)]
  pub fn cmp_nge(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnge_ps(self.0, other.0) })
  }

  /// as [cmp_nge](m128::cmp_nge), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_nge_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnge_ss(self.0, other.0) })
  }

  /// lanewise `!(self > other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_ngt(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpngt_ps(self.0, other.0) })
  }

  /// as [cmp_ngt](m128::cmp_ngt), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_ngt_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpngt_ss(self.0, other.0) })
  }

  /// lanewise `!(self <= other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nle(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnle_ps(self.0, other.0) })
  }

  /// as [cmp_nle](m128::cmp_nle), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_nle_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnle_ss(self.0, other.0) })
  }

  /// lanewise `!(self < other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nlt(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_ps(self.0, other.0) })
  }

  /// as [cmp_nlt](m128::cmp_nlt), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_nlt_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_ss(self.0, other.0) })
  }

  /// lanewise `self != NaN && other != NaN`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nonnan(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpord_ps(self.0, other.0) })
  }

  /// as [cmp_nonnan](m128::cmp_nonnan), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_nonnan_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpord_ss(self.0, other.0) })
  }

  /// lanewise `self == NaN || other == NaN`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nan(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpunord_ps(self.0, other.0) })
  }

  /// as [cmp_nan](m128::cmp_nan), lowest lane only, other lanes copy `self`.
  #[inline(always)]
  pub fn cmp_nan_low(self, other: Self) -> Self {
    Self(unsafe { _mm_cmpunord_ss(self.0, other.0) })
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self == other`
  #[inline(always)]
  pub fn cmp_int_eq_low(self, other: Self) -> i32 {
    unsafe { _mm_comieq_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self >= other`
  #[inline(always)]
  pub fn cmp_int_ge_low(self, other: Self) -> i32 {
    unsafe { _mm_comige_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self > other`
  #[inline(always)]
  pub fn cmp_int_gt_low(self, other: Self) -> i32 {
    unsafe { _mm_comigt_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self <= other`
  #[inline(always)]
  pub fn cmp_int_le_low(self, other: Self) -> i32 {
    unsafe { _mm_comile_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self < other`
  #[inline(always)]
  pub fn cmp_int_lt_low(self, other: Self) -> i32 {
    unsafe { _mm_comilt_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self != other`
  #[inline(always)]
  pub fn cmp_int_neq_low(self, other: Self) -> i32 {
    unsafe { _mm_comineq_ss(self.0, other.0) }
  }

  /// Rounds `i32` to `f32` and places into the low lane, other lanes copy
  /// `self`.
  #[inline(always)]
  pub fn round_i32_into_low(self, b: i32) -> Self {
    Self(unsafe { _mm_cvt_si2ss(self.0, b) })
  }

  /// Rounds `i64` to `f32` and places into the low lane, other lanes copy
  /// `self`.
  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub fn round_i64_into_low(self, b: i64) -> Self {
    Self(unsafe { _mm_cvtsi64_ss(self.0, b) })
  }

  /// Round the low lane to `i32` and returns it.
  #[inline(always)]
  pub fn round_i32_from_low(self) -> i32 {
    unsafe { _mm_cvt_ss2si(self.0) }
  }

  /// Round the low lane to `i64` and returns it.
  #[inline(always)]
  #[cfg(target_arch = "x86_64")]
  pub fn round_i64_from_low(self) -> i64 {
    unsafe { _mm_cvtss_si64(self.0) }
  }

  /// Truncate the low lane to `i32` and returns it.
  #[inline(always)]
  pub fn truncate_i32_from_low(self) -> i32 {
    unsafe { _mm_cvttss_si32(self.0) }
  }

  /// Truncate the low lane to `i64` and returns it.
  #[cfg(target_arch = "x86_64")]
  #[inline(always)]
  pub fn truncate_i64_from_low(self) -> i64 {
    unsafe { _mm_cvttss_si64(self.0) }
  }

  /// Extracts the lowest lane as an `f32`.
  #[inline(always)]
  pub fn extract_low(self) -> f32 {
    unsafe { _mm_cvtss_f32(self.0) }
  }

  /// f32x4 lanewise maximum of `self` and `other`.
  #[inline(always)]
  pub fn max(self, other: Self) -> Self {
    Self(unsafe { _mm_max_ps(self.0, other.0) })
  }

  /// f32x4 lanewise minimum of `self` and `other`.
  #[inline(always)]
  pub fn min(self, other: Self) -> Self {
    Self(unsafe { _mm_min_ps(self.0, other.0) })
  }

  /// low lane is max of `self` and `other`, other lanes are `self`.
  #[inline(always)]
  pub fn max_low(self, other: Self) -> Self {
    Self(unsafe { _mm_max_ss(self.0, other.0) })
  }

  /// low lane is min of `self` and `other`, other lanes are `self`.
  #[inline(always)]
  pub fn min_low(self, other: Self) -> Self {
    Self(unsafe { _mm_min_ss(self.0, other.0) })
  }

  /// output uses the low lane of `other` and the rest are `self`.
  #[inline(always)]
  pub fn move_low(self, other: Self) -> Self {
    Self(unsafe { _mm_move_ss(self.0, other.0) })
  }

  /// High lanes of `other` become the low lanes of the output, the high lanes
  /// of the output are the high lanes of `self`.
  ///
  /// ```txt
  /// lane:   3  2  1  0
  /// self:  [a, b, c, d]
  /// other: [e, f, g, h]
  /// -------------------
  /// out:   [a, b, e, f]
  /// ```
  #[inline(always)]
  pub fn move_high_low(self, other: Self) -> Self {
    Self(unsafe { _mm_movehl_ps(self.0, other.0) })
  }

  /// Low lanes of `other` become the high lanes of the output, the low lanes of
  /// the output are the low lanes of `self`.
  ///
  /// ```txt
  /// lane:   3  2  1  0
  /// self:  [a, b, c, d]
  /// other: [e, f, g, h]
  /// -------------------
  /// out:   [g, h, c, d]
  /// ```
  #[inline(always)]
  pub fn move_low_high(self, other: Self) -> Self {
    Self(unsafe { _mm_movelh_ps(self.0, other.0) })
  }

  /// Sets bits 0 through 3 of the output based on the most significant bits of
  /// lanes 0 through 3.
  #[inline(always)]
  pub fn move_mask(self) -> i32 {
    unsafe { _mm_movemask_ps(self.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self == other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_eq_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomieq_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self >= other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_ge_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomige_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self > other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_gt_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomigt_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self <= other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_le_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomile_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self < other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_lt_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomilt_ss(self.0, other.0) }
  }

  /// Compare low with int output (0==`false`, 1==`true`): `self != other`
  ///
  /// Won't signal an exception for QNaNs.
  #[inline(always)]
  pub fn ucmp_int_neq_low(self, other: Self) -> i32 {
    unsafe { _mm_ucomineq_ss(self.0, other.0) }
  }

  /// Unpack and interleave high lanes of `self` and `other`
  ///
  /// ```txt
  /// lane:   3  2  1  0
  /// self:  [a, b, c, d]
  /// other: [e, f, g, h]
  /// -------------------
  /// out:   [e, a, f, b]
  /// ```
  #[inline(always)]
  pub fn unpack_high(self, other: Self) -> Self {
    Self(unsafe { _mm_unpackhi_ps(self.0, other.0) })
  }

  /// Unpack and interleave low lanes of `self` and `other`
  ///
  /// ```txt
  /// lane:   3  2  1  0
  /// self:  [a, b, c, d]
  /// other: [e, f, g, h]
  /// -------------------
  /// out:   [g, c, h, d]
  /// ```
  #[inline(always)]
  pub fn unpack_low(self, other: Self) -> Self {
    Self(unsafe { _mm_unpacklo_ps(self.0, other.0) })
  }
}

/// Shuffles `a` and `b` into an output according to the indexes given.
///
/// * The two low lanes come from `a`
/// * The two high lanes come from `b`
/// * Remember that you can pass the same register as both positions if desired.
///
/// The index input literals are ordered so that it lines up the same as when
/// using the [set](crate::arch::x86_64::m128::set) method: highest index to
/// lowest index.
///
/// ```rust
/// #[cfg(target_arch = "x86")]
/// use lokacore::{shuffle128, arch::x86::m128};
/// #[cfg(target_arch = "x86_64")]
/// use lokacore::{shuffle128, arch::x86_64::m128};
///
/// // Indexes are ordered high to low: 3 2 1 0
///
/// let a = m128::set(9.0, 8.0, 7.0, 6.0);
/// let output = shuffle128!(a, a, 0, 1, 3, 2);
/// let expected = m128::set(6.0, 7.0, 9.0, 8.0);
/// assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
///
/// let b = m128::set(12.0, 13.0, 14.0, 15.0);
/// let output = shuffle128!(a, b, 0, 1, 3, 2);
/// let expected = m128::set(15.0, 14.0, 9.0, 8.0);
/// assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
/// ```
#[macro_export]
macro_rules! shuffle128 {
  ($a:ident, $b:ident, $i0:literal, $i1:literal, $i2:literal, $i3:literal) => {{
    const I0: i32 = (($i0 as u8) & 0b11) as i32;
    const I1: i32 = (($i1 as u8) & 0b11) as i32;
    const I2: i32 = (($i2 as u8) & 0b11) as i32;
    const I3: i32 = (($i3 as u8) & 0b11) as i32;
    const IMM8: i32 = (I0 << 6 | I1 << 4 | I2 << 2 | I3) as i32;
    #[cfg(all(target_arch = "x86", target_feature = "sse"))]
    {
      m128(unsafe { core::arch::x86::_mm_shuffle_ps($a.0, $b.0, IMM8) })
    }
    #[cfg(all(target_arch = "x86_64", target_feature = "sse"))]
    {
      m128(unsafe { core::arch::x86_64::_mm_shuffle_ps($a.0, $b.0, IMM8) })
    }
    #[cfg(not(target_feature = "sse"))]
    {
      compile_error!("the shuffle macro requires 'sse' to be enabled.");
    }
  }};
}

/// Treats the inputs as rows of a 4x4 matrix and transposes the matrix.
///
/// ```txt
/// tmp0 := _mm_unpacklo_ps(row0, row1);
/// tmp2 := _mm_unpacklo_ps(row2, row3);
/// tmp1 := _mm_unpackhi_ps(row0, row1);
/// tmp3 := _mm_unpackhi_ps(row2, row3);
/// row0 := _mm_movelh_ps(tmp0, tmp2);
/// row1 := _mm_movehl_ps(tmp2, tmp0);
/// row2 := _mm_movelh_ps(tmp1, tmp3);
/// row3 := _mm_movehl_ps(tmp3, tmp1);
/// ```
#[inline(always)]
pub fn transpose4(row0: &mut m128, row1: &mut m128, row2: &mut m128, row3: &mut m128) {
  unsafe { _MM_TRANSPOSE4_PS(&mut row0.0, &mut row1.0, &mut row2.0, &mut row3.0) }
}

const ALL_EXCEPTIONS: u32 = _MM_EXCEPT_INVALID
  | _MM_EXCEPT_DENORM
  | _MM_EXCEPT_DIV_ZERO
  | _MM_EXCEPT_OVERFLOW
  | _MM_EXCEPT_UNDERFLOW
  | _MM_EXCEPT_INEXACT;

/// Reads the `MXCSR` control and status register.
#[allow(bad_style)]
#[inline(always)]
pub fn get_MXCSR() -> u32 {
  unsafe { _mm_getcsr() }
}

/// Sets the `MXCSR` control and status register.
///
/// Modifications to this register only affect the current thread.
///
/// TODO: make this safe wrapped.
#[allow(bad_style)]
#[inline(always)]
pub unsafe fn set_MXCSR(val: u32) {
  _mm_setcsr(val)
}

/// Which exceptions are masked (ignored).
#[inline(always)]
pub fn get_exception_mask() -> u32 {
  unsafe { _MM_GET_EXCEPTION_MASK() }
}

/// Sets the [exception
/// mask](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#masking-flags)
///
/// **WARNING:** an unmasked exception calls the exception handler, and the
/// standard exception handler in Rust will simply terminate _the entire
/// process_.
#[inline(always)]
pub unsafe fn set_exception_mask(mask: u32) {
  if mask & !ALL_EXCEPTIONS > 0 {
    panic!("Illegal exception mask input: {}", mask)
  } else {
    _MM_SET_EXCEPTION_MASK(mask)
  }
}

/// Gets the current exception status.
#[inline(always)]
pub fn get_exception_state() -> u32 {
  unsafe { _MM_GET_EXCEPTION_STATE() }
}

/// Sets the current [exception
/// state](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#exception-flags)
#[inline(always)]
pub fn set_exception_state(state: u32) {
  if state & !ALL_EXCEPTIONS > 0 {
    panic!("Illegal exception state input: {}", state)
  } else {
    unsafe { _MM_SET_EXCEPTION_STATE(state) }
  }
}

/// If values to be denormalized should be set to zero instead.
///
/// Off by default.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum FlushZeroMode {
  /// Flush to zero.
  On = _MM_FLUSH_ZERO_ON,
  /// Don't flush to zero.
  Off = _MM_FLUSH_ZERO_OFF,
}
impl Default for FlushZeroMode {
  fn default() -> Self {
    FlushZeroMode::Off
  }
}

/// Gets the [flush to zero
/// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#denormals-are-zeroflush-to-zero-mode).
///
/// Off by default. If it's on then values that would be denormalized are set to
/// zero instead.
#[inline(always)]
pub fn get_flush_zero_mode() -> FlushZeroMode {
  match unsafe { _MM_GET_FLUSH_ZERO_MODE() } {
    _MM_FLUSH_ZERO_ON => FlushZeroMode::On,
    _MM_FLUSH_ZERO_OFF => FlushZeroMode::Off,
    v => panic!("CPU returned illegal flush zero mode value: {}", v),
  }
}

/// Sets the [flush to zero
/// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#denormals-are-zeroflush-to-zero-mode)
///
/// Off by default. If it's on then values that would be denormalized are set to
/// zero instead.
#[inline(always)]
pub fn set_flush_zero_mode(mode: FlushZeroMode) {
  unsafe { _MM_SET_FLUSH_ZERO_MODE(mode as u32) }
}

/// The rounding mode for whenever a value has to be rounded.
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RoundingMode {
  /// Default, Rounds to the closest possible value. If two values are equally
  /// close, round toward even.
  Nearest = _MM_ROUND_NEAREST,
  /// Round to negative infinity.
  Down = _MM_ROUND_DOWN,
  /// Round to positive infinity.
  Up = _MM_ROUND_UP,
  /// Round toward zero (truncate).
  TowardZero = _MM_ROUND_TOWARD_ZERO,
}
impl Default for RoundingMode {
  fn default() -> Self {
    RoundingMode::Nearest
  }
}

/// The current rounding mode.
#[inline(always)]
pub fn get_rounding_mode() -> RoundingMode {
  match unsafe { _MM_GET_ROUNDING_MODE() } {
    _MM_ROUND_NEAREST => RoundingMode::Nearest,
    _MM_ROUND_DOWN => RoundingMode::Down,
    _MM_ROUND_UP => RoundingMode::Up,
    _MM_ROUND_TOWARD_ZERO => RoundingMode::TowardZero,
    v => panic!("CPU returned illegal rounding mode value: {}", v),
  }
}

/// Sets the [rounding
/// mode](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html#rounding-mode)
/// of the current thread.
#[inline(always)]
pub fn set_rounding_mode(mode: RoundingMode) {
  unsafe { _MM_SET_ROUNDING_MODE(mode as u32) }
}

/// Prefetch data to all cache levels.
///
/// This hints to the CPU that the cache line that contains whatever data this
/// pointer points to should be fetched because it will be needed soon. It's
/// only a hint, the actual implementation depends on the particular CPU. An
/// invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
#[inline(always)]
pub fn prefetch_T0(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T0) }
}

/// Prefetch data to L2 cache and higher.
///
/// This hints to the CPU that the cache line that contains whatever data this
/// pointer points to should be fetched because it will be needed soon. It's
/// only a hint, the actual implementation depends on the particular CPU. An
/// invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
#[inline(always)]
pub fn prefetch_T1(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T1) }
}

/// Prefetch data to L3 cache and higher (or L2 on some systems).
///
/// This hints to the CPU that the cache line that contains whatever data this
/// pointer points to should be fetched because it will be needed soon. It's
/// only a hint, the actual implementation depends on the particular CPU. An
/// invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
#[inline(always)]
pub fn prefetch_T2(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T2) }
}

/// Non-temporal prefetch hint.
///
/// This is the least amount of hinting, but can avoid polluting the cache
/// accidentally (eg: if the target data ends up not being used). It's only a
/// hint, the actual implementation depends on the particular CPU. An invalid
/// pointer will not cause UB but it can dramatically _reduce_ performance, so
/// be mindful.
#[inline(always)]
pub fn prefetch_nontemporal(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_NTA) }
}

/// As
/// [`_mm_sfence`](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_sfence.html).
///
/// This forces all store-to-memory operations before this to be globally
/// visible before any such operations after.
#[inline(always)]
pub fn store_fence() {
  unsafe { _mm_sfence() }
}
