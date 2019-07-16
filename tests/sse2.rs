#![cfg(target_feature = "sse")]
#![allow(bad_style)]

#[path = "intel-simd-help.rs"]
mod intel_simd_help;
use intel_simd_help::*;

#[test]
fn m128i_debug() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:?}", m), "m128i(-1)");
  assert_eq!(&format!("{:#?}", m), &format!("m128i({})", max));
  assert_eq!(&format!("{:1?}", m), "m128i(-1)");
  assert_eq!(&format!("{:#1?}", m), &format!("m128i({})", max));
  let max = core::u64::MAX;
  assert_eq!(&format!("{:2?}", m), "m128i(-1, -1)");
  assert_eq!(&format!("{:#2?}", m), &format!("m128i({}, {})", max, max));
  let max = core::u32::MAX;
  assert_eq!(&format!("{:4?}", m), "m128i(-1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#4?}", m),
    &format!("m128i({}, {}, {}, {})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8?}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#8?}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16?}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#16?}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_display() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:}", m), "m128i(-1)");
  assert_eq!(&format!("{:#}", m), &format!("m128i({})", max));
  assert_eq!(&format!("{:1}", m), "m128i(-1)");
  assert_eq!(&format!("{:#1}", m), &format!("m128i({})", max));
  let max = core::u64::MAX;
  assert_eq!(&format!("{:2}", m), "m128i(-1, -1)");
  assert_eq!(&format!("{:#2}", m), &format!("m128i({}, {})", max, max));
  let max = core::u32::MAX;
  assert_eq!(&format!("{:4}", m), "m128i(-1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#4}", m),
    &format!("m128i({}, {}, {}, {})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(&format!("{:8}", m), "m128i(-1, -1, -1, -1, -1, -1, -1, -1)");
  assert_eq!(
    &format!("{:#8}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16}", m),
    "m128i(-1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)"
  );
  assert_eq!(
    &format!("{:#16}", m),
    &format!(
      "m128i({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_binary() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:b}", m), &format!("m128i({:b})", max));
  assert_eq!(&format!("{:#b}", m), &format!("m128i({:#b})", max));
  assert_eq!(&format!("{:1b}", m), &format!("m128i({:b})", max));
  assert_eq!(&format!("{:#1b}", m), &format!("m128i({:#b})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2b}", m),
    &format!("m128i({:b}, {:b})", max, max)
  );
  assert_eq!(
    &format!("{:#2b}", m),
    &format!("m128i({:#b}, {:#b})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4b}", m),
    &format!("m128i({:b}, {:b}, {:b}, {:b})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4b}", m),
    &format!("m128i({:#b}, {:#b}, {:#b}, {:#b})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8b}", m),
    &format!(
      "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8b}", m),
    &format!(
      "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16b}", m),
    &format!(
      "m128i({:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b}, {:b})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16b}", m),
    &format!(
      "m128i({:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b}, {:#b})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_lower_hex() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:x}", m), &format!("m128i({:x})", max));
  assert_eq!(&format!("{:#x}", m), &format!("m128i({:#x})", max));
  assert_eq!(&format!("{:1x}", m), &format!("m128i({:x})", max));
  assert_eq!(&format!("{:#1x}", m), &format!("m128i({:#x})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2x}", m),
    &format!("m128i({:x}, {:x})", max, max)
  );
  assert_eq!(
    &format!("{:#2x}", m),
    &format!("m128i({:#x}, {:#x})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4x}", m),
    &format!("m128i({:x}, {:x}, {:x}, {:x})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4x}", m),
    &format!("m128i({:#x}, {:#x}, {:#x}, {:#x})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8x}", m),
    &format!(
      "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8x}", m),
    &format!(
      "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16x}", m),
    &format!(
      "m128i({:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x}, {:x})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16x}", m),
    &format!(
      "m128i({:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x}, {:#x})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_octal() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:o}", m), &format!("m128i({:o})", max));
  assert_eq!(&format!("{:#o}", m), &format!("m128i({:#o})", max));
  assert_eq!(&format!("{:1o}", m), &format!("m128i({:o})", max));
  assert_eq!(&format!("{:#1o}", m), &format!("m128i({:#o})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2o}", m),
    &format!("m128i({:o}, {:o})", max, max)
  );
  assert_eq!(
    &format!("{:#2o}", m),
    &format!("m128i({:#o}, {:#o})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4o}", m),
    &format!("m128i({:o}, {:o}, {:o}, {:o})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4o}", m),
    &format!("m128i({:#o}, {:#o}, {:#o}, {:#o})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8o}", m),
    &format!(
      "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8o}", m),
    &format!(
      "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16o}", m),
    &format!(
      "m128i({:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o}, {:o})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16o}", m),
    &format!(
      "m128i({:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o}, {:#o})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}

#[test]
fn m128i_upper_hex() {
  let m: m128i = cast(core::u128::MAX);
  let max = core::u128::MAX;
  assert_eq!(&format!("{:X}", m), &format!("m128i({:X})", max));
  assert_eq!(&format!("{:#X}", m), &format!("m128i({:#X})", max));
  assert_eq!(&format!("{:1X}", m), &format!("m128i({:X})", max));
  assert_eq!(&format!("{:#1X}", m), &format!("m128i({:#X})", max));
  let max = core::u64::MAX;
  assert_eq!(
    &format!("{:2X}", m),
    &format!("m128i({:X}, {:X})", max, max)
  );
  assert_eq!(
    &format!("{:#2X}", m),
    &format!("m128i({:#X}, {:#X})", max, max)
  );
  let max = core::u32::MAX;
  assert_eq!(
    &format!("{:4X}", m),
    &format!("m128i({:X}, {:X}, {:X}, {:X})", max, max, max, max)
  );
  assert_eq!(
    &format!("{:#4X}", m),
    &format!("m128i({:#X}, {:#X}, {:#X}, {:#X})", max, max, max, max)
  );
  let max = core::u16::MAX;
  assert_eq!(
    &format!("{:8X}", m),
    &format!(
      "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})",
      max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#8X}", m),
    &format!(
      "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})",
      max, max, max, max, max, max, max, max
    )
  );
  let max = core::u8::MAX;
  assert_eq!(
    &format!("{:16X}", m),
    &format!(
      "m128i({:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X}, {:X})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
  assert_eq!(
    &format!("{:#16X}", m),
    &format!(
      "m128i({:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X}, {:#X})",
      max, max, max, max, max, max, max, max, max, max, max, max, max, max, max, max
    )
  );
}
