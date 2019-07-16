#![cfg(target_feature = "sse")]
#![allow(bad_style)]

#[path = "intel-simd-help.rs"]
mod intel_simd_help;
use intel_simd_help::*;

#[test]
fn m128_debug() {
  let m: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  assert_eq!(&format!("{:?}", m), "m128(5.0, 6.0, 7.0, 8.5)");
}

#[test]
fn m128_display() {
  let m: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  assert_eq!(&format!("{}", m), "m128(5, 6, 7, 8.5)");
}

#[test]
fn m128_lower_exp() {
  let m: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  assert_eq!(&format!("{:e}", m), "m128(5e0, 6e0, 7e0, 8.5e0)");
}

#[test]
fn m128_upper_exp() {
  let m: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  assert_eq!(&format!("{:E}", m), "m128(5E0, 6E0, 7E0, 8.5E0)");
}

#[test]
fn m128_add() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 1.0, 0.5]);
  let out: [f32; 4] = cast(a + b);
  assert_eq!(out, [-3.0, 10.0, 8.0, 9.0]);
}

#[test]
fn m128_add_assign() {
  let mut a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 1.0, 0.5]);
  a += b;
  let out: [f32; 4] = cast(a);
  assert_eq!(out, [-3.0, 10.0, 8.0, 9.0]);
}

#[test]
fn m128_add0() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 1.0, 0.5]);
  let out: [f32; 4] = cast(a.add0(b));
  assert_eq!(out, [-3.0_f32, 6.0, 7.0, 8.5]);
}

#[test]
fn m128_bitand() {
  let max = core::u32::MAX;
  let a: m128 = cast([0, max, max, 0]);
  let b: m128 = cast([max, 0, max, 0]);
  let out: [u32; 4] = cast(a & b);
  assert_eq!(out, [0, 0, max, 0]);
}

#[test]
fn m128_bitand_assign() {
  let max = core::u32::MAX;
  let mut a: m128 = cast([0, max, max, 0]);
  let b: m128 = cast([max, 0, max, 0]);
  a &= b;
  let out: [u32; 4] = cast(a);
  assert_eq!(out, [0, 0, max, 0]);
}

#[test]
fn m128_andnot() {
  let max = core::u32::MAX;
  let a: m128 = cast([0, max, max, 0]);
  let b: m128 = cast([max, 0, max, 0]);
  let out: [u32; 4] = cast(a.andnot(b));
  assert_eq!(out, [max, 0, 0, 0]);
}

#[test]
fn m128_cmp_eq() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_eq(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_eq0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_eq0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_ge() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ge(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_ge0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ge0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_gt() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_gt(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_gt0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_gt0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_le() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, true), (6.0, false)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_le(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_le0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, true), (6.0, false)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_le0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_lt() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_lt(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_lt0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_lt0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_ne() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ne(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_ne0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ne0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_nge() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nge(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_nge0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nge0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_ngt() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, true), (6.0, false)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ngt(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_ngt0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, true), (5.0, true), (6.0, false)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_ngt0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_nle() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nle(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_nle0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)]
    .iter()
    .copied()
  {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nle0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_nlt() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nlt(b));
    if t {
      assert_eq!(out, [max, max, max, max]);
    } else {
      assert_eq!(out, [0, 0, 0, 0]);
    }
  }
}

#[test]
fn m128_cmp_nlt0() {
  let max = core::u32::MAX;
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, true)].iter().copied() {
    let a: m128 = cast([f, f, f, f]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    let out: [u32; 4] = cast(a.cmp_nlt0(b));
    if t {
      assert_eq!(out, [max, f.to_bits(), f.to_bits(), f.to_bits()]);
    } else {
      assert_eq!(out, [0, f.to_bits(), f.to_bits(), f.to_bits()]);
    }
  }
}

#[test]
fn m128_cmp_ordinary() {
  let max = core::u32::MAX;
  let a: m128 = cast([1.0, f32::from_bits(max), 1.0, f32::from_bits(max)]);
  let b: m128 = cast([1.0, 1.0, f32::from_bits(max), f32::from_bits(max)]);
  let out: [u32; 4] = cast(a.cmp_ordinary(b));
  assert_eq!(out, [max, 0, 0, 0]);
}

#[test]
fn m128_cmp_ordinary0() {
  let max = core::u32::MAX;
  let a: m128 = cast([1.0, f32::from_bits(max), 1.0, f32::from_bits(max)]);
  let b: m128 = cast([1.0, 1.0, f32::from_bits(max), f32::from_bits(max)]);
  let out: [u32; 4] = cast(a.cmp_ordinary0(b));
  assert_eq!(out, [max, max, 1.0_f32.to_bits(), max]);
}

