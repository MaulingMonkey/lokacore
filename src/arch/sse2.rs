use super::*;

// the following are x86_64 only:

// _mm_cvtsd_si64

// _mm_cvtsd_si64x

// _mm_cvtsi64_sd

// _mm_cvtsi64x_sd

// _mm_cvtsi128_si64

// _mm_cvtsi128_si64x

// _mm_cvtsi64_si128

// _mm_cvtsi64x_si128

// _mm_cvttsd_si64

// _mm_cvttsd_si64x

// _mm_stream_si64

/// # SSE Operations
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

/// # SSE Operations
impl m128i {
  /// Sets the `i32` values in a register, high to low.
  #[inline(always)]
  pub fn set_i32(e3: i32, e2: i32, e1: i32, e0: i32) -> Self {
    m128i(unsafe { _mm_set_epi32(e3, e2, e1, e0) })
  }

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
