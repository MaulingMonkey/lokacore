//! Intrinsics for the [x86](https://en.wikipedia.org/wiki/X86) processor family.

/// As [_rdtsc](core::arch::x86::_rdtsc), just marked safe since it's always safe.
#[inline]
pub fn rdtsc() -> u64 {
  unsafe { core::arch::x86::_rdtsc() }
}
