#![cfg(target_feature = "sse")]

#[cfg(target_arch = "x86")]
use lokacore::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use lokacore::arch::x86_64::*;
use lokacore::*;

#[test]
#[allow(clippy::float_cmp)]
fn sse_sanity_tests() {
  // Note(Lokathor): Here is where many sanity checks for our assumptions of
  // `__m128` will go.
  #[cfg(target_arch = "x86")]
  use core::arch::x86::*;
  #[cfg(target_arch = "x86_64")]
  use core::arch::x86_64::*;
  use core::mem::*;
  unsafe {
    // An `__m128` is "basically" an `[f32;4]`, and if you inspect the inside
    // with pointer math stuff you get floats at each offset that are the same
    // as the indexes of the array you transmuted to get the `__m128`.
    let m: __m128 = transmute([5.0_f32, 6.0, 7.0, 8.0]);
    let m_ptr: *const f32 = &m as *const __m128 as *const f32;
    assert_eq!(*m_ptr.offset(0), 5.0);
    assert_eq!(*m_ptr.offset(1), 6.0);
    assert_eq!(*m_ptr.offset(2), 7.0);
    assert_eq!(*m_ptr.offset(3), 8.0);

    // Confusingly, the `set` order places the data into the lanes _backwards_
    // from the array index ordering.
    let m: __m128 = _mm_set_ps(5.0, 6.0, 7.0, 8.0);
    let m_ptr: *const f32 = &m as *const __m128 as *const f32;
    assert_eq!(*m_ptr.offset(0), 8.0);
    assert_eq!(*m_ptr.offset(1), 7.0);
    assert_eq!(*m_ptr.offset(2), 6.0);
    assert_eq!(*m_ptr.offset(3), 5.0);

    // And then the `setr` order places the data into the lanes the same as the
    // array index ordering.
    let m: __m128 = _mm_setr_ps(5.0, 6.0, 7.0, 8.0);
    let m_ptr: *const f32 = &m as *const __m128 as *const f32;
    assert_eq!(*m_ptr.offset(0), 5.0);
    assert_eq!(*m_ptr.offset(1), 6.0);
    assert_eq!(*m_ptr.offset(2), 7.0);
    assert_eq!(*m_ptr.offset(3), 8.0);

    // The "low" lane is offset 0
    let m: __m128 = _mm_set_ss(5.0);
    let m_ptr: *const f32 = &m as *const __m128 as *const f32;
    assert_eq!(*m_ptr.offset(0), 5.0);
    assert_eq!(*m_ptr.offset(1), 0.0);
    assert_eq!(*m_ptr.offset(2), 0.0);
    assert_eq!(*m_ptr.offset(3), 0.0);
    // Some operations affect only the low lane.
    {
      let b: __m128 = _mm_set1_ps(3.0);
      // PACKED (adds all lanes)
      let m_b = _mm_add_ps(m, b);
      let m_b_ptr: *const f32 = &m_b as *const __m128 as *const f32;
      assert_eq!(*m_b_ptr.offset(0), 8.0);
      assert_eq!(*m_b_ptr.offset(1), 3.0);
      assert_eq!(*m_b_ptr.offset(2), 3.0);
      assert_eq!(*m_b_ptr.offset(3), 3.0);
      // SINGLE (adds only offset0, other lanes copy "self", `m`)
      let m_b = _mm_add_ss(m, b);
      let m_b_ptr: *const f32 = &m_b as *const __m128 as *const f32;
      assert_eq!(*m_b_ptr.offset(0), 8.0);
      assert_eq!(*m_b_ptr.offset(1), 0.0);
      assert_eq!(*m_b_ptr.offset(2), 0.0);
      assert_eq!(*m_b_ptr.offset(3), 0.0);
    }
  }
}

macro_rules! lanes_eq {
  ($a:expr, $b:expr) => {
    let mut a_values = Align16([0.0; 4]);
    let mut b_values = Align16([0.0; 4]);
    $a.store(&mut a_values);
    $b.store(&mut b_values);
    let a_bits: u128 = cast(a_values);
    let b_bits: u128 = cast(b_values);
    assert_eq!(a_bits, b_bits, "a_bits:{:b}, b_bits:{:b}", a_bits, b_bits);
  };
}

macro_rules! check_masks {
  ($x:expr, [$b3:literal, $b2:literal, $b1:literal, $b0:literal]) => {
    let f = f32::from_bits(core::u32::MAX);
    let l3 = if $b3 { f } else { 0.0 };
    let l2 = if $b2 { f } else { 0.0 };
    let l1 = if $b1 { f } else { 0.0 };
    let l0 = if $b0 { f } else { 0.0 };
    let target = m128::set(l3, l2, l1, l0);
    lanes_eq!($x, target);
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
  check_masks!(a & b, [false, false, true, false]);
}

#[test]
fn m128_bitor() {
  let all = core::u32::MAX;
  let a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  check_masks!(a | b, [true, true, true, false]);
}

#[test]
fn m128_bitxor() {
  let all = core::u32::MAX;
  let a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  check_masks!(a ^ b, [true, true, false, false]);
}

#[test]
fn m128_bitand_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a &= b;
  check_masks!(a, [false, false, true, false]);
}

#[test]
fn m128_bitor_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a |= b;
  check_masks!(a, [true, true, true, false]);
}

#[test]
fn m128_bitxor_assign() {
  let all = core::u32::MAX;
  let mut a = m128::set(f32::from_bits(all), 0.0, f32::from_bits(all), 0.0);
  let b = m128::set(0.0, f32::from_bits(all), f32::from_bits(all), 0.0);
  a ^= b;
  check_masks!(a, [true, true, false, false]);
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
