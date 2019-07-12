use super::*;

/// Implements an unsafe marker trait on an array type if the element type
/// also supports that marker trait.
///
/// Syntax:
///
/// ```txt
/// impl_unsafe_marker_for_array!(TraitName, 0, 1, 2, 3, ...);
/// ```
macro_rules! impl_unsafe_marker_for_array {
  ( $marker:ident , $( $n:expr ),* ) => {
    $(unsafe impl<T> $marker for [T; $n] where T: $marker {})*
  }
}

/// Marker trait for types that can be safely made with
/// [zeroed](core::mem::zeroed).
///
/// ## Safety
///
/// * Your type must be inhabited (eg: no
///   [Infallible](core::convert::Infallible)).
/// * Your type must be allowed to be an "all zeroes" bit pattern (eg: no
///   [`NonNull<T>`](core::ptr::NonNull)).
pub unsafe trait Zeroable: Sized {
  /// Calls [zeroed](core::mem::zeroed).
  ///
  /// This is a trait method so that you can write `MyType::zeroed()` in your
  /// code. It is a contract of this trait that if you implement it on your type
  /// you _must not_ override this method. In the future this trait will become
  /// a `#[marker]` trait so that this is absolute. Until then just don't do it.
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
unsafe impl<T> Zeroable for PhantomData<T> where T: Zeroable {}
unsafe impl<T> Zeroable for Align2<T> where T: Zeroable {}
unsafe impl<T> Zeroable for Align4<T> where T: Zeroable {}
unsafe impl<T> Zeroable for Align8<T> where T: Zeroable {}
unsafe impl<T> Zeroable for Align16<T> where T: Zeroable {}
unsafe impl<T> Zeroable for Align32<T> where T: Zeroable {}
impl_unsafe_marker_for_array!(
  Zeroable, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
  24, 25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024
);

/// Marker trait for "plain old data".
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
/// * A struct needs to be `repr(C)`, or a `repr(transparent)` wrapper around a
///   `Pod` type.
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
unsafe impl<T> Pod for PhantomData<T> where T: Pod {}
//
unsafe impl Pod for Align2<[u8; 2]> {}
unsafe impl Pod for Align2<[i8; 2]> {}
//
unsafe impl Pod for Align4<[u8; 4]> {}
unsafe impl Pod for Align4<[i8; 4]> {}
unsafe impl Pod for Align4<[u16; 2]> {}
unsafe impl Pod for Align4<[i16; 2]> {}
//
unsafe impl Pod for Align8<[u8; 8]> {}
unsafe impl Pod for Align8<[i8; 8]> {}
unsafe impl Pod for Align8<[u16; 4]> {}
unsafe impl Pod for Align8<[i16; 4]> {}
unsafe impl Pod for Align8<[u32; 2]> {}
unsafe impl Pod for Align8<[i32; 2]> {}
unsafe impl Pod for Align8<[f32; 2]> {}
//
unsafe impl Pod for Align16<[u8; 16]> {}
unsafe impl Pod for Align16<[i8; 16]> {}
unsafe impl Pod for Align16<[u16; 8]> {}
unsafe impl Pod for Align16<[i16; 8]> {}
unsafe impl Pod for Align16<[u32; 4]> {}
unsafe impl Pod for Align16<[i32; 4]> {}
unsafe impl Pod for Align16<[f32; 4]> {}
unsafe impl Pod for Align16<[u64; 2]> {}
unsafe impl Pod for Align16<[i64; 2]> {}
unsafe impl Pod for Align16<[f64; 2]> {}
unsafe impl Pod for Align16<u128> {}
unsafe impl Pod for Align16<i128> {}
//
unsafe impl Pod for Align32<[u8; 32]> {}
unsafe impl Pod for Align32<[i8; 32]> {}
unsafe impl Pod for Align32<[u16; 16]> {}
unsafe impl Pod for Align32<[i16; 16]> {}
unsafe impl Pod for Align32<[u32; 8]> {}
unsafe impl Pod for Align32<[i32; 8]> {}
unsafe impl Pod for Align32<[f32; 8]> {}
unsafe impl Pod for Align32<[u64; 4]> {}
unsafe impl Pod for Align32<[i64; 4]> {}
unsafe impl Pod for Align32<[f64; 4]> {}
unsafe impl Pod for Align32<[u128; 2]> {}
unsafe impl Pod for Align32<[i128; 2]> {}
//
#[cfg(target_pointer_width = "32")]
unsafe impl Pod for Align8<[usize; 2]> {}
#[cfg(target_pointer_width = "32")]
unsafe impl Pod for Align16<[usize; 4]> {}
#[cfg(target_pointer_width = "32")]
unsafe impl Pod for Align32<[usize; 8]> {}
//
#[cfg(target_pointer_width = "64")]
unsafe impl Pod for Align16<[usize; 2]> {}
#[cfg(target_pointer_width = "64")]
unsafe impl Pod for Align32<[usize; 4]> {}
//
impl_unsafe_marker_for_array!(
  Pod, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
  25, 26, 27, 28, 29, 30, 31, 32, 48, 64, 96, 128, 256, 512, 1024
);
