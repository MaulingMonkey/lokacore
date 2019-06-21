#![no_std]
#![warn(missing_docs)]

//! Lokathor's crate of core-only odds and ends.

use core::{
  mem::{align_of, size_of},
  num::*,
  ptr::NonNull,
};

macro_rules! impl_unsafe_marker_for_array {
  ( $marker:ident , $( $n:expr ),* ) => {
    $(unsafe impl<T> $marker for [T; $n] where T: $marker {})*
  }
}

/// A trait for types that can be safely made with [zeroed](core::mem::zeroed).
///
/// ## Safety
///
/// * Your type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * Your type must be allowed to be an "all zeroes" bit pattern (eg: no
///   [NonNull<T>](core::ptr::NonNull)).
pub unsafe trait Zeroable: Sized {
  /// Calls [zeroed](core::mem::zeroed).
  ///
  /// This is a trait method so that you can write `MyType::zeroed()` in your
  /// code. It is a contract of this trait that if you implement it on your type
  /// you _must not_ override this method.
  fn zeroed() -> Self {
    unsafe { core::mem::zeroed() }
  }
}
unsafe impl Zeroable for () {}
unsafe impl Zeroable for bool {}
unsafe impl Zeroable for char {}
unsafe impl Zeroable for u8 {}
unsafe impl Zeroable for i8 {}
unsafe impl Zeroable for u16 {}
unsafe impl Zeroable for i16 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for u64 {}
unsafe impl Zeroable for i64 {}
unsafe impl Zeroable for usize {}
unsafe impl Zeroable for isize {}
unsafe impl Zeroable for u128 {}
unsafe impl Zeroable for i128 {}
unsafe impl Zeroable for f32 {}
unsafe impl Zeroable for f64 {}
unsafe impl Zeroable for Option<NonZeroI8> {}
unsafe impl Zeroable for Option<NonZeroI16> {}
unsafe impl Zeroable for Option<NonZeroI32> {}
unsafe impl Zeroable for Option<NonZeroI64> {}
unsafe impl Zeroable for Option<NonZeroI128> {}
unsafe impl Zeroable for Option<NonZeroIsize> {}
unsafe impl Zeroable for Option<NonZeroU8> {}
unsafe impl Zeroable for Option<NonZeroU16> {}
unsafe impl Zeroable for Option<NonZeroU32> {}
unsafe impl Zeroable for Option<NonZeroU64> {}
unsafe impl Zeroable for Option<NonZeroU128> {}
unsafe impl Zeroable for Option<NonZeroUsize> {}
unsafe impl<T> Zeroable for *mut T {}
unsafe impl<T> Zeroable for *const T {}
unsafe impl<T> Zeroable for Option<NonNull<T>> {}
impl_unsafe_marker_for_array!(
  Zeroable, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024
);

/// A marker trait for "plain old data".
///
/// The point of this trait is that once something is marked "plain old data"
/// you can really go to town with the bit fiddling and bit casting. Therefore,
/// it's a relatively strong claim to make about a type. Do not add this to your
/// type casually.
///
/// ## Safety
///
/// * The type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * The type must allow any bit pattern (eg: no `bool` or `char`).
/// * The type must not contain any padding bytes (eg: no `(u8, u16)`).
pub unsafe trait Pod: Zeroable + Copy {}

unsafe impl Pod for () {}
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
impl_unsafe_marker_for_array!(
  Pod, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
  25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024
);

/// The things that can go wrong when casting a slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SliceCastError {
  /// You tried to cast a slice to an element type with a higher alignment
  /// requirement but the slice wasn't aligned.
  TargetAlignmentGreaterAndInputNotAligned,
  /// You tried to cast between a zero-sized type and a non-zero-sized type.
  /// Because the output slice resizes based on the input and output types, it's
  /// fairly nonsensical to throw a ZST into the mix. You can go from a ZST to
  /// another ZST, if you want.
  CantConvertBetweenZSTAndNonZST,
  /// If the element size changes then the output slice changes length
  /// accordingly. If the output slice wouldn't be a whole number of elements
  /// then the conversion fails.
  OutputSliceWouldHaveSlop,
}

/// Try to convert a slice of one type into another.
///
/// * `input.as_ptr() as usize == output.as_ptr() as usize`
/// * `input.len() * size_of::<A>() == output.len() * size_of::<B>()`
///
/// ## Failure
///
/// * If the target type has a greater alignment requirement and the input slice
///   isn't aligned.
/// * If the target element type is a different size from the current element
///   type, and the output slice wouldn't be a whole number of elements when
///   accounting for the size change (eg: three `u16` values is 1.5 `u32`
///   values, so that's a failure).
/// * Similarly, you can't convert between a
///   [ZST](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)
///   and a non-ZST.
pub fn try_cast_slice<A: Pod, B: Pod>(a: &[A]) -> Result<&[B], SliceCastError> {
  if align_of::<B>() > align_of::<A>() && (a.as_ptr() as *const A as usize) % align_of::<B>() != 0 {
    // If the alignment requirement goes up then we check for possible
    // mis-alignment and error out if that's the case.
    Err(SliceCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else {
    if size_of::<B>() == size_of::<A>() {
      // If the sizes are the same this is a totally plain cast, even for ZST
      Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, a.len()) })
    } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
      // If the sizes aren't the same and one of them is a ZST, this is
      // hopeless.
      Err(SliceCastError::CantConvertBetweenZSTAndNonZST)
    } else {
      if core::mem::size_of_val(a) % size_of::<B>() != 0 {
        Err(SliceCastError::OutputSliceWouldHaveSlop)
      } else {
        let new_len = core::mem::size_of_val(a) / size_of::<B>();
        Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, new_len) })
      }
    }
  }
}

/// As [try_cast_slice](try_cast_slice), but unwraps the result for you.
pub fn cast_slice<A: Pod, B: Pod>(a: &[A]) -> &[B] {
  try_cast_slice(a).unwrap()
}
