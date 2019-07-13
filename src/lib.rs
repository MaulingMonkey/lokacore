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

/// `branchless_abs!(a, type)` gives the
/// [wrapping](https://doc.rust-lang.org/std/primitive.i8.html#method.wrapping_abs)
/// absolute value of any signed integer.
///
/// Branchless, so you can use it in a `const` context.
#[macro_export]
macro_rules! branchless_abs {
  ($x:expr, $u:ty) => {{
    let mask = $x >> ((core::mem::size_of::<$u>() * 8) - 1);
    ($x as $u).wrapping_add(mask) ^ mask
  }};
}

/// `branchless_min!(a, b, type)` gives the minimum of two integer values.
///
/// Branchless, so you can use it in a `const` context.
#[macro_export]
macro_rules! branchless_min {
  ($x:expr, $y:expr, $u:ty) => {
    $y ^ (($x ^ $y) & (($x < $y) as $u).wrapping_neg())
  };
}

/// `branchless_max!(a, b, type)` gives the maximum of two integer values.
///
/// Branchless, so you can use it in a `const` context.
#[macro_export]
macro_rules! branchless_max {
  ($x:expr, $y:expr, $u:ty) => {
    $x ^ (($x ^ $y) & (($x < $y) as $u).wrapping_neg())
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

/// The things that can go wrong when casting between [`Pod`] data forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PodCastError {
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
  /// When casting an individual `T`, `&T`, or `&mut T` value the source size
  /// and destination size must be an exact match.
  SizeMismatch,
}

/// As [`try_cast`], but unwraps the result for you.
pub fn cast<A: Pod, B: Pod>(a: A) -> B {
  try_cast(a).unwrap()
}

/// Try to cast `T` into `U`.
///
/// ## Failure
///
/// * If the types don't have the same size this fails.
pub fn try_cast<A: Pod, B: Pod>(a: A) -> Result<B, PodCastError> {
  if size_of::<A>() == size_of::<B>() {
    let mut b = B::zeroed();
    // Note(Lokathor): We copy in terms of `u8` because that allows us to bypass
    // any potential alignment difficulties.
    let ap = &a as *const A as *const u8;
    let bp = &mut b as *mut B as *mut u8;
    unsafe { core::intrinsics::copy_nonoverlapping(ap, bp, size_of::<A>()) }
    Ok(b)
  } else {
    Err(PodCastError::SizeMismatch)
  }
}

/// As [`try_cast_slice`], but unwraps the result for you.
pub fn cast_slice<A: Pod, B: Pod>(a: &[A]) -> &[B] {
  try_cast_slice(a).unwrap()
}

/// As [`try_cast_slice_mut`], but unwraps the result for you.
pub fn cast_slice_mut<A: Pod, B: Pod>(a: &[A]) -> &[B] {
  try_cast_slice(a).unwrap()
}

/// As [`try_cast_ref`], but unwraps the result for you.
pub fn cast_ref<A: Pod, B: Pod>(a: &A) -> &B {
  try_cast_ref(a).unwrap()
}

/// As [`try_cast_mut`], but unwraps the result for you.
pub fn cast_mut<A: Pod, B: Pod>(a: &mut A) -> &mut B {
  try_cast_mut(a).unwrap()
}

/// Try to convert a `&T` into `&U`.
///
/// ## Failure
///
/// * If the reference isn't aligned in the new type
/// * If the source type and target type aren't the same size.
pub fn try_cast_ref<A: Pod, B: Pod>(a: &A) -> Result<&B, PodCastError> {
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a as *const A as usize) % align_of::<B>() != 0 {
    Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe {
      (a as *const A as *const B)
        .as_ref()
        .unwrap_or_else(|| core::hint::unreachable_unchecked())
    })
  } else {
    Err(PodCastError::SizeMismatch)
  }
}

/// As [`try_cast_ref`], but `mut`.
pub fn try_cast_mut<A: Pod, B: Pod>(a: &mut A) -> Result<&mut B, PodCastError> {
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a as *mut A as usize) % align_of::<B>() != 0 {
    Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe {
      (a as *mut A as *mut B)
        .as_mut()
        .unwrap_or_else(|| core::hint::unreachable_unchecked())
    })
  } else {
    Err(PodCastError::SizeMismatch)
  }
}

/// Try to convert `&[T]` into `&[U]` (possibly with a change in length).
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
pub fn try_cast_slice<A: Pod, B: Pod>(a: &[A]) -> Result<&[B], PodCastError> {
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a.as_ptr() as usize) % align_of::<B>() != 0 {
    Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, a.len()) })
  } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
    Err(PodCastError::CantConvertBetweenZSTAndNonZST)
  } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
    let new_len = core::mem::size_of_val(a) / size_of::<B>();
    Ok(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, new_len) })
  } else {
    Err(PodCastError::OutputSliceWouldHaveSlop)
  }
}

/// As [`try_cast_slice`], but `mut`.
pub fn try_cast_slice_mut<A: Pod, B: Pod>(a: &mut [A]) -> Result<&mut [B], PodCastError> {
  // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
  // after monomorphization.
  if align_of::<B>() > align_of::<A>() && (a.as_ptr() as usize) % align_of::<B>() != 0 {
    Err(PodCastError::TargetAlignmentGreaterAndInputNotAligned)
  } else if size_of::<B>() == size_of::<A>() {
    Ok(unsafe { core::slice::from_raw_parts_mut(a.as_ptr() as *mut B, a.len()) })
  } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
    Err(PodCastError::CantConvertBetweenZSTAndNonZST)
  } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
    let new_len = core::mem::size_of_val(a) / size_of::<B>();
    Ok(unsafe { core::slice::from_raw_parts_mut(a.as_ptr() as *mut B, new_len) })
  } else {
    Err(PodCastError::OutputSliceWouldHaveSlop)
  }
}
