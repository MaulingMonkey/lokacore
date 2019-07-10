#![allow(clippy::too_many_arguments)]

use super::*;

/// # SSE2 Operations
impl m128 {
  /// lanewise convert the `f32` values into `i32`.
  #[inline(always)]
  pub fn convert_i32(self) -> m128i {
    m128i(unsafe { _mm_cvtps_epi32(self.0) })
  }

  /// lanewise truncate the `f32` values into `i32`.
  #[inline(always)]
  pub fn truncate_i32(self) -> m128i {
    m128i(unsafe { _mm_cvttps_epi32(self.0) })
  }

  /// Casts this value into an `m128i`
  #[inline(always)]
  pub fn cast_to_m128i(self) -> m128i {
    m128i(unsafe { _mm_castps_si128(self.0) })
  }

  /// Converts the low `f64` to `f32` and replaces the lowest lane (other lanes
  /// are unchanged).
  #[inline(always)]
  pub fn convert_replace_low_f64(self, other: m128d) -> m128 {
    m128(unsafe { _mm_cvtsd_ss(self.0, other.0) })
  }

  /// Converts the low `f64` to `f32` and stores in the lowest lane. Other lanes
  /// zero.
  #[inline(always)]
  pub fn convert_single_f64(reg: m128d) -> m128 {
    m128(unsafe { _mm_cvtpd_ps(reg.0) })
  }

  /// Casts this value into an `m128d`
  #[inline(always)]
  pub fn cast_m128d(self) -> m128d {
    m128d(unsafe { _mm_castps_pd(self.0) })
  }

  /// Convert the lower `f32` lanes to `f64` values.
  #[inline(always)]
  pub fn convert_low_f32_f64(self) -> m128d {
    m128d(unsafe { _mm_cvtps_pd(self.0) })
  }
}

/// # SSE2 Operations
impl m128d {
  /// Convert the `f64` values into `i32`, then place into the two lower `i32`
  /// lanes of an `m128i` (the other lanes are zero).
  #[inline(always)]
  pub fn convert_i32(self) -> m128i {
    m128i(unsafe { _mm_cvtpd_epi32(self.0) })
  }

  /// Convert the low lane `f64` value into `i32`
  #[inline(always)]
  pub fn convert_i32_single(self) -> i32 {
    unsafe { _mm_cvtsd_si32(self.0) }
  }

  /// Truncates the `f64` values into `i32`, then place into the two lower `i32`
  /// lanes of an `m128i` (the other lanes are zero).
  #[inline(always)]
  pub fn truncate_i32(self) -> m128i {
    m128i(unsafe { _mm_cvttpd_epi32(self.0) })
  }

  /// Truncates the low `f64` values into `i32` and return it.
  #[inline(always)]
  pub fn truncate_i32_single(self) -> i32 {
    unsafe { _mm_cvttsd_si32(self.0) }
  }

  /// Convert the low lane `f64` value into `i64`
  #[cfg(target_arch = "x86_64")]
  #[inline(always)]
  pub fn convert_i64_single(self) -> i64 {
    unsafe { _mm_cvtsd_si64(self.0) }
  }

  /// Truncates the low `f64` values into `i64` and return it.
  #[cfg(target_arch = "x86_64")]
  #[inline(always)]
  pub fn truncate_i64_single(self) -> i64 {
    unsafe { _mm_cvttsd_si64(self.0) }
  }

  /// Casts this value into an `m128i`
  #[inline(always)]
  pub fn cast_m128i(self) -> m128i {
    m128i(unsafe { _mm_castpd_si128(self.0) })
  }

  /// Extracts the low `f64` lane.
  #[inline(always)]
  pub fn extract_low_f64(self) -> f64 {
    unsafe { _mm_cvtsd_f64(self.0) }
  }

  /// Casts this value into an `m128`
  #[inline(always)]
  pub fn cast_m128(self) -> m128 {
    m128(unsafe { _mm_castpd_ps(self.0) })
  }

