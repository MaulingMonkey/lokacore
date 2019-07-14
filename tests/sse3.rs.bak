#![cfg(target_feature = "sse3")]

#[cfg(target_arch = "x86")]
use lokacore::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use lokacore::arch::x86_64::*;

#[test]
fn m128d_add_sub() {
  let a = m128d::splat(5.0);
  let b = m128d::set(2.0, 1.0);
  let out = a.add_sub(b);
  let expected = m128d::set(7.0, 4.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128_add_sub() {
  let a = m128::splat(5.0);
  let b = m128::set(2.0, 1.0, 3.0, 7.0);
  let out = a.add_sub(b);
  let expected = m128::set(7.0, 4.0, 8.0, -2.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128d_horizontal_add() {
  let a = m128d::set(3.0, 4.0);
  let b = m128d::set(5.0, 6.0);
  let out = a.horizontal_add(b);
  let expected = m128d::set(11.0, 7.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128_horizontal_add() {
  let a = m128::set(3.0, 4.0, 8.0, 2.0);
  let b = m128::set(5.0, 6.0, 10.0, 9.0);
  let out = a.horizontal_add(b);
  let expected = m128::set(11.0, 19.0, 7.0, 10.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128d_horizontal_sub() {
  let a = m128d::set(9.0, 3.0);
  let b = m128d::set(5.0, 6.0);
  let out = a.horizontal_sub(b);
  let expected = m128d::set(1.0, -6.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128_horizontal_sub() {
  let a = m128::set(3.0, 12.0, 8.0, 2.0);
  let b = m128::set(5.0, 6.0, 10.0, 9.0);
  let out = a.horizontal_sub(b);
  let expected = m128::set(1.0, -1.0, 9.0, -6.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128i_load_quick_unaligned() {
  let out = m128i::load_quick_unaligned(&12345);
  let expected = m128i::load_unaligned(&12345);
  assert_eq!(out.to_i128(), expected.to_i128());
}

#[test]
fn m128d_load_splat() {
  let out = m128d::load_splat(&5.0);
  let expected = m128d::splat(5.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128d_duplicate_low() {
  let a = m128d::set(3.0, 12.0);
  let out = a.duplicate_low();
  let expected = m128d::set(12.0, 12.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128_duplicate_odd() {
  let a = m128::set(3.0, 12.0, 7.0, 6.0);
  let out = a.duplicate_odd();
  let expected = m128::set(3.0, 3.0, 7.0, 7.0);
  assert_eq!(out.to_array(), expected.to_array());
}

#[test]
fn m128_duplicate_even() {
  let a = m128::set(3.0, 12.0, 7.0, 6.0);
  let out = a.duplicate_even();
  let expected = m128::set(12.0, 12.0, 6.0, 6.0);
  assert_eq!(out.to_array(), expected.to_array());
}
