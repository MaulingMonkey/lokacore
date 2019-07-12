#![no_std]
#![warn(missing_docs)]

//! Lokathor's crate of core-only odds and ends.

use core::{
  marker::PhantomData,
  mem::{align_of, size_of},
  num::*,
  ptr::NonNull,
};

pub mod arch;

mod marker;
pub use marker::*;

/// Fiddly to use, but totally gets you the minimum without branching.
///
/// Works for any integral type.
#[macro_export]
macro_rules! branchless_min {
  ($x:ident, $y:ident, $u:ty) => {
    $y ^ (($x ^ $y) & (<$u>::wrapping_neg(($x < $y) as $u)))
  };
}

/// Fiddly to use, but totally gets you the maximum without branching.
///
/// Works for any integral type.
#[macro_export]
macro_rules! branchless_max {
  ($x:ident, $y:ident, $u:ty) => {
    $x ^ (($x ^ $y) & (<$u>::wrapping_neg(($x < $y) as $u)))
  };
}

/// Wrap the inner value to a minimum of 2.
///
/// This is for alignment shenanigans, you're not expected to use it in a
/// struct, more just in function arguments and such.
#[derive(Debug, Clone, Copy)]
#[repr(align(2))]
pub struct Align2<T>(pub T);

/// Wrap the inner value to a minimum of 4.
///
/// This is for alignment shenanigans, you're not expected to use it in a
/// struct, more just in function arguments and such.
#[derive(Debug, Clone, Copy)]
#[repr(align(4))]
pub struct Align4<T>(pub T);

/// Wrap the inner value to a minimum of 8.
///
/// This is for alignment shenanigans, you're not expected to use it in a
/// struct, more just in function arguments and such.
#[derive(Debug, Clone, Copy)]
#[repr(align(8))]
pub struct Align8<T>(pub T);

/// Wrap the inner value to a minimum of 16.
///
/// This is for alignment shenanigans, you're not expected to use it in a
/// struct, more just in function arguments and such.
#[derive(Debug, Clone, Copy)]
#[repr(align(16))]
pub struct Align16<T>(pub T);

/// Wrap the inner value to a minimum of 32.
///
/// This is for alignment shenanigans, you're not expected to use it in a
/// struct, more just in function arguments and such.
#[derive(Debug, Clone, Copy)]
#[repr(align(32))]
pub struct Align32<T>(pub T);

/// Re-interprets `&T` as `&[u8]`.
///
/// Any ZST becomes an empty slice, and in that case the pointer value of that
/// empty slice might not match the pointer value of the input reference.
pub fn bytes_of<T: Pod>(t: &T) -> &[u8] {
  try_cast_slice::<T, u8>(core::slice::from_ref(t)).unwrap_or(&[])
}

/// Re-interprets `&mut T` as `&mut [u8]`.
///
/// Any ZST becomes an empty slice, and in that case the pointer value of that
/// empty slice might not match the pointer value of the input reference.
pub fn bytes_of_mut<T: Pod>(t: &mut T) -> &mut [u8] {
  try_cast_slice_mut::<T, u8>(core::slice::from_mut(t)).unwrap_or(&mut [])
}

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

/// As [try_cast_slice](try_cast_slice), but unwraps the result for you.
pub fn cast_slice<A: Pod, B: Pod>(a: &[A]) -> &[B] {
  try_cast_slice(a).unwrap()
}

/// As [try_cast_slice_mut](try_cast_slice_mut), but unwraps the result for you.
pub fn cast_slice_mut<A: Pod, B: Pod>(a: &[A]) -> &[B] {
  try_cast_slice(a).unwrap()
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
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a.as_ptr() as usize) % align_of::<B>() != 0 {
    Err(SliceCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, a.len()) })
  } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
    Err(SliceCastError::CantConvertBetweenZSTAndNonZST)
  } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
    let new_len = core::mem::size_of_val(a) / size_of::<B>();
    Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, new_len) })
  } else {
    Err(SliceCastError::OutputSliceWouldHaveSlop)
  }
}

/// As [try_cast_slice](try_cast_slice), but `mut`.
pub fn try_cast_slice_mut<A: Pod, B: Pod>(a: &mut [A]) -> Result<&mut [B], SliceCastError> {
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a.as_ptr() as usize) % align_of::<B>() != 0 {
    Err(SliceCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe { core::slice::from_raw_parts_mut(a.as_ptr() as *mut B, a.len()) })
  } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
    Err(SliceCastError::CantConvertBetweenZSTAndNonZST)
  } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
    let new_len = core::mem::size_of_val(a) / size_of::<B>();
    Ok(unsafe { core::slice::from_raw_parts_mut(a.as_ptr() as *mut B, new_len) })
  } else {
    Err(SliceCastError::OutputSliceWouldHaveSlop)
  }
}