  /// bitwise `!self & other`.
  #[inline(always)]
  pub fn andnot(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_andnot_pd(self.0, other.0) })
  }

  /// lanewise `self == other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_eq(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpeq_pd(self.0, other.0) })
  }

  /// lanewise `self >= other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_ge(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpge_pd(self.0, other.0) })
  }

  /// lanewise `self > other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_gt(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpgt_pd(self.0, other.0) })
  }

  /// lanewise `self <= other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_le(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmple_pd(self.0, other.0) })
  }

  /// lanewise `self < other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_lt(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmplt_pd(self.0, other.0) })
  }

  /// lanewise `self != other`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_neq(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpneq_pd(self.0, other.0) })
  }

  /// lanewise `!(self >= other)`, 0 for `false`, all bits for `true`.
  ///
  /// Also this intrinsic triggers 3rd Impact _every time you use it_.
  #[inline(always)]
  pub fn cmp_nge(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpnge_pd(self.0, other.0) })
  }

  /// lanewise `!(self > other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_ngt(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpngt_pd(self.0, other.0) })
  }

  /// lanewise `!(self <= other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nle(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpnle_pd(self.0, other.0) })
  }

  /// lanewise `!(self < other)`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nlt(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpnlt_pd(self.0, other.0) })
  }

  /// lanewise `self != NaN && other != NaN`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nonnan(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpord_pd(self.0, other.0) })
  }

  /// lanewise `self == NaN || other == NaN`, 0 for `false`, all bits for `true`.
  #[inline(always)]
  pub fn cmp_nan(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_cmpunord_pd(self.0, other.0) })
  }

  /// Loads the `f64` into both lanes.
  #[inline(always)]
  pub fn load_all(addr: &f64) -> m128d {
    m128d(unsafe { _mm_load1_pd(addr) })
  }

  /// Loads the `f64` values, high index to low index.
  #[inline(always)]
  pub fn load(arr: &Align16<[f64; 2]>) -> m128d {
    // TODO: TEST THAT INDEX 0 GOES INTO LANE 0.
    let p = arr as *const Align16<[f64; 2]> as *const f64;
    debug_assert!(p as usize % 16 == 0);
    m128d(unsafe { _mm_load_pd(p) })
  }

  /// Overwrite the upper lane with the new value.
  #[inline(always)]
  pub fn load_high(self, new: &f64) -> m128d {
    m128d(unsafe { _mm_loadh_pd (self.0, new) })
  }

  /// Overwrite the lower lane with the new value.
  #[inline(always)]
  pub fn load_load(self, new: &f64) -> m128d {
    m128d(unsafe { _mm_loadl_pd  (self.0, new) })
  }

  /// Loads the `f64` values, low index to high index.
  #[inline(always)]
  pub fn loadr(arr: &Align16<[f64; 2]>) -> m128d {
    // TODO: TEST THAT INDEX 0 GOES INTO LANE 0.
    let p = arr as *const Align16<[f64; 2]> as *const f64;
    debug_assert!(p as usize % 16 == 0);
    m128d(unsafe { _mm_loadr_pd (p) })
  }

  /// Loads the `f64` values, high index to low index.
  #[inline(always)]
  pub fn loadu(arr: &[f64; 2]) -> m128d {
    // TODO: TEST THAT INDEX 0 GOES INTO LANE 0.
    let p = arr as *const[f64; 2] as *const f64;
    debug_assert!(p as usize % 16 == 0);
    m128d(unsafe { _mm_loadu_pd (p) })
  }

  /// lanewise f64x2 maximum
  #[inline(always)]
  pub fn max(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_max_pd(self.0, other.0) })
  }

  /// lanewise f64x2 minimum
  #[inline(always)]
  pub fn min(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_min_pd(self.0, other.0) })
  }
  
  /// Sets bits 0 and 1 based on the most significant bits of the two lanes.
  #[inline(always)]
  pub fn move_mask(self) -> i32 {
    unsafe { _mm_movemask_pd(self.0) }
  }

  /// Loads the `f64` into both lanes.
  #[inline(always)]
  pub fn set_all(f: f64) -> m128d {
    m128d(unsafe { _mm_set1_pd (f) })
  }
  
  /// Sets the doubles into a register, high then low.
  #[inline(always)]
  pub fn set(high: f64, low: f64) -> Self {
    m128d(unsafe { _mm_set_pd(high, low) })
  }
  
  /// Sets the doubles into a register, low then high.
  #[inline(always)]
  pub fn setr(high: f64, low: f64) -> Self {
    m128d(unsafe { _mm_setr_pd(high, low) })
  }
  
  /// Gives zero in both lanes.
  #[inline(always)]
  pub fn zeroed() -> Self {
    m128d(unsafe { _mm_setzero_pd() })
  }

  // TODO: _mm_shuffle_pd (macro? only two bits of input)
  
  /// lanewise f64x2 square root.
  #[inline(always)]
  pub fn sqrt(self) -> m128d {
    m128d(unsafe { _mm_sqrt_pd (self.0) })
  }

  /// Stores the lower `f64` into both slots of the array
  #[inline(always)]
  pub fn store_lower_all(self, arr: &mut Align16<[f64; 2]>) {
    // TODO: TEST THAT INDEX 0 GOES INTO LANE 0.
    let p = arr as *mut Align16<[f64; 2]> as *mut f64;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store1_pd(p, self.0) };
  }

  /// Store the lanes into the slots of the array. Lowest lane to lowest index,
  /// and so on.
  #[inline(always)]
  pub fn store(self, addr: &mut Align16<[f64; 2]>) {
    let p = addr as *mut Align16<[f64; 2]> as *mut f64;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store_pd(p, self.0) };
  }

  /// As [store](m128d::store), but makes a new array and returns it for you.
  #[inline(always)]
  pub fn to_array(self) -> [f64; 2] {
    let mut a = Align16([0.0f64; 2]);
    self.store(&mut a);
    a.0
  }

  /// Stores the higher `f64`
  #[inline(always)]
  pub fn store_high(self, addr: &mut f64) {
    unsafe { _mm_storeh_pd (addr, self.0) };
  }

  /// Stores the lower `f64`
  #[inline(always)]
  pub fn store_low(self, addr: &mut f64) {
    unsafe { _mm_storel_pd  (addr, self.0) };
  }

  /// Store the lanes into the slots of the array in reverse order. Highest lane
  /// to lowest index, and so on.
  #[inline(always)]
  pub fn storer(self, addr: &mut Align16<[f64; 2]>) {
    let p = addr as *mut Align16<[f64; 2]> as *mut f64;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storer_pd(p, self.0) };
  }

  /// Store the lanes into the slots of the array. Lowest lane to lowest index,
  /// and so on.
  #[inline(always)]
  pub fn storeu(self, addr: &mut [f64; 2]) {
    let p = addr as *mut [f64; 2] as *mut f64;
    unsafe { _mm_storeu_pd(p, self.0) };
  }

  /// Unpack and interleave the high lanes of `self` and `other`.
  ///
  /// ```txt
  /// self:   ab
  /// other:  AB
  /// ----------
  /// output: Aa
  /// ```
  #[inline(always)]
  pub fn unpack_high(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_unpackhi_pd(self.0, other.0) })
  }

  /// Unpack and interleave the low lanes of `self` and `other`.
  ///
  /// ```txt
  /// self:   ab
  /// other:  AB
  /// ----------
  /// output: Bb
  /// ```
  #[inline(always)]
  pub fn unpack_low(self, other: m128d) -> m128d {
    m128d(unsafe { _mm_unpacklo_pd(self.0, other.0) })
  }
}

