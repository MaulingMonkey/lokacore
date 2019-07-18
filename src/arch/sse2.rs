#![cfg(target_feature="sse2")]

use super::*;
use core::ops::*;

/// # SSE2 Operations
impl m128 {
  /// This rounds each lane to `i32`.
  #[inline(always)]
  pub fn round_i32x4(self) -> m128i {
    m128i(unsafe { _mm_cvtps_epi32(self.0) })
  }

  /// This truncates each lane to `i32`.
  #[inline(always)]
  pub fn truncate_i32x4(self) -> m128i {
    m128i(unsafe { _mm_cvttps_epi32(self.0) })
  }

  /// This "rounds" the lower two lanes to `f64`.
  ///
  /// `f64` has more precision than `f32` so there's no actually rounding going
  /// on here, but I'll just call it rounding so that the naming stays
  /// consistent with other similar methods.
  #[inline(always)]
  pub fn round_f64x2(self) -> m128d {
    m128d(unsafe { _mm_cvtps_pd(self.0) })
  }

  /// Lane 0 is the low `f64` of `rhs` rounded to `f32`, other lanes are `self`.
  #[inline(always)]
  pub fn f64_round_copy0(self, rhs: m128d) -> Self {
    Self(unsafe { _mm_cvtsd_ss(self.0, rhs.0) })
  }

  /// Cast the bits of this `m128` directly to `m128i` without modification.
  #[inline(always)]
  pub fn cast_m128i(self) -> m128i {
    m128i(unsafe { _mm_castps_si128(self.0) })
  }
}

/// A 128-bit SIMD value. Integral data, lanes determined by each op.
///
/// * This documentation numbers the lanes based on the index you'd need to use
///   to access that lane if the value were cast to an array.
/// * This is also the way that the type is printed out using
///   [`Debug`](core::fmt::Debug), [`Display`](core::fmt::Display),
///   [`LowerExp`](core::fmt::LowerExp), and [`UpperExp`](core::fmt::UpperExp).
/// * This is not necessarily the ordering you'll see if you look an `xmm`
///   register in a debugger! Basically because of how little-endian works.
/// * Most operations work per-lane, "lanewise".
/// * Some operations work using lane 0 only. When appropriate, these have the
///   same name as the lanewise version but with a `0` on the end. Eg: `cmp_eq`
///   and `cmp_eq0`. The other lanes are simply copied forward from `self`.
/// * Comparisons give "bool-ish" output, where all bits 1 in a lane is true,
///   and all bits 0 in a lane is false. Unfortunately, all bits 1 with an `f32`
///   is one of the `NaN` values, and `NaN != NaN`, so it can be a little tricky
///   to work with until you're used to it.
#[derive(Clone, Copy)]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128i(pub __m128i);

unsafe impl Zeroable for m128i {}
unsafe impl Pod for m128i {}

