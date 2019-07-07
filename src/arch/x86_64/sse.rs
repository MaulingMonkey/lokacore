use super::*;

/// # Constructors
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
  pub fn set1(f: f32) -> Self {
    m128(unsafe { _mm_set1_ps(f) })
  }

  /// Sets the given value as the lowest lane, other lanes zero.
  pub fn set_single(f: f32) -> Self {
    m128(unsafe { _mm_set_ss(f) })
  }

  /// Returns a register with all lanes zero.
  pub fn zeroed() -> Self {
    m128(unsafe { _mm_setzero_ps() })
  }

  /// Loads the `f32` referenced into all lanes.
  pub fn load1(f_ref: &f32) -> Self {
    m128(unsafe { _mm_load1_ps(f_ref) })
  }

  /// Loads an array of 16-byte aligned `f32` values.
  pub fn load(al_fs: Align16<[f32; 4]>) -> Self {
    m128(unsafe { _mm_load_ps(al_fs.0.as_ptr()) })
  }

  /// Loads the `f32` referenced into the lowest lane, others are 0.
  pub fn load_single(f_ref: &f32) -> Self {
    m128(unsafe { _mm_load_ss(f_ref) })
  }
}

/// # Operations (TODO: sort these better)
impl m128 {
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

  /// Reads the `MXCSR` control and status register.
  #[allow(bad_style)]
  pub fn get_MXCSR() -> u32 {
    unsafe { _mm_getcsr() }
  }
}
