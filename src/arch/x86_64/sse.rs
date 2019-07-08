use super::*;

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
/// // Indexes are ordered high to low: 3 2 1 0
/// 
/// use lokacore::{shuffle, arch::x86_64::m128};
/// let a = m128::set(9.0, 8.0, 7.0, 6.0);
/// let output = shuffle!(a, a, 0, 1, 3, 2);
/// let expected = m128::set(6.0, 7.0, 9.0, 8.0);
/// assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
///
/// let b = m128::set(12.0, 13.0, 14.0, 15.0);
/// let output = shuffle!(a, b, 0, 1, 3, 2);
/// let expected = m128::set(15.0, 14.0, 9.0, 8.0);
/// assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
/// ```
#[macro_export]
macro_rules! shuffle {
  ($a:ident, $b:ident, $i0:literal, $i1:literal, $i2:literal, $i3:literal) => {{
    const I0: i32 = (($i0 as u8) & 0b11) as i32;
    const I1: i32 = (($i1 as u8) & 0b11) as i32;
    const I2: i32 = (($i2 as u8) & 0b11) as i32;
    const I3: i32 = (($i3 as u8) & 0b11) as i32;
    const IMM8: i32 = (I0 << 6 | I1 << 4 | I2 << 2 | I3) as i32;
    #[cfg(all(target_arch = "x86", target_feature="sse"))]
    {
      m128(unsafe { core::arch::x86::_mm_shuffle_ps($a.0, $b.0, IMM8) })
    }
    #[cfg(all(target_arch = "x86_64", target_feature="sse"))]
    {
      m128(unsafe { core::arch::x86_64::_mm_shuffle_ps($a.0, $b.0, IMM8) })
    }
    #[cfg(not(target_feature="sse"))]
    {
      compile_error!("the shuffle macro requires 'sse' to be enabled.");
    }
  }};
}

#[test]
fn test_shuffle() {
  let a = m128::set(9.0, 8.0, 7.0, 6.0);

  let output = shuffle!(a, a, 0, 0, 0, 0);
  let expected = m128::set_all(6.0);
  assert_eq!(0b1111, expected.cmp_eq(output).move_mask());

  let output = shuffle!(a, a, 0, 0, 0, 1);
  let expected = m128::set(6.0, 6.0, 6.0, 7.0);
  assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
}

/// Reads the `MXCSR` control and status register.
#[allow(bad_style)]
pub fn get_MXCSR() -> u32 {
  unsafe { _mm_getcsr() }
}

/// Which exceptions are masked (ignored).
pub fn get_exception_mask() -> u32 {
  unsafe { _MM_GET_EXCEPTION_MASK() }
}

/// Gets the current exception status.
pub fn get_exception_state() -> u32 {
  unsafe { _MM_GET_EXCEPTION_STATE() }
}

/// If values that would be denormalized are set to zero instead. 
pub fn get_flush_zero_mode() -> u32 {
  unsafe { _MM_GET_FLUSH_ZERO_MODE() }
}

/// The current rounding mode.
pub fn get_rounding_mode() -> u32 {
  unsafe { _MM_GET_ROUNDING_MODE() }
}

/// As
/// [_mm_sfence](https://doc.rust-lang.org/core/arch/x86_64/fn._mm_sfence.html),
/// just marked safe. This forces all store-to-memory operations before this to
/// be globally visible before any such operations after.
pub fn store_fence() {
  unsafe { _mm_sfence() }
}

/// Hints to the CPU that the cache line that this pointer is part of should be
/// fetched into all levels of the cache hierarchy.
///
/// It's only a hint, the actual implementation depends on the particular CPU.
/// An invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
pub fn prefetch_T0(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T0) }
}

/// Hints to the CPU that the cache line that this pointer is part of should be
/// fetched into L2 cache and higher.
///
/// It's only a hint, the actual implementation depends on the particular CPU.
/// An invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
pub fn prefetch_T1(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T1) }
}

/// Hints to the CPU that the cache line that this pointer is part of should be
/// fetched into L3 cache and higher (or L2 if there's no L3).
///
/// It's only a hint, the actual implementation depends on the particular CPU.
/// An invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
#[allow(bad_style)]
pub fn prefetch_T2(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_T2) }
}