impl core::fmt::Debug for m128i {
  /// Debug formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give `uX` instead of `iX` output.
  ///
  /// Eg, for 4 lanes of `u32`:
  /// ```txt
  /// format!("{:#4?}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        if f.alternate() {
          let a: [u64; 2] = cast(self.0);
          write!(f, "m128i({:?}, {:?})", a[0], a[1])
        } else {
          let a: [i64; 2] = cast(self.0);
          write!(f, "m128i({:?}, {:?})", a[0], a[1])
        }
      }
      Some(4) => {
        if f.alternate() {
          let a: [u32; 4] = cast(self.0);
          write!(f, "m128i({:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3])
        } else {
          let a: [i32; 4] = cast(self.0);
          write!(f, "m128i({:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        if f.alternate() {
          let a: [u16; 8] = cast(self.0);
          write!(
            f,
            "m128i({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          let a: [i16; 8] = cast(self.0);
          write!(
            f,
            "m128i({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        if f.alternate() {
          let a: [u8; 16] = cast(self.0);
          write!(f, "m128i({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        } else {
          let a: [i8; 16] = cast(self.0);
          write!(f, "m128i({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        }
      }
      _ => {
        if f.alternate() {
          let a: u128 = cast(self.0);
          write!(f, "m128i({:?})", a)
        } else {
          let a: i128 = cast(self.0);
          write!(f, "m128i({:?})", a)
        }
      }
    }
  }
}

impl core::fmt::Display for m128i {
  /// Display formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give `uX` instead of `iX` output.
  ///
  /// Eg, for 4 lanes of `u32`:
  /// ```txt
  /// format!("{:#4?}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        if f.alternate() {
          let a: [u64; 2] = cast(self.0);
          write!(f, "m128i({}, {})", a[0], a[1])
        } else {
          let a: [i64; 2] = cast(self.0);
          write!(f, "m128i({}, {})", a[0], a[1])
        }
      }
      Some(4) => {
        if f.alternate() {
          let a: [u32; 4] = cast(self.0);
          write!(f, "m128i({}, {}, {}, {})", a[0], a[1], a[2], a[3])
        } else {
          let a: [i32; 4] = cast(self.0);
          write!(f, "m128i({}, {}, {}, {})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        if f.alternate() {
          let a: [u16; 8] = cast(self.0);
          write!(
            f,
            "m128i({}, {}, {}, {}, {}, {}, {}, {})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          let a: [i16; 8] = cast(self.0);
          write!(
            f,
            "m128i({}, {}, {}, {}, {}, {}, {}, {})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        if f.alternate() {
          let a: [u8; 16] = cast(self.0);
          write!(
            f,
            "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
            a[6],
            a[7],
            a[8],
            a[9],
            a[10],
            a[11],
            a[12],
            a[13],
            a[14],
            a[15]
          )
        } else {
          let a: [i8; 16] = cast(self.0);
          write!(
            f,
            "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            a[0],
            a[1],
            a[2],
            a[3],
            a[4],
            a[5],
            a[6],
            a[7],
            a[8],
            a[9],
            a[10],
            a[11],
            a[12],
            a[13],
            a[14],
            a[15]
          )
        }
      }
      _ => {
        if f.alternate() {
          let a: u128 = cast(self.0);
          write!(f, "m128i({})", a)
        } else {
          let a: i128 = cast(self.0);
          write!(f, "m128i({})", a)
        }
      }
    }
  }
}

impl core::fmt::Binary for m128i {
  /// Binary formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give leading `0b`.
  ///
  /// Eg, for 4 lanes and leading `0b`:
  /// ```txt
  /// format!("{:#4b}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        let a: [u64; 2] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#b}, {:#b})", a[0], a[1])
        } else {
          write!(f, "m128i({:b}, {:b})", a[0], a[1])
        }
      }
      Some(4) => {
        let a: [u32; 4] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#b}, {:#b}, {:#b}, {:#b})",
            a[0], a[1], a[2], a[3]
          )
        } else {
          write!(f, "m128i({:b}, {:b}, {:b}, {:b})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        let a: [u16; 8] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          write!(
            f,
            "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        let a: [u8; 16] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        } else {
          write!(f, "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        }
      }
      _ => {
        let a: u128 = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#b})", a)
        } else {
          write!(f, "m128i({:b})", a)
        }
      }
    }
  }
}

impl core::fmt::LowerHex for m128i {
  /// LowerHex formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give leading `0x`.
  ///
  /// Eg, for 4 lanes and leading `0x`:
  /// ```txt
  /// format!("{:#4x}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        let a: [u64; 2] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#x}, {:#x})", a[0], a[1])
        } else {
          write!(f, "m128i({:x}, {:x})", a[0], a[1])
        }
      }
      Some(4) => {
        let a: [u32; 4] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#x}, {:#x}, {:#x}, {:#x})",
            a[0], a[1], a[2], a[3]
          )
        } else {
          write!(f, "m128i({:x}, {:x}, {:x}, {:x})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        let a: [u16; 8] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          write!(
            f,
            "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        let a: [u8; 16] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        } else {
          write!(f, "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        }
      }
      _ => {
        let a: u128 = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#x})", a)
        } else {
          write!(f, "m128i({:x})", a)
        }
      }
    }
  }
}

impl core::fmt::Octal for m128i {
  /// Octal formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give leading `0o`.
  ///
  /// Eg, for 4 lanes and leading `0o`:
  /// ```txt
  /// format!("{:#4o}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        let a: [u64; 2] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#o}, {:#o})", a[0], a[1])
        } else {
          write!(f, "m128i({:o}, {:o})", a[0], a[1])
        }
      }
      Some(4) => {
        let a: [u32; 4] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#o}, {:#o}, {:#o}, {:#o})",
            a[0], a[1], a[2], a[3]
          )
        } else {
          write!(f, "m128i({:o}, {:o}, {:o}, {:o})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        let a: [u16; 8] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          write!(
            f,
            "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        let a: [u8; 16] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        } else {
          write!(f, "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        }
      }
      _ => {
        let a: u128 = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#o})", a)
        } else {
          write!(f, "m128i({:o})", a)
        }
      }
    }
  }
}

