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
