//! Architecture specific functionality, safe wrapped.
//!
//! I only attempt to cover _stable_ intrinsics from [core::arch].
//!
//! Note that architecture specific modules are only conditionally included
//! based on the `target_arch` setting at compile time, so the docs that you see
//! here will only show the module for the arch that the doc machine was
//! targeting (eg: `docs.rs` uses `x86_64`).
//!
//! Currently supported arches are:
//!
//! * `x86`
//! * `x86_64`
//!
//! Technically there are two stable intrinsics for `wasm32` but they're for
//! talking to the global allocator and I don't want to mess with that because I
//! don't have enough WASM experience to say what's safe or not about that.
//!
//! **Please Note:** This particular module just safe wraps each available
//! intrinsic as directly as possible. Trait implementations from [core::ops]
//! and [core::convert] are provided when appropriate. This is intended to be a
//! _minimal_ layer that just provides safety so that users don't have to use
//! `unsafe` all over the place. Higher level abstractions of various styles can
//! be built on top of this layer by other modules and other crates if you want.

use super::*;

#[cfg(target_arch = "x86")]
pub mod x86;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;