impl core::fmt::UpperHex for m128i {
  /// UpperHex formats in offset order.
  ///
  /// * Use `width` to specify the lane count you want (default 1).
  /// * Use `alternate` format specifier to give leading `0x`.
  ///
  /// Eg, for 4 lanes and leading `0x`:
  /// ```txt
  /// format!("{:#4X}", m)
  /// ```
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    match f.width() {
      Some(2) => {
        let a: [u64; 2] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#X}, {:#X})", a[0], a[1])
        } else {
          write!(f, "m128i({:X}, {:X})", a[0], a[1])
        }
      }
      Some(4) => {
        let a: [u32; 4] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#X}, {:#X}, {:#X}, {:#X})",
            a[0], a[1], a[2], a[3]
          )
        } else {
          write!(f, "m128i({:X}, {:X}, {:X}, {:X})", a[0], a[1], a[2], a[3])
        }
      }
      Some(8) => {
        let a: [u16; 8] = cast(self.0);
        if f.alternate() {
          write!(
            f,
            "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        } else {
          write!(
            f,
            "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})",
            a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]
          )
        }
      }
      Some(16) => {
        let a: [u8; 16] = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        } else {
          write!(f, "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})", a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15])
        }
      }
      _ => {
        let a: u128 = cast(self.0);
        if f.alternate() {
          write!(f, "m128i({:#X})", a)
        } else {
          write!(f, "m128i({:X})", a)
        }
      }
    }
  }
}

impl BitAnd for m128i {
  type Output = Self;
  /// Bitwise AND.
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    Self(unsafe { _mm_and_si128(self.0, rhs.0) })
  }
}
impl BitAndAssign for m128i {
  /// Bitwise AND.
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_si128(self.0, rhs.0) };
  }
}

/// # SSE2 Operations
impl m128d {
  //
}

/// A 128-bit SIMD value. Always used as `f64x2`.
///
/// * This documentation numbers the lanes based on the index you'd need to use
///   to access that lane if the value were cast to an array.
/// * This is also the way that the type is printed out using
///   [`Debug`](core::fmt::Debug), [`Display`](core::fmt::Display),
///   [`LowerExp`](core::fmt::LowerExp), and [`UpperExp`](core::fmt::UpperExp).
/// * This is not necessarily the ordering you'll see if you look an `xmm`
///   register in a debugger! Basically because of how little-endian works.
/// * Most operations work per-lane, "lanewise".
/// * Some operations work using lane 0 only. When appropriate, these have the
///   same name as the lanewise version but with a `0` on the end. Eg: `cmp_eq`
///   and `cmp_eq0`. The other lanes are simply copied forward from `self`.
/// * Comparisons give "bool-ish" output, where all bits 1 in a lane is true,
///   and all bits 0 in a lane is false. Unfortunately, all bits 1 with an `f32`
///   is one of the `NaN` values, and `NaN != NaN`, so it can be a little tricky
///   to work with until you're used to it.
#[derive(Clone, Copy)]
#[allow(bad_style)]
#[repr(transparent)]
pub struct m128d(pub __m128d);

unsafe impl Zeroable for m128d {}
unsafe impl Pod for m128d {}

impl core::fmt::Debug for m128d {
  /// Debug formats in offset order.
  ///
  /// All `Formatter` information is passed directly to each individual `f64`
  /// lane being formatted.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f64; 2] = cast(self.0);
    f.write_str("m128d(")?;
    core::fmt::Debug::fmt(&a[0], f)?;
    f.write_str(", ")?;
    core::fmt::Debug::fmt(&a[1], f)?;
    f.write_str(")")
  }
}

impl core::fmt::Display for m128d {
  /// Display formats in offset order.
  ///
  /// All `Formatter` information is passed directly to each individual `f64`
  /// lane being formatted.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f64; 2] = cast(self.0);
    f.write_str("m128d(")?;
    core::fmt::Display::fmt(&a[0], f)?;
    f.write_str(", ")?;
    core::fmt::Display::fmt(&a[1], f)?;
    f.write_str(")")
  }
}

impl core::fmt::LowerExp for m128d {
  /// LowerExp formats in offset order.
  ///
  /// All `Formatter` information is passed directly to each individual `f64`
  /// lane being formatted.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f64; 2] = cast(self.0);
    f.write_str("m128d(")?;
    core::fmt::LowerExp::fmt(&a[0], f)?;
    f.write_str(", ")?;
    core::fmt::LowerExp::fmt(&a[1], f)?;
    f.write_str(")")
  }
}