#[test]
fn m128_cmp_nan() {
  let max = core::u32::MAX;
  let a: m128 = cast([1.0, f32::from_bits(max), 1.0, f32::from_bits(max)]);
  let b: m128 = cast([1.0, 1.0, f32::from_bits(max), f32::from_bits(max)]);
  let out: [u32; 4] = cast(a.cmp_nan(b));
  assert_eq!(out, [0, max, max, max]);
}

#[test]
fn m128_cmp_nan0() {
  let max = core::u32::MAX;
  let a: m128 = cast([1.0, f32::from_bits(max), 1.0, f32::from_bits(max)]);
  let b: m128 = cast([1.0, 1.0, f32::from_bits(max), f32::from_bits(max)]);
  let out: [u32; 4] = cast(a.cmp_nan0(b));
  assert_eq!(out, [0, max, 1.0_f32.to_bits(), max]);
}

#[test]
fn m128_cmpi_eq0() {
  for (f, i) in [(4.0_f32, 0), (5.0, 1), (6.0, 0)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_eq0(b));
  }
}

#[test]
fn m128_cmpi_ge0() {
  for (f, i) in [(4.0_f32, 0), (5.0, 1), (6.0, 1)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_ge0(b));
  }
}

#[test]
fn m128_cmpi_gt0() {
  for (f, i) in [(4.0_f32, 0), (5.0, 0), (6.0, 1)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_gt0(b));
  }
}

#[test]
fn m128_cmpi_le0() {
  for (f, i) in [(4.0_f32, 1), (5.0, 1), (6.0, 0)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_le0(b));
  }
}

#[test]
fn m128_cmpi_lt0() {
  for (f, i) in [(4.0_f32, 1), (5.0, 0), (6.0, 0)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_lt0(b));
  }
}

#[test]
fn m128_cmpi_ne0() {
  for (f, i) in [(4.0_f32, 1), (5.0, 0), (6.0, 1)].iter().copied() {
    let a: m128 = cast([f, 0.0, 0.0, 0.0]);
    let b: m128 = cast([5.0_f32, 5.0, 5.0, 5.0]);
    assert_eq!(i, a.cmpi_ne0(b));
  }
}

#[test]
fn m128_round_replace0_i32() {
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 1.0]);
  for i in [0, -1, i32::max_value(), i32::min_value()].iter().copied() {
    let out: [u32; 4] = cast(a.round_replace0_i32(i));
    assert_eq!(out, [(i as f32).to_bits(), 0, 0, 1.0_f32.to_bits()]);
  }
}

#[test]
fn m128_round_extract0_i32() {
  // Note(Lokathor): These asserts are for the default round mode, "round
  // nearest", which rounds to even if two values are equally close.
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 5_i32);
  let a: m128 = cast([5.3_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 5_i32);
  let a: m128 = cast([5.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 6_i32);
  let a: m128 = cast([5.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 6_i32);
  let a: m128 = cast([-1.2_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), -1_i32);
  let a: m128 = cast([-1.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), -2_i32);
  let a: m128 = cast([-1.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), -2_i32);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn m128_round_replace0_i64() {
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 1.0]);
  for i in [0, -1, i64::max_value(), i64::min_value()].iter().copied() {
    let out: [u32; 4] = cast(a.round_replace0_i64(i));
    assert_eq!(out, [(i as f32).to_bits(), 0, 0, 1.0_f32.to_bits()]);
  }
}

#[test]
fn m128_extract0_f32() {
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 1.0]);
  let f: f32 = a.extract0_f32();
  assert_eq!(f.to_bits(), 5.0_f32.to_bits());
}

#[test]
#[cfg(target_arch = "x86_64")]
fn m128_round_extract0_i64() {
  // Note(Lokathor): These asserts are for the default round mode, "round
  // nearest", which rounds to even if two values are equally close.
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), 5_i64);
  let a: m128 = cast([5.3_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), 5_i64);
  let a: m128 = cast([5.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), 6_i64);
  let a: m128 = cast([5.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), 6_i64);
  let a: m128 = cast([-1.2_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), -1_i64);
  let a: m128 = cast([-1.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), -2_i64);
  let a: m128 = cast([-1.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i64(), -2_i64);
}

