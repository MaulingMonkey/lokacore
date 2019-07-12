#![cfg(target_feature = "sse")]

#[cfg(target_arch = "x86")]
use lokacore::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use lokacore::arch::x86_64::*;
use lokacore::*;

macro_rules! lanes_eq {
  ($a:expr, $b:expr) => {
    assert_eq!($a.cmp_eq($b).move_mask(), 15);
  };
}

#[test]
fn m128_debug() {
  let m = m128::set(5.0, 6.0, 7.0, 8.5);
  assert_eq!(&format!("{:?}", m), "m128(5, 6, 7, 8.5)");
}

#[test]
fn m128_add() {
  let a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(6.0, 8.0, 10.0, 12.5);
  lanes_eq!(a + b, expected);
}

#[test]
fn m128_sub() {
  let a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(4.0, 4.0, 4.0, 4.5);
  lanes_eq!(a - b, expected);
}

#[test]
fn m128_div() {
  let a = m128::set(5.0, 6.0, 9.0, 8.0);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(5.0, 3.0, 3.0, 2.0);
  lanes_eq!(a / b, expected);
}

#[test]
fn m128_mul() {
  let a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(5.0, 12.0, 21.0, 34.0);
  lanes_eq!(a * b, expected);
}

#[test]
fn m128_add_assign() {
  let mut a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(6.0, 8.0, 10.0, 12.5);
  a += b;
  lanes_eq!(a, expected);
}

#[test]
fn m128_sub_assign() {
  let mut a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(4.0, 4.0, 4.0, 4.5);
  a -= b;
  lanes_eq!(a, expected);
}

#[test]
fn m128_div_assign() {
  let mut a = m128::set(5.0, 6.0, 9.0, 8.0);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(5.0, 3.0, 3.0, 2.0);
  a /= b;
  lanes_eq!(a, expected);
}

#[test]
fn m128_mul_assign() {
  let mut a = m128::set(5.0, 6.0, 7.0, 8.5);
  let b = m128::set(1.0, 2.0, 3.0, 4.0);
  let expected = m128::set(5.0, 12.0, 21.0, 34.0);
  a *= b;
  lanes_eq!(a, expected);
}

#[test]
fn m128_neg() {
  let a = m128::set(5.0, 6.0, 7.0, 8.5);
  let expected = m128::set(-5.0, -6.0, -7.0, -8.5);
  lanes_eq!(-a, expected);
  lanes_eq!(-(-a), a);
}

#[test]
fn m128_bitand() {
  let all = core::u32::MAX;
  let a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  assert_eq!(&format!("{:?}", a & b), "m128(0, 0, NaN, 0)");
}

#[test]
fn m128_bitor() {
  let all = core::u32::MAX;
  let a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  assert_eq!(&format!("{:?}", a | b), "m128(NaN, NaN, NaN, 0)");
}

#[test]
fn m128_bitxor() {
  let all = core::u32::MAX;
  let a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  assert_eq!(&format!("{:?}", a ^ b), "m128(NaN, NaN, 0, 0)");
}

#[test]
fn m128_bitand_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a &= b;
  assert_eq!(&format!("{:?}", a), "m128(0, 0, NaN, 0)");
}

#[test]
fn m128_bitor_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a |= b;
  assert_eq!(&format!("{:?}", a), "m128(NaN, NaN, NaN, 0)");
}

#[test]
fn m128_bitxor_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a ^= b;
  assert_eq!(&format!("{:?}", a), "m128(NaN, NaN, 0, 0)");
}

#[test]
fn m128_set_group() {
  lanes_eq!(m128::set(0.0, 0.0, 0.0, 0.0), m128::zeroed());
  lanes_eq!(
    m128::set(1.0, 2.0, 3.0, 4.0),
    m128::set_reverse(4.0, 3.0, 2.0, 1.0)
  );
  lanes_eq!(m128::splat(2.0), m128::set(2.0, 2.0, 2.0, 2.0));
  lanes_eq!(m128::set_low(2.0), m128::set(0.0, 0.0, 0.0, 2.0));
}

#[test]
fn m128_load_group() {
  let a = Align16([1.0, 2.0, 3.0, 4.0]);
  lanes_eq!(m128::load(&a), m128::set(4.0, 3.0, 2.0, 1.0));
  let b = Align16([9.0, 8.0, 7.0, 6.0]);
  lanes_eq!(m128::load_reverse(&b), m128::set(9.0, 8.0, 7.0, 6.0));
  lanes_eq!(m128::load_splat(&3.5), m128::splat(3.5));
  lanes_eq!(m128::load_low(&3.5), m128::set_low(3.5));
}

#[test]
fn m128_store_group() {
  let mut a = Align16([1.0, 2.0, 3.0, 4.0]);
  let mut ma = m128::load(&a);
  ma *= m128::splat(2.0);
  ma.store(&mut a);
  assert_eq!(a.0, [2.0, 4.0, 6.0, 8.0]);
  ma.store_reverse(&mut a);
  assert_eq!(a.0, [8.0, 6.0, 4.0, 2.0]);
  ma.store_splat(&mut a);
  assert_eq!(a.0, [2.0, 2.0, 2.0, 2.0]);
  ma *= m128::splat(3.0);
  ma.store_low(&mut a.0[1]);
  assert_eq!(a.0, [2.0, 6.0, 2.0, 2.0]);
}

#[test]
fn test_shuffle() {
  let a = m128::set(9.0, 8.0, 7.0, 6.0);

  let output = shuffle128!(a, a, 0, 0, 0, 0);
  let expected = m128::splat(6.0);
  assert_eq!(0b1111, expected.cmp_eq(output).move_mask());

  let output = shuffle128!(a, a, 0, 0, 0, 1);
  let expected = m128::set(6.0, 6.0, 6.0, 7.0);
  assert_eq!(0b1111, expected.cmp_eq(output).move_mask());
}