/// Hints to the CPU that the cache line that this pointer is part of should be
/// fetched using non-temporal access. This can bring it closer without
/// polluting the cache.
///
/// It's only a hint, the actual implementation depends on the particular CPU.
/// An invalid pointer will not cause UB but it can dramatically _reduce_
/// performance, so be mindful.
pub fn prefetch_nontemporal(p: *const impl Sized) {
  unsafe { _mm_prefetch(p as *const i8, _MM_HINT_NTA) }
}

// TODO: https://doc.rust-lang.org/core/arch/x86_64/fn._mm_prefetch.html

// TODO: https://doc.rust-lang.org/core/arch/x86_64/fn._mm_setcsr.html

// TODO: https://doc.rust-lang.org/core/arch/x86_64/fn._mm_sfence.html

// TODO: https://doc.rust-lang.org/core/arch/x86_64/fn._mm_shuffle_ps.html

// TODO: Get/Set Exception Mask

// TODO: Get/Set Exception State

// TODO: Get/Set Flush Zero Mode

// TODO: Get/Set Rounding Mode

// TODO: Sort all methods into blocks with big labels so that people can find
// what they want within the rustdoc easier.

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
pub fn transpose4(row0: &mut m128, row1: &mut m128, row2: &mut m128, row3: &mut m128) {
  unsafe { _MM_TRANSPOSE4_PS(&mut row0.0, &mut row1.0, &mut row2.0, &mut row3.0) }
}

impl m128 {
  /// Sets the floats into a register, high to low. The first argument is the
  /// "highest" lane index (bits 96..=127), and arguments after that proceed
  /// down from there.
  pub fn set(e3: f32, e2: f32, e1: f32, e0: f32) -> Self {
    m128(unsafe { _mm_set_ps(e3, e2, e1, e0) })
  }

  /// Sets the floats into a register, low to high. The first argument is the
  /// "lowest" lane index (bits 0..=31), and arguments after that proceed up
  /// from there.
  pub fn setr(e3: f32, e2: f32, e1: f32, e0: f32) -> Self {
    m128(unsafe { _mm_setr_ps(e3, e2, e1, e0) })
  }

  /// Sets the given value as all lanes in the register.
  pub fn set_all(f: f32) -> Self {
    m128(unsafe { _mm_set1_ps(f) })
  }

  /// Sets the given value as the lowest lane, other lanes zero.
  pub fn set_single(f: f32) -> Self {
    m128(unsafe { _mm_set_ss(f) })
  }

  /// Loads an array of 16-byte aligned `f32` values, with each index matching
  /// each lane.
  pub fn load(al_fs_ref: &Align16<[f32; 4]>) -> Self {
    // TODO: TEST THAT INDEX 0 GOES INTO LANE 0.
    let p = al_fs_ref as *const Align16<[f32; 4]> as *const f32;
    debug_assert!(p as usize % 16 == 0);
    m128(unsafe { _mm_load_ps(p) })
  }

  /// Loads an array of 16-byte aligned `f32` values, with reverse ordering.
  pub fn loadr(al_fs_ref: &Align16<[f32; 4]>) -> Self {
    // TODO: TEST THAT INDEX 3 GOES INTO LANE 0.
    let p = al_fs_ref as *const Align16<[f32; 4]> as *const f32;
    debug_assert!(p as usize % 16 == 0);
    m128(unsafe { _mm_loadr_ps(p) })
  }

  /// Loads an array of `f32` values, without any required alignment.
  pub fn loadu(fs_ref: &[f32; 4]) -> Self {
    let p = fs_ref as *const [f32; 4] as *const f32;
    debug_assert!(p as usize % 16 == 0);
    m128(unsafe { _mm_loadr_ps(p) })
  }

  /// Loads the `f32` referenced into all lanes.
  pub fn load_all(f_ref: &f32) -> Self {
    m128(unsafe { _mm_load1_ps(f_ref) })
  }

  /// Loads the `f32` referenced into the lowest lane, others are 0.
  pub fn load_single(f_ref: &f32) -> Self {
    m128(unsafe { _mm_load_ss(f_ref) })
  }

  /// Returns a register with all lanes zero.
  pub fn zeroed() -> Self {
    m128(unsafe { _mm_setzero_ps() })
  }

