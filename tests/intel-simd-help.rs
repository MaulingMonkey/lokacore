//!
//! Module for test helpers common to all Intel SIMD testing.
//!

#[cfg(target_arch = "x86")]
pub use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
pub use core::arch::x86_64::*;

#[cfg(target_arch = "x86")]
pub use lokacore::arch::x86::*;
#[cfg(target_arch = "x86_64")]
pub use lokacore::arch::x86_64::*;

pub use core::mem::*;
pub use lokacore::{shuffle128, *};

#[macro_export]
macro_rules! assert_approx_f32 {
  ($a:expr, $b:expr, $max_diff:expr) => {
    let diff: f32 = ($a - $b).abs();
    assert!(diff <= $max_diff, "diff was:{}", diff);
  };
}
