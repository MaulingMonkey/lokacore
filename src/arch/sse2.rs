#![allow(clippy::too_many_arguments)]

use super::*;

/// # debug functionality (TODO: sort this to where they go)
impl m128d {
  /// Sets the doubles into a register, high then low.
  #[inline(always)]
  pub fn set(high: f64, low: f64) -> Self {
    m128d(unsafe { _mm_set_pd(high, low) })
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
}

/// # debug functionality (TODO: sort this to where they go)
impl m128i {
  /// Store the data as a single `u128`, and you can re-interpret that however
  /// you like.
  #[inline(always)]
  pub fn store(self, addr: &mut Align16<u128>) {
    let p = addr as *mut Align16<u128> as *mut __m128i;
    debug_assert!(p as usize % 16 == 0);
    unsafe { _mm_store_si128(p, self.0) };
  }

  /// As [store](m128i::store), but returns a new `u128` for you.
  #[inline(always)]
  pub fn to_u128(self) -> u128 {
    let mut u = Align16(0u128);
    self.store(&mut u);
    u.0
  }
}

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

// // // // // // // // // // // // // // // // // // // // //

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
}

/// Store the value using a non-temporal hint.
#[inline(always)]
pub fn store_nontemporal_i32(addr: &mut i32, val: i32) {
  unsafe { _mm_stream_si32(addr, val) };
}

/// Store the value using a non-temporal hint.
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn store_nontemporal_i64(addr: &mut i64, val: i64) {
  unsafe { _mm_stream_si64(addr, val) };
}