impl core::ops::Add for m128d {
  type Output = Self;
  fn add(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_add_pd(self.0, rhs.0) })
  }
}

impl core::ops::AddAssign for m128d {
  fn add_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_add_pd(self.0, rhs.0) }
  }
}

impl core::ops::BitAnd for m128d {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_and_pd(self.0, rhs.0) })
  }
}

impl core::ops::BitAndAssign for m128d {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_pd(self.0, rhs.0) }
  }
}

impl core::ops::Div for m128d {
  type Output = Self;
  fn div(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_div_pd(self.0, rhs.0) })
  }
}

impl core::ops::DivAssign for m128d {
  fn div_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_div_pd(self.0, rhs.0) }
  }
}

impl core::ops::Mul for m128d {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_mul_pd(self.0, rhs.0) })
  }
}

impl core::ops::MulAssign for m128d {
  fn mul_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_mul_pd(self.0, rhs.0) }
  }
}

impl core::ops::BitOr for m128d {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_or_pd(self.0, rhs.0) })
  }
}

impl core::ops::BitOrAssign for m128d {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_or_pd(self.0, rhs.0) }
  }
}

impl core::ops::BitXor for m128d {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_xor_pd(self.0, rhs.0) })
  }
}

impl core::ops::BitXorAssign for m128d {
  fn bitxor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_xor_pd(self.0, rhs.0) }
  }
}

