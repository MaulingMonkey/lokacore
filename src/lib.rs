#![no_std]
#![warn(missing_docs)]

//! Lokathor's crate of core-only odds and ends.

use core::{num::*, ptr::NonNull};

/// A trait for "plain old data".
///
/// ## Safety
///
/// * The type must allow **any** bit pattern (eg: no `bool` or `char`).
/// * The type must **not** contain any padding bytes (eg: no `(u8, u16)`).
/// * The type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
///
/// Generally, if you can't use a type with [zeroed](core::mem::zeroed) then it
/// doesn't qualify for `Pod`, and if you can use it with `zeroed` then it
/// _still might not_ qualify as `Pod`.
pub unsafe trait Pod: Copy {}

unsafe impl Pod for u8 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for i64 {}
unsafe impl Pod for usize {}
unsafe impl Pod for isize {}
unsafe impl Pod for u128 {}
unsafe impl Pod for i128 {}
unsafe impl Pod for f32 {}
unsafe impl Pod for f64 {}
unsafe impl Pod for Option<NonZeroI8> {}
unsafe impl Pod for Option<NonZeroI16> {}
unsafe impl Pod for Option<NonZeroI32> {}
unsafe impl Pod for Option<NonZeroI64> {}
unsafe impl Pod for Option<NonZeroI128> {}
unsafe impl Pod for Option<NonZeroIsize> {}
unsafe impl Pod for Option<NonZeroU8> {}
unsafe impl Pod for Option<NonZeroU16> {}
unsafe impl Pod for Option<NonZeroU32> {}
unsafe impl Pod for Option<NonZeroU64> {}
unsafe impl Pod for Option<NonZeroU128> {}
unsafe impl Pod for Option<NonZeroUsize> {}
unsafe impl<T> Pod for *mut T {}
unsafe impl<T> Pod for *const T {}
unsafe impl<T> Pod for Option<NonNull<T>> {}
macro_rules! impl_pod_array {
  ( $( $n:expr ),* ) => {
    $(unsafe impl<T> Pod for [T; $n] where T: Pod {})*
  }
}
impl_pod_array!(
  0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
  27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024
);

/// Creates a zeroed value of the type.
pub fn pod_zeroed<T: Pod>() -> T {
  // This is safe because a POD must allow any bit pattern, thusly it allows a
  // zeroed bit pattern.
  unsafe { core::mem::zeroed() }
}