impl core::fmt::UpperExp for m128d {
  /// UpperExp formats in offset order.
  ///
  /// All `Formatter` information is passed directly to each individual `f64`
  /// lane being formatted.
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let a: [f64; 2] = cast(self.0);
    f.write_str("m128d(")?;
    core::fmt::UpperExp::fmt(&a[0], f)?;
    f.write_str(", ")?;
    core::fmt::UpperExp::fmt(&a[1], f)?;
    f.write_str(")")
  }
}

impl Add for m128d {
  type Output = Self;
  /// Lanewise addition.
  #[inline(always)]
  fn add(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_pd(self.0, rhs.0) })
  }
}
impl AddAssign for m128d {
  /// Lanewise addition.
  #[inline(always)]
  fn add_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_add_pd(self.0, rhs.0) };
  }
}

impl BitAnd for m128d {
  type Output = Self;
  /// Bitwise AND.
  #[inline(always)]
  fn bitand(self, rhs: Self) -> Self {
    Self(unsafe { _mm_and_pd(self.0, rhs.0) })
  }
}
impl BitAndAssign for m128d {
  /// Bitwise AND.
  #[inline(always)]
  fn bitand_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_and_pd(self.0, rhs.0) };
  }
}

impl Div for m128d {
  type Output = Self;
  /// Lanewise division.
  #[inline(always)]
  fn div(self, rhs: Self) -> Self {
    Self(unsafe { _mm_div_pd(self.0, rhs.0) })
  }
}
impl DivAssign for m128d {
  /// Lanewise division.
  #[inline(always)]
  fn div_assign(&mut self, rhs: Self) {
    self.0 = unsafe { _mm_div_pd(self.0, rhs.0) };
  }
}

