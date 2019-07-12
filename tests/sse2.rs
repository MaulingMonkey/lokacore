#![cfg(target_feature = "sse2")]

#[cfg(target_arch = "x86")]
use lokacore::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use lokacore::arch::x86_64::*;
//use lokacore::*;

#[test]
fn m128i_debug() {
  let m = m128i::set_i32(-1, 0, 1, 15);
  let expected = (u128::from(-1i32 as u32) << 96) as i128 | 1 << 32 | 15;
  assert_eq!(&format!("{:?}", m), &format!("m128i({})", expected));
}

#[test]
fn m128d_debug() {
  extern crate std;
  let m = m128d::set(5.0, 6.5);
  assert_eq!(&std::format!("{:?}", m), "m128d(5, 6.5)");
}