impl core::ops::Sub for m128d {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self {
    m128d(unsafe { _mm_sub_pd(self.0, rhs.0) })
  }
}

impl core::ops::SubAssign for m128d {
  fn sub_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_sub_pd(self.0, rhs.0) }
  }
}

/// # SSE2 Operations
impl m128i {
  /// lanewise i8x16 addition
  #[inline(always)]
  pub fn add_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_add_epi8(self.0, other.0) })
  }

  /// lanewise i8x16 addition with saturation
  #[inline(always)]
  pub fn saturating_add_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_adds_epi8(self.0, other.0) })
  }

  /// lanewise i8x16 equality comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_eq_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpeq_epi8(self.0, other.0) })
  }

  /// lanewise i8x16 greater than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_gt_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpgt_epi8(self.0, other.0) })
  }

  /// lanewise i8x16 less than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_lt_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmplt_epi8(self.0, other.0) })
  }

  /// Mask of the most significant bit of each `i8` lane.
  #[inline(always)]
  pub fn movemask_i8(self) -> i32 {
    unsafe { _mm_movemask_epi8(self.0) }
  }

  /// Sets the value as all `i8` lanes
  #[inline(always)]
  pub fn set_all_i8(val: i8) -> Self {
    m128i(unsafe { _mm_set1_epi8(val) })
  }

  /// Sets the `i8` values into the lanes, high to low.
  #[inline(always)]
  pub fn set_i8(
    e15: i8,
    e14: i8,
    e13: i8,
    e12: i8,
    e11: i8,
    e10: i8,
    e9: i8,
    e8: i8,
    e7: i8,
    e6: i8,
    e5: i8,
    e4: i8,
    e3: i8,
    e2: i8,
    e1: i8,
    e0: i8,
  ) -> Self {
    m128i(unsafe {
      _mm_set_epi8(
        e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
      )
    })
  }

  /// Sets the `i8` values into the lanes reversed, low to high.
  #[inline(always)]
  pub fn setr_i8(
    e15: i8,
    e14: i8,
    e13: i8,
    e12: i8,
    e11: i8,
    e10: i8,
    e9: i8,
    e8: i8,
    e7: i8,
    e6: i8,
    e5: i8,
    e4: i8,
    e3: i8,
    e2: i8,
    e1: i8,
    e0: i8,
  ) -> Self {
    m128i(unsafe {
      _mm_setr_epi8(
        e15, e14, e13, e12, e11, e10, e9, e8, e7, e6, e5, e4, e3, e2, e1, e0,
      )
    })
  }

  /// lanewise i8x16 subtraction
  #[inline(always)]
  pub fn sub_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sub_epi8(self.0, other.0) })
  }

  /// lanewise i8x16 subtraction with saturation
  #[inline(always)]
  pub fn saturating_sub_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_subs_epi8(self.0, other.0) })
  }

  /// Unpack and interleave the high `i8` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcdefghijklmnop
  /// other:  ABCDEFGHIJKLMNOP
  /// ------------------------
  /// output: AaBbCcDdEeFfGgHh
  /// ```
  #[inline(always)]
  pub fn unpack_high_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpackhi_epi8(self.0, other.0) })
  }

  /// Unpack and interleave the low `i8` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcdefghijklmnop
  /// other:  ABCDEFGHIJKLMNOP
  /// ------------------------
  /// output: IiJjKkLlMmNnOoPp
  /// ```
  #[inline(always)]
  pub fn unpack_low_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpacklo_epi8(self.0, other.0) })
  }

  /// lanewise u8x16 addition with saturation
  #[inline(always)]
  pub fn saturating_add_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_adds_epu8(self.0, other.0) })
  }

  /// lanewise u8x16 average between this and other, `(a+b+1) >> 1`
  #[inline(always)]
  pub fn average_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_avg_epu8(self.0, other.0) })
  }

  /// lanewise u8x16 maximum value
  #[inline(always)]
  pub fn max_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_max_epu8(self.0, other.0) })
  }

  /// lanewise u8x16 minimum value
  #[inline(always)]
  pub fn min_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_min_epu8(self.0, other.0) })
  }

  /// lanewise u8x16 absolute difference, then 8-lane horizontal sum, then those
  /// sums (u16 values) are placed into the two `i64` lanes (other bits 0).
  #[inline(always)]
  pub fn sad_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sad_epu8(self.0, other.0) })
  }

  /// lanewise u8x16 subtraction with saturation
  #[inline(always)]
  pub fn saturating_sub_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_subs_epu8(self.0, other.0) })
  }

  /// lanewise i16x8 addition
  #[inline(always)]
  pub fn add_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_add_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 addition with saturation
  #[inline(always)]
  pub fn saturating_add_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_adds_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 equality comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_eq_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpeq_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 greater than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_gt_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpgt_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 less than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_lt_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmplt_epi16(self.0, other.0) })
  }

  /// Extracts the `i16` lane with the `index` given (`0..8`).
  #[inline(always)]
  pub fn extract_i16(self, index: usize) -> i32 {
    match index {
      0 => unsafe { _mm_extract_epi16(self.0, 0) },
      1 => unsafe { _mm_extract_epi16(self.0, 1) },
      2 => unsafe { _mm_extract_epi16(self.0, 2) },
      3 => unsafe { _mm_extract_epi16(self.0, 3) },
      4 => unsafe { _mm_extract_epi16(self.0, 4) },
      5 => unsafe { _mm_extract_epi16(self.0, 5) },
      6 => unsafe { _mm_extract_epi16(self.0, 6) },
      7 => unsafe { _mm_extract_epi16(self.0, 7) },
      _ => panic!("extract_i16: index out of bounds: {}", index),
    }
  }

  /// Inserts the `i16` to the lane with the `index` given (`0..8`).
  #[inline(always)]
  pub fn insert_i16(self, val: i16, index: usize) -> m128i {
    match index {
      0 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 0) }),
      1 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 1) }),
      2 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 2) }),
      3 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 3) }),
      4 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 4) }),
      5 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 5) }),
      6 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 6) }),
      7 => m128i(unsafe { _mm_insert_epi16(self.0, i32::from(val), 7) }),
      _ => panic!("insert_i16: index out of bounds: {}", index),
    }
  }

  /// lanewise i16x8 multiplication, then hadd pairs of `i32` intermediates to
  /// form an i32x4 output
  #[inline(always)]
  pub fn mul_hadd_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_madd_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 maximum value
  #[inline(always)]
  pub fn max_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_max_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 minimum value
  #[inline(always)]
  pub fn min_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_min_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 multiply and keep the high half of the `i32` intermediate.
  #[inline(always)]
  pub fn mul_high_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_mulhi_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 multiply and keep the low half of the `i32` intermediate.
  ///
  /// This is basically a `wrapping_mul_i16`
  #[inline(always)]
  pub fn mul_low_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_mullo_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 pack each lane into a saturated `i8` value, `self` makes up
  /// the low lanes, and `other` makes the high lanes.
  #[inline(always)]
  pub fn saturating_pack_i16_i8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_mullo_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 pack each lane into a saturated `u8` value, `self` makes up
  /// the low lanes, and `other` makes the high lanes.
  #[inline(always)]
  pub fn saturating_pack_i16_u8(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_packs_epi16(self.0, other.0) })
  }

  /// Sets the value as all `i16` lanes
  #[inline(always)]
  pub fn set_all_i16(val: i16) -> Self {
    m128i(unsafe { _mm_set1_epi16(val) })
  }

  /// Sets the `i16` values into the lanes, high to low.
  #[inline(always)]
  pub fn set_i16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> Self {
    m128i(unsafe { _mm_set_epi16(e7, e6, e5, e4, e3, e2, e1, e0) })
  }

  /// Sets the `i16` values into the lanes reversed, low to high.
  #[inline(always)]
  pub fn setr_i16(e7: i16, e6: i16, e5: i16, e4: i16, e3: i16, e2: i16, e1: i16, e0: i16) -> Self {
    m128i(unsafe { _mm_setr_epi16(e7, e6, e5, e4, e3, e2, e1, e0) })
  }

  // TODO: _mm_shufflehi_epi16 (macro)

  // TODO: _mm_shufflelo_epi16 (macro)

  /// lanewise i16x8 shift left (0s shifted in). `other` provides the amount to
  /// shift each lane.
  #[inline(always)]
  pub fn shift_left_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sll_epi16(self.0, other.0) })
  }

  // TODO: _mm_slli_epi16 (macro)

  /// lanewise i16x8 shift right (sign bit shifted in). `other` provides the
  /// amount to shift each lane.
  #[inline(always)]
  pub fn shift_right_sign_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sra_epi16(self.0, other.0) })
  }

  // TODO: _mm_srai_epi16 (macro)

  /// lanewise i16x8 shift right (0s shifted in). `other` provides the
  /// amount to shift each lane.
  #[inline(always)]
  pub fn shift_right_zero_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_srl_epi16(self.0, other.0) })
  }

  // TODO: _mm_srli_epi16 (macro)

  /// lanewise i16x8 subtraction
  #[inline(always)]
  pub fn sub_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sub_epi16(self.0, other.0) })
  }

  /// lanewise i16x8 subtraction with saturation
  #[inline(always)]
  pub fn saturating_sub_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_subs_epi16(self.0, other.0) })
  }

  /// Unpack and interleave the high `i16` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcdefgh
  /// other:  ABCDEFGH
  /// ----------------
  /// output: AaBbCcDd
  /// ```
  #[inline(always)]
  pub fn unpack_high_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpackhi_epi16(self.0, other.0) })
  }

  /// Unpack and interleave the low `i16` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcdefgh
  /// other:  ABCDEFGH
  /// ----------------
  /// output: EeFfGgHh
  /// ```
  #[inline(always)]
  pub fn unpack_low_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpacklo_epi16(self.0, other.0) })
  }

  /// lanewise u16x8 addition with saturation
  #[inline(always)]
  pub fn saturating_add_u16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_adds_epu16(self.0, other.0) })
  }

  /// lanewise u16x8 average between this and other, `(a+b+1) >> 1`
  #[inline(always)]
  pub fn average_u16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_avg_epu16(self.0, other.0) })
  }

  /// lanewise u16x8 multiplication, keeping the high bits.
  #[inline(always)]
  pub fn mul_high_u16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_mulhi_epu16(self.0, other.0) })
  }

  /// lanewise u16x8 subtraction with saturation
  #[inline(always)]
  pub fn saturating_sub_u16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_subs_epu16(self.0, other.0) })
  }

  /// lanewise u64x2 multiply of the low 32-bits, store as u64x2
  #[inline(always)]
  pub fn mul_low32_u64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_mul_epu32(self.0, other.0) })
  }

  /// lanewise i32x4 addition
  #[inline(always)]
  pub fn add_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_add_epi32(self.0, other.0) })
  }

  /// lanewise i32x4 equality comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_eq_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpeq_epi32(self.0, other.0) })
  }

  /// lanewise i32x4 greater than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_gt_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmpgt_epi32(self.0, other.0) })
  }

  /// lanewise i32x4 less than comparison, `true`==-1, `false`==0
  #[inline(always)]
  pub fn cmp_lt_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_cmplt_epi32(self.0, other.0) })
  }

  /// lanewise i32x4 pack each lane into a saturated `i16` value, `self` makes up
  /// the low lanes, and `other` makes the high lanes.
  #[inline(always)]
  pub fn saturating_pack_i32_i16(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_packs_epi32(self.0, other.0) })
  }

  /// Sets the `i32` value to all `i32` lanes.
  #[inline(always)]
  pub fn set_all_i32(val: i32) -> Self {
    m128i(unsafe { _mm_set1_epi32(val) })
  }

  /// Sets the `i32` values in a register, high to low.
  #[inline(always)]
  pub fn set_i32(e3: i32, e2: i32, e1: i32, e0: i32) -> Self {
    m128i(unsafe { _mm_set_epi32(e3, e2, e1, e0) })
  }

  // TODO: _mm_shuffle_epi32 (macro)

  /// lanewise i32x4 shift left (0s shifted in). `other` provides the amount to
  /// shift each lane.
  #[inline(always)]
  pub fn shift_left_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sll_epi32(self.0, other.0) })
  }

  // TODO: _mm_slli_epi32 (macro)

  /// lanewise i32x4 shift right (sign bit shifted in). `other` provides the
  /// amount to shift each lane.
  #[inline(always)]
  pub fn shift_right_sign_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sra_epi32(self.0, other.0) })
  }

  // TODO: _mm_srai_epi32 (macro)

  /// lanewise i32x4 shift right (0s shifted in). `other` provides the
  /// amount to shift each lane.
  #[inline(always)]
  pub fn shift_right_zero_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_srl_epi32(self.0, other.0) })
  }

  // TODO: _mm_srli_epi32 (macro)

  /// lanewise i32x4 subtraction
  #[inline(always)]
  pub fn sub_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sub_epi32(self.0, other.0) })
  }

  /// Unpack and interleave the high `i32` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcd
  /// other:  ABCD
  /// ------------
  /// output: AaBb
  /// ```
  #[inline(always)]
  pub fn unpack_high_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpackhi_epi32(self.0, other.0) })
  }

  /// Unpack and interleave the low `i32` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   abcd
  /// other:  ABCD
  /// ------------
  /// output: CcDd
  /// ```
  #[inline(always)]
  pub fn unpack_low_i32(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpacklo_epi32(self.0, other.0) })
  }

  /// Gets out the lowest `i32` lane.
  #[inline(always)]
  pub fn extract_lowest_i32(self) -> i32 {
    unsafe { _mm_cvtsi128_si32(self.0) }
  }

  /// lanewise i64x2 addition
  #[inline(always)]
  pub fn add_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_add_epi64(self.0, other.0) })
  }

  /// Load the low `i64` value into a new register.
  #[inline(always)]
  pub fn load_low_i64(addr: &m128i) -> m128i {
    m128i(unsafe { _mm_loadl_epi64(&addr.0) })
  }

  /// Move the low `i64` value into a new register.
  #[inline(always)]
  pub fn move_low_i64(self) -> m128i {
    m128i(unsafe { _mm_move_epi64(self.0) })
  }

  /// lanewise i64x2 shift left (0s shifted in). `other` provides the amount to
  /// shift each lane.
  #[inline(always)]
  pub fn shift_left_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sll_epi64(self.0, other.0) })
  }

  // TODO: _mm_slli_epi64 (macro)

  /// lanewise i64x2 shift right (0s shifted in). `other` provides the
  /// amount to shift each lane.
  #[inline(always)]
  pub fn shift_right_zero_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_srl_epi64(self.0, other.0) })
  }

  // TODO: _mm_srli_epi64 (macro)

  /// Store the low `i64` lane to the address provided.
  #[inline(always)]
  pub fn store_low_i64(self, addr: &mut m128i) {
    unsafe { _mm_storel_epi64(&mut addr.0, self.0) }
  }

  /// lanewise i64x2 subtraction
  #[inline(always)]
  pub fn sub_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_sub_epi64(self.0, other.0) })
  }

  /// Unpack and interleave the high `i64` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   ab
  /// other:  AB
  /// ----------
  /// output: Aa
  /// ```
  #[inline(always)]
  pub fn unpack_high_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpackhi_epi64(self.0, other.0) })
  }

  /// Unpack and interleave the low `i64` values of `self` and `other`.
  ///
  /// ```txt
  /// self:   ab
  /// other:  AB
  /// ----------
  /// output: Bb
  /// ```
  #[inline(always)]
  pub fn unpack_low_i64(self, other: m128i) -> m128i {
    m128i(unsafe { _mm_unpacklo_epi64(self.0, other.0) })
  }

  /// Extract the low `i64` lane
  #[cfg(target_arch = "x86_64")]
  #[inline(always)]
  pub fn extract_lowest_i64(self) -> i64 {
    unsafe { _mm_cvtsi128_si64(self.0) }
  }

  /// Sets the `i64` value to both `i64` lanes.
  #[inline(always)]
  pub fn set_all_i64(val: i64) -> Self {
    m128i(unsafe { _mm_set1_epi64x(val) })
  }

  /// Sets the `i64` values in a register, high then low.
  #[inline(always)]
  pub fn set_i64(high: i64, low: i64) -> Self {
    m128i(unsafe { _mm_set_epi64x(high, low) })
  }

  /// bitwise `!self & other`.
  #[inline(always)]
  pub fn andnot(self, other: m128i) -> Self {
    m128i(unsafe { _mm_andnot_si128(self.0, other.0) })
  }

  // TODO: _mm_bslli_si128 (immediate)

  // TODO: _mm_bsrli_si128 (immediate)

  /// Sets the `i32` as the lowest lane, other lanes 0
  #[inline(always)]
  pub fn set_single_i32(val: i32) -> Self {
    m128i(unsafe { _mm_cvtsi32_si128(val) })
  }

  /// Sets the `i64` as the lowest lane, other lanes 0
  #[inline(always)]
  pub fn set_single_i64(val: i64) -> Self {
    m128i(unsafe { _mm_cvtsi64_si128(val) })
  }

  /// Loads the integer data from the address given.
  #[inline(always)]
  pub fn load(addr: &Align16<i128>) -> Self {
    let p = addr as *const Align16<i128> as *const __m128i;
    debug_assert!(p as usize % 16 == 0);
    m128i(unsafe { _mm_load_si128(p) })
  }

  /// Loads the integer data from the address given.
  #[inline(always)]
  pub fn loadu(addr: &i128) -> Self {
    let p = addr as *const i128 as *const __m128i;
    debug_assert!(p as usize % 16 == 0);
    m128i(unsafe { _mm_loadu_si128(p) })
  }

  /// Creates a zeroed value.
  #[inline(always)]
  pub fn zeroed() -> Self {
    m128i(unsafe { _mm_setzero_si128() })
  }

  // TODO: _mm_slli_si128 (immediate)

  // TODO: _mm_srli_si128 (immediate)

  /// Store the data as a single `u128`, and you can re-interpret that however
  /// you like.
  #[inline(always)]
  pub fn store(self, addr: &mut Align16<i128>) {
    let p = addr as *mut Align16<i128> as *mut __m128i;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store_si128(p, self.0) };
  }

  /// Store the data as a single `i128`, and you can re-interpret that however
  /// you like.
  #[inline(always)]
  pub fn storeu(self, addr: &mut i128) {
    let p = addr as *mut i128 as *mut __m128i;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_storeu_si128(p, self.0) };
  }

  /// As [store](m128i::store), but returns a new `i128` for you.
  #[inline(always)]
  pub fn to_i128(self) -> i128 {
    let mut u = Align16(0i128);
    self.store(&mut u);
    u.0
  }

  /// Convert each `i32` lane into an `f32` lane.
  #[inline(always)]
  pub fn convert_f32s(self) -> m128 {
    m128(unsafe { _mm_cvtepi32_ps(self.0) })
  }

  /// Cast directly to `m128`
  #[inline(always)]
  pub fn cast_m128(self) -> m128 {
    m128(unsafe { _mm_castsi128_ps(self.0) })
  }

  /// Casts this value into an `m128d`
  #[inline(always)]
  pub fn cast_m128d(self) -> m128d {
    m128d(unsafe { _mm_castsi128_pd(self.0) })
  }

  /// Convert the lower `i32` lanes to `f64` values.
  #[inline(always)]
  pub fn convert_low_i32_f64(self) -> m128d {
    m128d(unsafe { _mm_cvtepi32_pd(self.0) })
  }
}