/// # SSE2 Operations
impl m128d {
  /// Adds the low lane, high lane unaffected.
  #[inline(always)]
  pub fn add0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_add_sd(self.0, rhs.0) })
  }

  /// Bitwise `(!self) & rhs`
  #[inline(always)]
  pub fn andnot(self, rhs: Self) -> Self {
    Self(unsafe { _mm_andnot_pd(self.0, rhs.0) })
  }

  /// Cast the bits of this `m128d` directly to `m128i` without modification.
  #[inline(always)]
  pub fn cast_m128i(self) -> m128i {
    m128i(unsafe { _mm_castpd_si128(self.0) })
  }

  /// Lanewise `self == rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_eq(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpeq_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self == rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_eq0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpeq_sd(self.0, rhs.0) })
  }

  /// Lanewise `self >= rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_ge(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpge_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self >= rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_ge0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpge_sd(self.0, rhs.0) })
  }

  /// Lanewise `self > rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_gt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpgt_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self > rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_gt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpgt_sd(self.0, rhs.0) })
  }

  /// Lanewise `self <= rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_le(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmple_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self <= rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_le0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmple_sd(self.0, rhs.0) })
  }

  /// Lanewise `self < rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_lt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmplt_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self < rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_lt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmplt_sd(self.0, rhs.0) })
  }

  /// Lanewise `self != rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_ne(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpneq_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self != rhs`, bool-ish output
  #[inline(always)]
  pub fn cmp_ne0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpneq_sd(self.0, rhs.0) })
  }

  /// Lanewise `!(self >= rhs)`, bool-ish output
  /// 
  /// Also, 3rd Impact and all that, of course.
  #[inline(always)]
  pub fn cmp_nge(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnge_pd(self.0, rhs.0) })
  }

  /// Lane 0: `!(self >= rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_nge0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnge_sd(self.0, rhs.0) })
  }

  /// Lanewise `!(self > rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_ngt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpngt_pd(self.0, rhs.0) })
  }

  /// Lane 0: `!(self > rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_ngt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpngt_sd(self.0, rhs.0) })
  }

  /// Lanewise `!(self <= rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_nle(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnle_pd(self.0, rhs.0) })
  }

  /// Lane 0: `!(self <= rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_nle0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnle_sd(self.0, rhs.0) })
  }

  /// Lanewise `!(self < rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_nlt(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_pd(self.0, rhs.0) })
  }

  /// Lane 0: `!(self < rhs)`, bool-ish output
  #[inline(always)]
  pub fn cmp_nlt0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpnlt_sd(self.0, rhs.0) })
  }

  /// Lanewise `self.not_nan() & rhs.not_nan()`, bool-ish output
  #[inline(always)]
  pub fn cmp_ordinary(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpord_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self.not_nan() & rhs.not_nan()`, bool-ish output
  #[inline(always)]
  pub fn cmp_ordinary0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpord_sd(self.0, rhs.0) })
  }

  /// Lanewise `self.is_nan() | rhs.is_nan()`, bool-ish output
  #[inline(always)]
  pub fn cmp_nan(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpunord_pd(self.0, rhs.0) })
  }

  /// Lane 0: `self.is_nan() | rhs.is_nan()`, bool-ish output
  #[inline(always)]
  pub fn cmp_nan0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_cmpunord_sd(self.0, rhs.0) })
  }

  /// Lane 0: `self == rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_eq0(self, rhs: Self) -> i32 {
    unsafe { _mm_comieq_sd(self.0, rhs.0) }
  }

  /// Lane 0: `self >= rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_ge0(self, rhs: Self) -> i32 {
    unsafe { _mm_comige_sd(self.0, rhs.0) }
  }

  /// Lane 0: `self > rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_gt0(self, rhs: Self) -> i32 {
    unsafe { _mm_comigt_sd(self.0, rhs.0) }
  }

  /// Lane 0: `self <= rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_le0(self, rhs: Self) -> i32 {
    unsafe { _mm_comile_sd(self.0, rhs.0) }
  }

  /// Lane 0: `self < rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_lt0(self, rhs: Self) -> i32 {
    unsafe { _mm_comilt_sd(self.0, rhs.0) }
  }

  /// Lane 0: `self != rhs`, 0 or 1 `i32` output.
  #[inline(always)]
  pub fn cmpi_ne0(self, rhs: Self) -> i32 {
    unsafe { _mm_comineq_sd(self.0, rhs.0) }
  }

  /// Round the lanes to `i32` and place as the two lower lanes of an [`m128i`]
  #[inline(always)]
  pub fn round_i32x4(self) -> m128i {
    m128i(unsafe { _mm_cvtpd_epi32(self.0) })
  }

  /// Round the lanes to `f32` and place as the two lower lanes of an [`m128`]
  #[inline(always)]
  pub fn round_f32x4(self) -> m128 {
    m128(unsafe { _mm_cvtpd_ps(self.0) })
  }

  /// Get the lower lane value as `f64`.
  #[inline(always)]
  pub fn extract0(self) -> f64 {
    unsafe { _mm_cvtsd_f64(self.0) }
  }

  /// Round lower lane to `i32` and return it.
  #[inline(always)]
  pub fn round_i32_extract0(self) -> i32 {
    unsafe { _mm_cvtsd_si32(self.0) }
  }

  /// Round lower lane to `i64` and return it.
  #[cfg(target_arch="x86_64")]
  #[inline(always)]
  pub fn round_i64_extract0(self) -> i64 {
    unsafe { _mm_cvtsd_si64(self.0) }
  }

  /// Replace lane 0 with `i32` rounded to `f64`, lane 1 unaffected.
  #[inline(always)]
  pub fn replace0_with_i32(self, rhs: i32) -> Self {
    m128d(unsafe { _mm_cvtsi32_sd(self.0, rhs) })
  }

  /// Replace lane 0 with `i64` rounded to `f64`, lane 1 unaffected.
  #[inline(always)]
  pub fn replace0_with_i64(self, rhs: i64) -> Self {
    m128d(unsafe { _mm_cvtsi64_sd(self.0, rhs) })
  }

  /// Replace lane 0 with `rhs` low `f32` rounded to `f64`, lane 1 unaffected.
  #[inline(always)]
  pub fn replace0_with_f32(self, rhs: m128) -> Self {
    m128d(unsafe { _mm_cvtss_sd(self.0, rhs.0) })
  }

  /// Truncate the lanes to `i32` and place as the two lower lanes of an [`m128i`]
  #[inline(always)]
  pub fn truncate_i32x4(self) -> m128i {
    m128i(unsafe { _mm_cvttpd_epi32(self.0) })
  }

  /// Truncate lane 0 to `i32` and return it.
  #[inline(always)]
  pub fn truncate0_i32(self) -> i32 {
    unsafe { _mm_cvttsd_si32(self.0) }
  }

  /// Truncate lane 0 to `i64` and return it.
  #[cfg(target_arch="x86_64")]
  #[inline(always)]
  pub fn truncate0_i64(self) -> i64 {
    unsafe { _mm_cvttsd_si64(self.0) }
  }

  /// Divides the low lane, high lane unaffected.
  #[inline(always)]
  pub fn div0(self, rhs: Self) -> Self {
    Self(unsafe { _mm_div_sd(self.0, rhs.0) })
  }
}
