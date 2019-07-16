#![cfg(target_feature="sse2")]

use super::*;
use core::ops::*;

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
