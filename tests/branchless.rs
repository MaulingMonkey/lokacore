use lokacore::*;

#[test]
fn test_branchless_min_and_max() {
  for x in core::u8::MIN..=core::u8::MAX {
    for y in core::u8::MIN..=core::u8::MAX {
      assert_eq!(branchless_min!(x, y, u8), x.min(y));
      assert_eq!(branchless_max!(x, y, u8), x.max(y));
    }
  }
  for x in core::i8::MIN..=core::i8::MAX {
    for y in core::i8::MIN..=core::i8::MAX {
      assert_eq!(branchless_min!(x, y, i8), x.min(y));
      assert_eq!(branchless_max!(x, y, i8), x.max(y));
    }
  }
}

#[test]
fn test_branchless_abs() {
  for x in core::i8::MIN..=core::i8::MAX {
    assert_eq!(branchless_abs!(x, i8), x.wrapping_abs());
  }
}
