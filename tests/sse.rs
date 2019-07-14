#![cfg(target_feature = "sse")]

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
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, false), (5.0, true), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)].iter().copied() {
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
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)].iter().copied() {
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
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, true), (5.0, false), (6.0, false)].iter().copied() {
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
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)].iter().copied() {
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
  for (f, t) in [(4.0_f32, false), (5.0, false), (6.0, true)].iter().copied() {
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
  assert_eq!(a.round_extract0_i32(), 5);
  let a: m128 = cast([5.3_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 5);
  let a: m128 = cast([5.5_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 6);
  let a: m128 = cast([5.7_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), 6);
  let a: m128 = cast([-1.2_f32, 0.0, 0.0, 0.0]);
  assert_eq!(a.round_extract0_i32(), -1);
}
