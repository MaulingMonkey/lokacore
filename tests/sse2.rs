#![cfg(target_feature = "sse")]
#![allow(bad_style)]

#[path = "intel-simd-help.rs"]
mod intel_simd_help;
use intel_simd_help::*;

#[test]
fn m128i_debug() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:?}", m), "m128i(-1)");
  assert_eq!(&format!("{:#?}", m), &format!("m128i({})", max));
  assert_eq!(&format!("{:1?}", m), "m128i(-1)");
  assert_eq!(&format!("{:#1?}", m), &format!("m128i({})", max));
  let max = core::u64::MAX;
  assert_eq!(&format!("{:2?}", m), "m128i(-1, -1)");
  assert_eq!(&format!("{:#2?}", m), &format!("m128i({}, {})", max, max));
  let max = core::u32::MAX;
  assert_eq!(&format!("{:4?}", m), "m128i(-1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#4?}", m),
    &format!("m128i({}, {}, {}, {})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8?}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#8?}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16?}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#16?}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_display() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:}", m), "m128i(-1)");
  assert_eq!(&format!("{:#}", m), &format!("m128i({})", max));
  assert_eq!(&format!("{:1}", m), "m128i(-1)");
  assert_eq!(&format!("{:#1}", m), &format!("m128i({})", max));
  let max = core::u64::MAX;
  assert_eq!(&format!("{:2}", m), "m128i(-1, -1)");
  assert_eq!(&format!("{:#2}", m), &format!("m128i({}, {})", max, max));
  let max = core::u32::MAX;
  assert_eq!(&format!("{:4}", m), "m128i(-1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#4}", m),
    &format!("m128i({}, {}, {}, {})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(&format!("{:8}", m), "m128i(-1, -1, -1, -1, -1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#8}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#16}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_binary() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:b}", m), &format!("m128i({:b})", max));
  assert_eq!(&format!("{:#b}", m), &format!("m128i({:#b})", max));
  assert_eq!(&format!("{:1b}", m), &format!("m128i({:b})", max));
  assert_eq!(&format!("{:#1b}", m), &format!("m128i({:#b})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2b}", m),
    &format!("m128i({:b}, {:b})", max, max)
  );
  assert_eq!(
    &format!("{:#2b}", m),
    &format!("m128i({:#b}, {:#b})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4b}", m),
    &format!("m128i({:b}, {:b}, {:b}, {:b})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4b}", m),
    &format!("m128i({:#b}, {:#b}, {:#b}, {:#b})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8b}", m),
    &format!(
      "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8b}", m),
    &format!(
      "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16b}", m),
    &format!(
      "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16b}", m),
    &format!(
      "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_lower_hex() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:x}", m), &format!("m128i({:x})", max));
  assert_eq!(&format!("{:#x}", m), &format!("m128i({:#x})", max));
  assert_eq!(&format!("{:1x}", m), &format!("m128i({:x})", max));
  assert_eq!(&format!("{:#1x}", m), &format!("m128i({:#x})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2x}", m),
    &format!("m128i({:x}, {:x})", max, max)
  );
  assert_eq!(
    &format!("{:#2x}", m),
    &format!("m128i({:#x}, {:#x})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4x}", m),
    &format!("m128i({:x}, {:x}, {:x}, {:x})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4x}", m),
    &format!("m128i({:#x}, {:#x}, {:#x}, {:#x})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8x}", m),
    &format!(
      "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8x}", m),
    &format!(
      "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16x}", m),
    &format!(
      "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16x}", m),
    &format!(
      "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_octal() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:o}", m), &format!("m128i({:o})", max));
  assert_eq!(&format!("{:#o}", m), &format!("m128i({:#o})", max));
  assert_eq!(&format!("{:1o}", m), &format!("m128i({:o})", max));
  assert_eq!(&format!("{:#1o}", m), &format!("m128i({:#o})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2o}", m),
    &format!("m128i({:o}, {:o})", max, max)
  );
  assert_eq!(
    &format!("{:#2o}", m),
    &format!("m128i({:#o}, {:#o})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4o}", m),
    &format!("m128i({:o}, {:o}, {:o}, {:o})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4o}", m),
    &format!("m128i({:#o}, {:#o}, {:#o}, {:#o})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8o}", m),
    &format!(
      "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8o}", m),
    &format!(
      "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16o}", m),
    &format!(
      "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16o}", m),
    &format!(
      "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_upper_hex() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:X}", m), &format!("m128i({:X})", max));
  assert_eq!(&format!("{:#X}", m), &format!("m128i({:#X})", max));
  assert_eq!(&format!("{:1X}", m), &format!("m128i({:X})", max));
  assert_eq!(&format!("{:#1X}", m), &format!("m128i({:#X})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2X}", m),
    &format!("m128i({:X}, {:X})", max, max)
  );
  assert_eq!(
    &format!("{:#2X}", m),
    &format!("m128i({:#X}, {:#X})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4X}", m),
    &format!("m128i({:X}, {:X}, {:X}, {:X})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4X}", m),
    &format!("m128i({:#X}, {:#X}, {:#X}, {:#X})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8X}", m),
    &format!(
      "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8X}", m),
    &format!(
      "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16X}", m),
    &format!(
      "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16X}", m),
    &format!(
      "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128d_debug() {
  let m: m128d = cast([5.0_f64, 6.0]);
  assert_eq!(&format!("{:?}", m), "m128d(5.0, 6.0)");
}

#[test]
fn m128d_display() {
  let m: m128d = cast([5.0_f64, 6.0]);
  assert_eq!(&format!("{}", m), "m128d(5, 6)");
}

#[test]
fn m128d_lower_exp() {
  let m: m128d = cast([5.0_f64, 6.0]);
  assert_eq!(&format!("{:e}", m), "m128d(5e0, 6e0)");
}

#[test]
fn m128d_upper_exp() {
  let m: m128d = cast([5.0_f64, 6.0]);
  assert_eq!(&format!("{:E}", m), "m128d(5E0, 6E0)");
}

#[test]
fn m128_round_i32x4() {
  let m: m128 = cast([5.0_f32, 6.1, 7.9, 8.5]);
  let mi: m128i = m.round_i32x4();
  let mi_arr: [i32; 4] = cast(mi);
  assert_eq!(mi_arr, [5, 6, 8, 8]);
}

#[test]
fn m128_truncate_i32x4() {
  let m: m128 = cast([5.0_f32, 6.1, 7.9, 8.5]);
  let mi: m128i = m.truncate_i32x4();
  let mi_arr: [i32; 4] = cast(mi);
  assert_eq!(mi_arr, [5, 6, 7, 8]);
}

#[test]
fn m128_round_f64x2() {
  let m: m128 = cast([5.0_f32, 6.5, 7.9, 8.5]);
  let md: m128d = m.round_f64x2();
  let md_arr: [f64; 2] = cast(md);
  assert_eq!(md_arr, [5.0_f64, 6.5]);
}

#[test]
fn m128_bitand() {
  let max = core::u32::MAX;
  let a: m128i = cast([0, max, max, 0]);
  let b: m128i = cast([max, 0, max, 0]);
  let out: [u32; 4] = cast(a & b);
  assert_eq!(out, [0, 0, max, 0]);
}

#[test]
fn m128_bitand_assign() {
  let max = core::u32::MAX;
  let mut a: m128i = cast([0, max, max, 0]);
  let b: m128i = cast([max, 0, max, 0]);
  a &= b;
  let out: [u32; 4] = cast(a);
  assert_eq!(out, [0, 0, max, 0]);
}

#[test]
fn m128_cast_m128i() {
  let m: m128 = cast(12_345_678_u128);
  let mi: m128i = m.cast_m128i();
  let mi_bits: u128 = cast(mi);
  assert_eq!(mi_bits, 12_345_678_u128);
}

#[test]
fn m128d_add() {
  let a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([-8.0_f64, 4.0]);
  let out: [f64; 2] = cast(a + b);
  assert_eq!(out, [-3.0, 10.0]);
}

#[test]
fn m128d_add_assign() {
  let mut a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([-8.0_f64, 4.0]);
  a += b;
  let out: [f64; 2] = cast(a);
  assert_eq!(out, [-3.0, 10.0]);
}

#[test]
fn m128d_add0() {
  let a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([-8.0_f64, 4.0]);
  let out: [f64; 2] = cast(a.add0(b));
  assert_eq!(out, [-3.0, 6.0]);
}

#[test]
fn m128d_bitand() {
  let a: m128d = cast([core::u64::MAX, 0]);
  let b: m128d = cast([core::u64::MAX, core::u64::MAX]);
  let out: [u64; 2] = cast(a & b);
  assert_eq!(out, [core::u64::MAX, 0]);
}

#[test]
fn m128d_bitand_assign() {
  let mut a: m128d = cast([core::u64::MAX, 0]);
  let b: m128d = cast([core::u64::MAX, core::u64::MAX]);
  a &= b;
  let out: [u64; 2] = cast(a);
  assert_eq!(out, [core::u64::MAX, 0]);
}

#[test]
fn m128d_andnot() {
  let a: m128d = cast([core::u64::MAX, 0]);
  let b: m128d = cast([core::u64::MAX, core::u64::MAX]);
  let out: [u64; 2] = cast(a.andnot(b));
  assert_eq!(out, [0, core::u64::MAX]);
}

#[test]
fn m128d_cast_m128i() {
  let m: m128d = cast(12_345_678_u128);
  let mi: m128i = m.cast_m128i();
  let mi_bits: u128 = cast(mi);
  assert_eq!(mi_bits, 12_345_678_u128);
}
