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
}

/// # Operations
impl m128 {
  /// f32x4 lane-wise addition
  pub fn add(self, other: m128) -> m128 {
    m128(unsafe { _mm_add_ps(self.0, other.0) })
  }
}