  /// Store the lowest lane to all slots in the array.
  pub fn store_all(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store1_ps(p, self.0) };
  }

  /// Store the lanes into the slots of the array. Lowest lane to lowest index,
  /// and so on.
  pub fn store(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store_ps(p, self.0) };
  }

  /// Store the lanes into the slots of the array. Highest lane to lowest index,
  /// and so on.
  pub fn storer(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storer_ps(p, self.0) };
  }

  /// Store the lanes into the slots of the array with a non-temporal memory
  /// hint. Lowest lane to lowest index, and so on.
  pub fn store_nontemporal(self, addr: &mut Align16<[f32; 4]>) {
    let p = addr as *mut Align16<[f32; 4]> as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_stream_ps(p, self.0) };
  }

  /// Store the lanes into the slots of the array. Lowest lane to lowest index,
  /// and so on.
  pub fn storeu(self, addr: &mut [f32; 4]) {
    let p = addr as *mut [f32; 4] as *mut f32;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storeu_ps(p, self.0) };
  }

  /// Store the lowest lane to the address.
  pub fn store_single(self, addr: &mut f32) {
    unsafe { _mm_store_ss(addr, self.0) };
  }

  /// f32x4 lanewise addition
  pub fn add(self, other: m128) -> m128 {
    m128(unsafe { _mm_add_ps(self.0, other.0) })
  }

  /// low lane is `self+other`, other lanes are just `self`
  pub fn add_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_add_ss(self.0, other.0) })
  }

  /// bitwise `self & other`.
  pub fn and(self, other: m128) -> m128 {
    m128(unsafe { _mm_and_ps(self.0, other.0) })
  }

  /// bitwise `!self & other`.
  pub fn andnot(self, other: m128) -> m128 {
    m128(unsafe { _mm_andnot_ps(self.0, other.0) })
  }

  /// lanewise `self == other`, 0 for `false`, all bits for `true`.
  pub fn cmp_eq(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpeq_ps(self.0, other.0) })
  }

  /// as [cmp_eq](m128::cmp_eq), lowest lane only, other lanes copy `self`.
  pub fn cmp_eq_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpeq_ss(self.0, other.0) })
  }

  /// lanewise `self >= other`, 0 for `false`, all bits for `true`.
  pub fn cmp_ge(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpge_ps(self.0, other.0) })
  }

  /// as [cmp_ge](m128::cmp_ge), lowest lane only, other lanes copy `self`.
  pub fn cmp_ge_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpge_ss(self.0, other.0) })
  }

  /// lanewise `self > other`, 0 for `false`, all bits for `true`.
  pub fn cmp_gt(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpgt_ps(self.0, other.0) })
  }

  /// as [cmp_gt](m128::cmp_gt), lowest lane only, other lanes copy `self`.
  pub fn cmp_gt_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpgt_ss(self.0, other.0) })
  }

  /// lanewise `self <= other`, 0 for `false`, all bits for `true`.
  pub fn cmp_le(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmple_ps(self.0, other.0) })
  }

  /// as [cmp_le](m128::cmp_le), lowest lane only, other lanes copy `self`.
  pub fn cmp_le_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmple_ss(self.0, other.0) })
  }

  /// lanewise `self < other`, 0 for `false`, all bits for `true`.
  pub fn cmp_lt(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmplt_ps(self.0, other.0) })
  }

  /// as [cmp_lt](m128::cmp_lt), lowest lane only, other lanes copy `self`.
  pub fn cmp_lt_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmplt_ss(self.0, other.0) })
  }

  /// lanewise `self != other`, 0 for `false`, all bits for `true`.
  pub fn cmp_neq(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpneq_ps(self.0, other.0) })
  }

  /// as [cmp_neq](m128::cmp_neq), lowest lane only, other lanes copy `self`.
  pub fn cmp_neq_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpneq_ss(self.0, other.0) })
  }

  /// lanewise `!(self >= other)`, 0 for `false`, all bits for `true`.
  ///
  /// Also this intrinsic triggers 3rd Impact _every time you use it_.
  pub fn cmp_nge(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnge_ps(self.0, other.0) })
  }

  /// as [cmp_nge](m128::cmp_nge), lowest lane only, other lanes copy `self`.
  pub fn cmp_nge_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnge_ss(self.0, other.0) })
  }

  /// lanewise `!(self > other)`, 0 for `false`, all bits for `true`.
  pub fn cmp_ngt(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpngt_ps(self.0, other.0) })
  }

  /// as [cmp_ngt](m128::cmp_ngt), lowest lane only, other lanes copy `self`.
  pub fn cmp_ngt_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpngt_ss(self.0, other.0) })
  }

  /// lanewise `!(self <= other)`, 0 for `false`, all bits for `true`.
  pub fn cmp_nle(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnle_ps(self.0, other.0) })
  }

  /// as [cmp_nle](m128::cmp_nle), lowest lane only, other lanes copy `self`.
  pub fn cmp_nle_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnle_ss(self.0, other.0) })
  }

  /// lanewise `!(self < other)`, 0 for `false`, all bits for `true`.
  pub fn cmp_nlt(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnlt_ps(self.0, other.0) })
  }

  /// as [cmp_nlt](m128::cmp_nlt), lowest lane only, other lanes copy `self`.
  pub fn cmp_nlt_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpnlt_ss(self.0, other.0) })
  }

  /// lanewise `self != NaN && other != NaN`, 0 for `false`, all bits for `true`.
  pub fn cmp_nonnan(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpord_ps(self.0, other.0) })
  }

  /// as [cmp_nonnan](m128::cmp_nonnan), lowest lane only, other lanes copy `self`.
  pub fn cmp_nonnan_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpord_ss(self.0, other.0) })
  }

  /// lanewise `self == NaN || other == NaN`, 0 for `false`, all bits for `true`.
  pub fn cmp_nan(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpunord_ps(self.0, other.0) })
  }

  /// as [cmp_nan](m128::cmp_nan), lowest lane only, other lanes copy `self`.
  pub fn cmp_nan_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_cmpunord_ss(self.0, other.0) })
  }

  /// Compares lowest lane, `self==other`, 0 for `false`, 1 for `true`.
  pub fn comi_eq_single(self, other: m128) -> i32 {
    unsafe { _mm_comieq_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self>=other`, 0 for `false`, 1 for `true`.
  pub fn comi_ge_single(self, other: m128) -> i32 {
    unsafe { _mm_comige_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self>other`, 0 for `false`, 1 for `true`.
  pub fn comi_gt_single(self, other: m128) -> i32 {
    unsafe { _mm_comigt_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self<=other`, 0 for `false`, 1 for `true`.
  pub fn comi_le_single(self, other: m128) -> i32 {
    unsafe { _mm_comile_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self<other`, 0 for `false`, 1 for `true`.
  pub fn comi_lt_single(self, other: m128) -> i32 {
    unsafe { _mm_comilt_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self!=other`, 0 for `false`, 1 for `true`.
  pub fn comi_neq_single(self, other: m128) -> i32 {
    unsafe { _mm_comineq_ss(self.0, other.0) }
  }

  /// Converts the `i32` into the lowest lane, other lanes copy `self`.
  pub fn cvt_i32f32_single(self, b: i32) -> m128 {
    m128(unsafe { _mm_cvt_si2ss(self.0, b) })
  }

  /// Converts the `i64` into the lowest lane, other lanes copy `self`.
  pub fn cvt_i64f32_single(self, b: i64) -> m128 {
    m128(unsafe { _mm_cvtsi64_ss(self.0, b) })
  }

  /// Round (by mode) the lowest lane to `i32` and returns it.
  pub fn cvt_f32i32_single(self) -> i32 {
    unsafe { _mm_cvt_ss2si(self.0) }
  }

  /// Round (by mode) the lowest lane to `i64` and returns it.
  pub fn cvt_f32i64_single(self) -> i64 {
    unsafe { _mm_cvtss_si64(self.0) }
  }

  /// Truncate the lowest lane to `i32` and returns it.
  pub fn cvt_f32i32_single_trunc(self) -> i32 {
    unsafe { _mm_cvttss_si32(self.0) }
  }

  /// Truncate the lowest lane to `i64` and returns it.
  pub fn cvt_f32i64_single_trunc(self) -> i64 {
    unsafe { _mm_cvttss_si64(self.0) }
  }

  /// Extracts the lowest lane as an `f32`.
  pub fn cvt_f32_single(self) -> f32 {
    unsafe { _mm_cvtss_f32(self.0) }
  }

  /// f32x4 lanewise division.
  pub fn div(self, other: m128) -> m128 {
    m128(unsafe { _mm_div_ps(self.0, other.0) })
  }

  /// low lane is `self/other`, other lanes are `self`.
  pub fn div_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_div_ss(self.0, other.0) })
  }

  /// f32x4 lanewise maximum of `self` and `other`
  pub fn max(self, other: m128) -> m128 {
    m128(unsafe { _mm_max_ps(self.0, other.0) })
  }

  /// low lane is max of `self` and `other`, other lanes are `self`.
  pub fn max_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_max_ss(self.0, other.0) })
  }

  /// f32x4 lanewise minimum of `self` and `other`
  pub fn min(self, other: m128) -> m128 {
    m128(unsafe { _mm_min_ps(self.0, other.0) })
  }

  /// low lane is min of `self` and `other`, other lanes are `self`.
  pub fn min_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_min_ss(self.0, other.0) })
  }

  /// output uses the low lane of `other` and the rest are `self`.
  pub fn move_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_move_ss(self.0, other.0) })
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
  pub fn move_high_low(self, other: m128) -> m128 {
    m128(unsafe { _mm_movehl_ps(self.0, other.0) })
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
  pub fn move_low_high(self, other: m128) -> m128 {
    m128(unsafe { _mm_movelh_ps(self.0, other.0) })
  }

  /// Sets bits 0 through 3 of the output based on the most significant bits of
  /// lanes 0 through 3.
  pub fn move_mask(self) -> i32 {
    unsafe { _mm_movemask_ps(self.0) }
  }

  /// f32x4 lanewise multiplication.
  pub fn mul(self, other: m128) -> m128 {
    m128(unsafe { _mm_mul_ps(self.0, other.0) })
  }

  /// low lane is `self*other`, other lanes are `self`.
  pub fn mul_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_mul_ss(self.0, other.0) })
  }

  /// bitwise `self | other`.
  pub fn or(self, other: m128) -> m128 {
    m128(unsafe { _mm_or_ps(self.0, other.0) })
  }

  /// f32x4 lanewise reciprocal approximation: `1.0/self[n]`
  ///
  /// Maximum relative error for the approximation is `1.5*2^-12`.
  pub fn reciprocal(self) -> m128 {
    m128(unsafe { _mm_rcp_ps(self.0) })
  }

  /// As [reciprocal](m128::reciprocal) in the low lane, other lanes unchanged.
  pub fn reciprocal_single(self) -> m128 {
    m128(unsafe { _mm_rcp_ss(self.0) })
  }

  /// f32x4 lanewise reciprocal square root approximation: `1.0/sqrt(self[n])`
  ///
  /// Maximum relative error for the approximation is `1.5*2^-12`.
  pub fn reciprocal_sqrt(self) -> m128 {
    m128(unsafe { _mm_rsqrt_ps(self.0) })
  }

  /// As [reciprocal_sqrt](m128::reciprocal_sqrt) in the low lane, other lanes
  /// unchanged.
  pub fn reciprocal_sqrt_single(self) -> m128 {
    m128(unsafe { _mm_rsqrt_ss(self.0) })
  }

  /// f32x4 lanewise square root.
  pub fn sqrt(self) -> m128 {
    m128(unsafe { _mm_sqrt_ps(self.0) })
  }

  /// square root in the low lane, other lanes unchanged.
  pub fn sqrt_single(self) -> m128 {
    m128(unsafe { _mm_sqrt_ss(self.0) })
  }

  /// f32x4 lanewise subtraction, `self - other`.
  pub fn sub(self, other: m128) -> m128 {
    m128(unsafe { _mm_sub_ps(self.0, other.0) })
  }

  /// `a-b` in the low lane, other lanes unchanged.
  pub fn sub_single(self, other: m128) -> m128 {
    m128(unsafe { _mm_sub_ss(self.0, other.0) })
  }

  /// Compares lowest lane, `self==other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_eq_single(self, other: m128) -> i32 {
    unsafe { _mm_ucomieq_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self>=other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_ge_single(self, other: m128) -> i32 {
    unsafe { _mm_ucomige_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self>other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_gt_single(self, other: m128) -> i32 {
    unsafe { _mm_ucomigt_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self<=other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_le_single(self, other: m128) -> i32 {
    unsafe { _mm_ucomile_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self<other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_lt_single(self, other: m128) -> i32 {
    unsafe { _mm_ucomilt_ss(self.0, other.0) }
  }

  /// Compares lowest lane, `self!=other`, 0 for `false`, 1 for `true`.
  ///
  /// "Won't signal an exception for QNaNs."
  pub fn ucomi_neq_single(self, other: m128) -> i32 {
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
  pub fn unpack_high(self, other: m128) -> m128 {
    m128(unsafe { _mm_unpackhi_ps(self.0, other.0) })
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
  pub fn unpack_low(self, other: m128) -> m128 {
    m128(unsafe { _mm_unpacklo_ps(self.0, other.0) })
  }

  /// bitwise `self XOR other`.
  pub fn xor(self, other: m128) -> m128 {
    m128(unsafe { _mm_xor_ps(self.0, other.0) })
  }
}