#[test]
fn m128_truncate_extract0_i32() {
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), 5_i32);
  let a: m128 = cast([5.3_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), 5_i32);
  let a: m128 = cast([5.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), 5_i32);
  let a: m128 = cast([5.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), 5_i32);
  let a: m128 = cast([-1.2_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), -1_i32);
  let a: m128 = cast([-1.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), -1_i32);
  let a: m128 = cast([-1.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i32(), -1_i32);
}

#[test]
#[cfg(target_arch = "x86_64")]
fn m128_truncate_extract0_i64() {
  let a: m128 = cast([5.0_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), 5_i64);
  let a: m128 = cast([5.3_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), 5_i64);
  let a: m128 = cast([5.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), 5_i64);
  let a: m128 = cast([5.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), 5_i64);
  let a: m128 = cast([-1.2_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), -1_i64);
  let a: m128 = cast([-1.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), -1_i64);
  let a: m128 = cast([-1.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.truncate_extract0_i64(), -1_i64);
}

#[test]
fn m128_div() {
  let a: m128 = cast([8.0_f32, 12.0, 15.0, 9.0]);
  let b: m128 = cast([-2.0_f32, 3.0, 2.0, 0.5]);
  let out: [f32; 4] = cast(a / b);
  assert_eq!(out, [-4.0, 4.0, 7.5, 18.0]);
}

#[test]
fn m128_div_assign() {
  let mut a: m128 = cast([8.0_f32, 12.0, 15.0, 9.0]);
  let b: m128 = cast([-2.0_f32, 3.0, 2.0, 0.5]);
  a /= b;
  let out: [f32; 4] = cast(a);
  assert_eq!(out, [-4.0, 4.0, 7.5, 18.0]);
}

#[test]
fn m128_div0() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([2.0_f32, 4.0, 1.0, 0.5]);
  let out: [f32; 4] = cast(a.div0(b));
  assert_eq!(out, [2.5_f32, 6.0, 7.0, 8.5]);
}

#[test]
fn test_exception_mask() {
  let _: ExceptionMask = exception_mask();
}

#[test]
fn ExceptionMask_methods() {
  let mut mask = unsafe { ExceptionMask::from_raw_unchecked(0) };
  //
  assert!(!mask.invalid());
  mask.set_invalid(true);
  assert!(mask.invalid());
  //
  assert!(!mask.denorm());
  mask.set_denorm(true);
  assert!(mask.denorm());
  //
  assert!(!mask.div_zero());
  mask.set_div_zero(true);
  assert!(mask.div_zero());
  //
  assert!(!mask.overflow());
  mask.set_overflow(true);
  assert!(mask.overflow());
  //
  assert!(!mask.underflow());
  mask.set_underflow(true);
  assert!(mask.underflow());
  //
  assert!(!mask.inexact());
  mask.set_inexact(true);
  assert!(mask.inexact());
  //
  let e = ExceptionMask::default();
  assert_eq!(e.to_raw(), 0);
  //
  let mut e = ExceptionMask::default();
  e.set_invalid(true);
  assert_eq!(e.to_raw(), _MM_MASK_INVALID);
  //
  let mut e = ExceptionMask::default();
  e.set_denorm(true);
  assert_eq!(e.to_raw(), _MM_MASK_DENORM);
  //
  let mut e = ExceptionMask::default();
  e.set_div_zero(true);
  assert_eq!(e.to_raw(), _MM_MASK_DIV_ZERO);
  //
  let mut e = ExceptionMask::default();
  e.set_overflow(true);
  assert_eq!(e.to_raw(), _MM_MASK_OVERFLOW);
  //
  let mut e = ExceptionMask::default();
  e.set_underflow(true);
  assert_eq!(e.to_raw(), _MM_MASK_UNDERFLOW);
  //
  let mut e = ExceptionMask::default();
  e.set_inexact(true);
  assert_eq!(e.to_raw(), _MM_MASK_INEXACT);
}

#[test]
fn test_exception_state() {
  let _: ExceptionState = exception_state();
}

