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

#[test]
fn m128d_cmp_eq() {
  let max = core::u64::MAX;
  let a: m128d = cast([5.0, 6.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_eq(b)), [max, 0]);
}

#[test]
fn m128d_cmp_eq0() {
  let max = core::u64::MAX;
  let a: m128d = cast([5.0, 6.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_eq0(b)), [max, 6.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_ge() {
  let max = core::u64::MAX;
  let a: m128d = cast([5.0, 12.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ge(b)), [max, max]);
}

#[test]
fn m128d_cmp_ge0() {
  let max = core::u64::MAX;
  let a: m128d = cast([5.0, 6.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ge0(b)), [max, 6.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_gt() {
  let max = core::u64::MAX;
  let a: m128d = cast([5.0, 12.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_gt(b)), [0, max]);
}

#[test]
fn m128d_cmp_gt0() {
  let a: m128d = cast([5.0, 6.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_gt0(b)), [0, 6.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_le() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_le(b)), [max, max]);
}

#[test]
fn m128d_cmp_le0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_le0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_lt() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_lt(b)), [max, 0]);
}

#[test]
fn m128d_cmp_lt0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_lt0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_ne() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ne(b)), [max, 0]);
}

#[test]
fn m128d_cmp_ne0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ne0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_nge() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nge(b)), [max, 0]);
}

#[test]
fn m128d_cmp_nge0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nge0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_ngt() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ngt(b)), [max, max]);
}

#[test]
fn m128d_cmp_ngt0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ngt0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_nle() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nle(b)), [0, 0]);
}

#[test]
fn m128d_cmp_nle0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nle0(b)), [0, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_nlt() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nlt(b)), [0, max]);
}

#[test]
fn m128d_cmp_nlt0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nlt0(b)), [0, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_ordinary() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ordinary(b)), [max, max]);
}

#[test]
fn m128d_cmp_ordinary0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_ordinary0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmp_nan() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0_f64.to_bits(), max]);
  let b: m128d = cast([5.0, 7.0]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nan(b)), [0, max]);
}

#[test]
fn m128d_cmp_nan0() {
  let max = core::u64::MAX;
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([max, 7.0_f64.to_bits()]);
  assert_eq!(cast::<m128d, [u64;2]>(a.cmp_nan0(b)), [max, 7.0_f64.to_bits()]);
}

#[test]
fn m128d_cmpi_eq0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_eq0(b), 1);
}

#[test]
fn m128d_cmpi_ge0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_ge0(b), 1);
}

#[test]
fn m128d_cmpi_gt0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_gt0(b), 0);
}

#[test]
fn m128d_cmpi_le0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_le0(b), 1);
}

#[test]
fn m128d_cmpi_lt0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_lt0(b), 0);
}

#[test]
fn m128d_cmpi_ne0() {
  let a: m128d = cast([4.0, 7.0]);
  let b: m128d = cast([4.0, 5.0]);
  assert_eq!(a.cmpi_ne0(b), 0);
}

#[test]
fn m128d_round_i32x4() {
  let a: m128d = cast([4.8, 7.1]);
  let r: m128i = a.round_i32x4();
  let r_i32s: [i32;4] = cast(r);
  assert_eq!(r_i32s, [5, 7, 0, 0]);
}

#[test]
fn m128d_round_f32x4() {
  let a: m128d = cast([4.5, 7.1]);
  let r: m128 = a.round_f32x4();
  let r_f32s: [f32;4] = cast(r);
  assert_eq!(r_f32s, [4.5, 7.1, 0.0, 0.0]);
}

#[test]
#[allow(clippy::float_cmp)]
fn m128d_extract0() {
  let a: m128d = cast([4.5, 7.1]);
  assert_eq!(a.extract0(), 4.5_f64);
}

#[test]
fn m128d_round_i32_extract0() {
  let a: m128d = cast([4.5, 7.1]);
  assert_eq!(a.round_i32_extract0(), 4_i32);
}

#[test]
#[cfg(target_arch="x86_64")]
fn m128d_round_i64_extract0() {
  let a: m128d = cast([4.5, 7.1]);
  assert_eq!(a.round_i64_extract0(), 4_i64);
}

#[test]
fn m128d_replace0_with_i32() {
  let a: m128d = cast([4.5, 7.1]);
  let b: m128d = a.replace0_with_i32(20_i32);
  let c: [f64;2] = cast(b);
  assert_eq!(c, [20.0, 7.1]);
}

#[test]
fn m128d_replace0_with_i64() {
  let a: m128d = cast([4.5, 7.1]);
  let b: m128d = a.replace0_with_i64(20_i64);
  let c: [f64;2] = cast(b);
  assert_eq!(c, [20.0, 7.1]);
}

#[test]
fn m128d_replace0_with_f32() {
  let a: m128d = cast([4.5, 7.1]);
  let b: m128d = a.replace0_with_f32(m128::set0(8.0));
  let c: [f64;2] = cast(b);
  assert_eq!(c, [8.0, 7.1]);
}

#[test]
fn m128d_truncate_i32x4() {
  let a: m128d = cast([4.8, 7.1]);
  let r: m128i = a.truncate_i32x4();
  let r_i32s: [i32;4] = cast(r);
  assert_eq!(r_i32s, [4, 7, 0, 0]);
}

#[test]
fn m128d_truncate0_i32() {
  let a: m128d = cast([4.5, 7.1]);
  assert_eq!(a.truncate0_i32(), 4_i32);
}

#[test]
#[cfg(target_arch="x86_64")]
fn m128d_truncate0_i64() {
  let a: m128d = cast([4.5, 7.1]);
  assert_eq!(a.truncate0_i64(), 4_i64);
}

#[test]
fn m128d_div() {
  let a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([-2.0_f64, 3.0]);
  let out: [f64; 2] = cast(a / b);
  assert_eq!(out, [-2.5, 2.0]);
}

#[test]
fn m128d_div_assign() {
  let mut a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([-2.0_f64, 3.0]);
  a /= b;
  let out: [f64; 2] = cast(a);
  assert_eq!(out, [-2.5, 2.0]);
}

#[test]
fn m128d_div0() {
  let a: m128d = cast([5.0_f64, 6.0]);
  let b: m128d = cast([2.0_f64, 4.0]);
  let out: [f64; 2] = cast(a.div0(b));
  assert_eq!(out, [2.5, 6.0]);
}
