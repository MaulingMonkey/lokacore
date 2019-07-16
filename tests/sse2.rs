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