#[test]
fn ExceptionState_methods() {
  let mut mask = unsafe { ExceptionState::from_raw_unchecked(0) };
  //
  assert!(!mask.invalid());
  mask.set_invalid(true);
  assert!(mask.invalid());
  //
  assert!(!mask.denorm());
  mask.set_denorm(true);
  assert!(mask.denorm());
  //
  assert!(!mask.div_zero());
  mask.set_div_zero(true);
  assert!(mask.div_zero());
  //
  assert!(!mask.overflow());
  mask.set_overflow(true);
  assert!(mask.overflow());
  //
  assert!(!mask.underflow());
  mask.set_underflow(true);
  assert!(mask.underflow());
  //
  assert!(!mask.inexact());
  mask.set_inexact(true);
  assert!(mask.inexact());
  //
  let e = ExceptionState::default();
  assert_eq!(e.to_raw(), 0);
  //
  let mut e = ExceptionState::default();
  e.set_invalid(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_INVALID);
  //
  let mut e = ExceptionState::default();
  e.set_denorm(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_DENORM);
  //
  let mut e = ExceptionState::default();
  e.set_div_zero(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_DIV_ZERO);
  //
  let mut e = ExceptionState::default();
  e.set_overflow(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_OVERFLOW);
  //
  let mut e = ExceptionState::default();
  e.set_underflow(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_UNDERFLOW);
  //
  let mut e = ExceptionState::default();
  e.set_inexact(true);
  assert_eq!(e.to_raw(), _MM_EXCEPT_INEXACT);
}

#[test]
fn test_flush_zero_mode_on() {
  let _: bool = flush_zero_mode_on();
}

#[test]
fn test_rounding_mode() {
  let _: RoundingMode = rounding_mode();
}

#[test]
fn test_control_status_register() {
  let _: u32 = control_status_register();
}

#[test]
fn m128_load() {
  let aligned_array = Align16([5.0_f32, 6.0, 7.0, 8.0]);
  let m_load: m128 = m128::load(&aligned_array);
  let u32x4_load: [u32; 4] = cast(m_load);
  let m_transmute: m128 = cast(aligned_array);
  let u32x4_transmute: [u32; 4] = cast(m_transmute);
  assert_eq!(u32x4_load[0], u32x4_transmute[0]);
  assert_eq!(u32x4_load[1], u32x4_transmute[1]);
  assert_eq!(u32x4_load[2], u32x4_transmute[2]);
  assert_eq!(u32x4_load[3], u32x4_transmute[3]);
  // Note(Lokathor): extra sanity check that offset0 == index0
  let lane0 = unsafe { *(&m_load as *const m128 as *const f32) };
  let lane0_bits = lane0.to_bits();
  assert_eq!(lane0_bits, aligned_array.0[0].to_bits());
}

#[test]
fn m128_load_splat() {
  let float = 5.0_f32;
  let m: m128 = m128::load_splat(&float);
  let m_bits: [u32; 4] = cast(m);
  let float_bits_x4: [u32; 4] = cast([float, float, float, float]);
  assert_eq!(m_bits, float_bits_x4);
}

#[test]
fn m128_load0() {
  let float = 5.0_f32;
  let m: m128 = m128::load0(&float);
  let m_bits: [u32; 4] = cast(m);
  let float_bits_x4: [u32; 4] = cast([float, 0.0, 0.0, 0.0]);
  assert_eq!(m_bits, float_bits_x4);
}

#[test]
fn m128_load_reverse() {
  let aligned_array = Align16([5.0_f32, 6.0, 7.0, 8.0]);
  let m_load_reverse: m128 = m128::load_reverse(&aligned_array);
  let u32x4_load_reverse: [u32; 4] = cast(m_load_reverse);
  let m_transmute: m128 = cast(aligned_array);
  let u32x4_transmute: [u32; 4] = cast(m_transmute);
  assert_eq!(u32x4_load_reverse[0], u32x4_transmute[3]);
  assert_eq!(u32x4_load_reverse[1], u32x4_transmute[2]);
  assert_eq!(u32x4_load_reverse[2], u32x4_transmute[1]);
  assert_eq!(u32x4_load_reverse[3], u32x4_transmute[0]);
  // Note(Lokathor): extra sanity check that offset0 == index3
  let lane0 = unsafe { *(&m_load_reverse as *const m128 as *const f32) };
  let lane0_bits = lane0.to_bits();
  assert_eq!(lane0_bits, aligned_array.0[3].to_bits());
}

#[test]
fn m128_load_unaligned() {
  let array = [5.0_f32, 6.0, 7.0, 8.0];
  let m_load_unaligned: m128 = m128::load_unaligned(&array);
  let u32x4_load: [u32; 4] = cast(m_load_unaligned);
  let m_transmute: m128 = cast(array);
  let u32x4_transmute: [u32; 4] = cast(m_transmute);
  assert_eq!(u32x4_load[0], u32x4_transmute[0]);
  assert_eq!(u32x4_load[1], u32x4_transmute[1]);
  assert_eq!(u32x4_load[2], u32x4_transmute[2]);
  assert_eq!(u32x4_load[3], u32x4_transmute[3]);
  // Note(Lokathor): extra sanity check that offset0 == index0
  let lane0 = unsafe { *(&m_load_unaligned as *const m128 as *const f32) };
  let lane0_bits = lane0.to_bits();
  assert_eq!(lane0_bits, array[0].to_bits());
}