impl core::ops::BitAnd for m128i {
  type Output = Self;
  fn bitand(self, rhs: Self) -> Self {
    m128i(unsafe { _mm_and_si128(self.0, rhs.0) })
  }
}

impl core::ops::BitAndAssign for m128i {
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_si128(self.0, rhs.0) }
  }
}

impl core::ops::BitOr for m128i {
  type Output = Self;
  fn bitor(self, rhs: Self) -> Self {
    m128i(unsafe { _mm_or_si128(self.0, rhs.0) })
  }
}

impl core::ops::BitOrAssign for m128i {
  fn bitor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_or_si128(self.0, rhs.0) }
  }
}

impl core::ops::BitXor for m128i {
  type Output = Self;
  fn bitxor(self, rhs: Self) -> Self {
    m128i(unsafe { _mm_xor_si128(self.0, rhs.0) })
  }
}

impl core::ops::BitXorAssign for m128i {
  fn bitxor_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_xor_si128(self.0, rhs.0) }
  }
}

/// Serializes all load-from-memory and store-to-memory operations.
#[inline(always)]
pub fn memory_fence() {
  unsafe { _mm_mfence() }
}

/// Serializes all load-from-memory operations.
#[inline(always)]
pub fn load_fence() {
  unsafe { _mm_lfence() }
}

/// Invalidate and flush the cache line that contains the pointer from all
/// levels of the cache hierarchy.
#[inline(always)]
pub fn cache_line_flush(p: *const impl Sized) {
  unsafe { _mm_clflush(p as *mut u8) }
}
