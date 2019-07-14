#![cfg(target_feature="sse")]

#[path="intel-simd-help.rs"]
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