#[test]
fn m128_max() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.max(b));
  assert_eq!(out, [5.0_f32, 6.0, 12.0, 8.5]);
}

#[test]
fn m128_max0() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.max0(b));
  assert_eq!(out, [5.0_f32, 6.0, 7.0, 8.5]);
}

#[test]
fn m128_min() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.min(b));
  assert_eq!(out, [-8.0_f32, 4.0, 7.0, 0.5]);
}

#[test]
fn m128_min0() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.min0(b));
  assert_eq!(out, [-8.0_f32, 6.0, 7.0, 8.5]);
}

#[test]
fn m128_copy0() {
  let a: m128 = cast([5.0_f32, 6.0, 13.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.copy0(b));
  assert_eq!(out, [-8.0_f32, 6.0, 13.0, 8.5]);
}

#[test]
fn m128_copy_high_low() {
  let a: m128 = cast([5.0_f32, 6.0, 13.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.copy_high_low(b));
  assert_eq!(out, [12.0_f32, 0.5, 13.0, 8.5]);
}

#[test]
fn m128_copy_low_high() {
  let a: m128 = cast([5.0_f32, 6.0, 13.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 12.0, 0.5]);
  let out: [f32; 4] = cast(a.copy_low_high(b));
  assert_eq!(out, [5.0_f32, 6.0, -8.0, 4.0]);
}

#[test]
fn m128_move_mask() {
  let max = core::u32::MAX;
  for target_mask in 0..16 {
    let arr: [u32; 4] = [
      max * (target_mask & 0b1),
      max * ((target_mask & 0b10) >> 1),
      max * ((target_mask & 0b100) >> 2),
      max * ((target_mask & 0b1000) >> 3),
    ];
    let m: m128 = cast(arr);
    let out_mask: i32 = m.move_mask();
    assert_eq!(out_mask as u32, target_mask);
  }
}

#[test]
fn m128_mul() {
  let a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 1.0, 0.5]);
  let out: [f32; 4] = cast(a * b);
  assert_eq!(out, [-40.0, 24.0, 7.0, 4.25]);
}

#[test]
fn m128_mul_assign() {
  let mut a: m128 = cast([5.0_f32, 6.0, 7.0, 8.5]);
  let b: m128 = cast([-8.0_f32, 4.0, 1.0, 0.5]);
  a *= b;
  let out: [f32; 4] = cast(a);
  assert_eq!(out, [-40.0, 24.0, 7.0, 4.25]);
}

#[test]
fn m128_bitor() {
  let max = core::u32::MAX;
  let a: m128 = cast([0, max, max, 0]);
  let b: m128 = cast([max, 0, max, 0]);
  let out: [u32; 4] = cast(a | b);
  assert_eq!(out, [max, max, max, 0]);
}

#[test]
fn m128_bitor_assign() {
  let max = core::u32::MAX;
  let mut a: m128 = cast([0, max, max, 0]);
  let b: m128 = cast([max, 0, max, 0]);
  a |= b;
  let out: [u32; 4] = cast(a);
  assert_eq!(out, [max, max, max, 0]);
}

#[test]
fn test_prefetch() {
  //Note(Lokathor): There's nothing to assert since there's no correctness
  //change between a prefetch or not, but we'll just call each once in case we
  //delete them or rename them on accident or something.
  prefetch0(&0);
  prefetch1(&1);
  prefetch2(&2);
  prefetch_nta(&['n', 't', 'a']);
}

//
// TODO: fix how we test for approximations
//

#[test]
fn m128_reciprocal() {
  let a: m128 = cast([2.0_f32, 3.0, 5.0, -12.3]);
  let out: [f32; 4] = cast(a.reciprocal());
  assert_approx_f32!(out[0], 1.0/2.0, 0.001);
  assert_approx_f32!(out[1], 1.0/3.0, 0.001);
  assert_approx_f32!(out[2], 1.0/5.0, 0.001);
  assert_approx_f32!(out[3], 1.0/-12.3, 0.001);
}

#[test]
#[allow(clippy::float_cmp)]
fn m128_reciprocal0() {
  let a: m128 = cast([2.0_f32, 6.0, 7.0, 8.5]);
  let out: [f32; 4] = cast(a.reciprocal0());
  assert_approx_f32!(out[0], 1.0/2.0, 0.001);
  assert_eq!(out[1], 6.0);
  assert_eq!(out[2], 7.0);
  assert_eq!(out[3], 8.5);
}
