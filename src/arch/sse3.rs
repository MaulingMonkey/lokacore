use super::*;

/// # SSE3 Operations
impl m128 {
  //
}

/// # SSE3 Operations
impl m128d {
  /// Adds the low lanes and subtracts the high lanes.
  #[inline(always)]
  pub fn add_sub(self, other: m128d) -> Self {
    Self(unsafe { _mm_addsub_pd(self.0, other.0) })
  }
}

/// # SSE3 Operations
impl m128i {
  //
}
