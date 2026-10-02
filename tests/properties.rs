// Randomised round trips across bases and unit types, checked against the
// standard library's own radix formatting where it applies.
use base_custom::{BaseCustom, DecimalError};

/// Small deterministic xorshift generator so the tests need no dependencies.
struct Rng(u64);

impl Rng {
  fn next(&mut self) -> u64 {
    self.0 ^= self.0 << 13;
    self.0 ^= self.0 >> 7;
    self.0 ^= self.0 << 17;
    self.0
  }

  /// Values spread over every magnitude, not just near u64::MAX.
  fn value(&mut self) -> u64 {
    let bits = self.next() % 65;
    if bits == 64 {
      self.next()
    } else {
      self.next() & ((1u64 << bits) - 1)
    }
  }
}

#[test]
fn char_matches_std_radix_formatting() {
  let binary = BaseCustom::<char>::new("01".chars().collect());
  let octal = BaseCustom::<char>::new("01234567".chars().collect());
  let decimal = BaseCustom::<char>::new("0123456789".chars().collect());
  let hex = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  let mut rng = Rng(0x2545_F491_4F6C_DD1D);
  for _ in 0..20_000 {
    let v = rng.value();
    assert_eq!(binary.gen(v), format!("{:b}", v));
    assert_eq!(octal.gen(v), format!("{:o}", v));
    assert_eq!(decimal.gen(v), v.to_string());
    assert_eq!(hex.gen(v), format!("{:x}", v));
    assert_eq!(hex.decimal(format!("{:x}", v)), v);
    assert_eq!(binary.try_decimal(format!("{:b}", v)), Ok(v));
  }
}

#[test]
fn every_unit_type_round_trips_in_many_bases() {
  let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
  for size in 2..=255u32 {
    // Non-ASCII characters exercise the hashed lookup path too.
    let chars: Vec<char> = (0..size)
      .map(|i| char::from_u32(0x21 + i * 3).unwrap())
      .collect();
    let as_char = BaseCustom::<char>::new(chars.clone());
    let as_string = BaseCustom::<String>::new(chars.iter().collect::<String>(), None);
    let as_delimited = BaseCustom::<String>::new(
      chars
        .iter()
        .map(|c| format!("<{}>", c))
        .collect::<Vec<_>>()
        .join(","),
      Some(','),
    );
    let as_bytes =
      BaseCustom::<u8>::new(&(0..size).map(|i| (i * 7 % 256) as u8).collect::<Vec<_>>());
    assert_eq!(as_char.base, u64::from(size));
    assert_eq!(as_bytes.base, u64::from(size));

    for i in 0..size as usize {
      assert_eq!(as_char.position(*as_char.nth(i).unwrap()), Some(i));
      assert_eq!(as_string.position(as_string.nth(i).unwrap()), Some(i));
      assert_eq!(as_bytes.position(as_bytes.nth(i).unwrap()), Some(i));
    }

    for _ in 0..40 {
      let v = rng.value();
      assert_eq!(as_char.decimal(as_char.gen(v)), v);
      assert_eq!(as_string.decimal(as_string.gen(v)), v);
      assert_eq!(as_delimited.decimal(as_delimited.gen(v)), v);
      assert_eq!(as_bytes.decimal(&as_bytes.gen(v)), v);
      assert_eq!(as_char.gen(v), as_string.gen(v));
    }
    assert_eq!(as_char.gen(0), as_char.zero().to_string());
  }
}

#[test]
fn values_past_u64_max_overflow_in_every_type() {
  for size in 2..=40u32 {
    let chars: Vec<char> = (0..size)
      .map(|i| char::from_u32(0x30 + i).unwrap())
      .collect();
    let base = BaseCustom::<char>::new(chars);
    let top = base.nth(base.base as usize - 1).unwrap();
    // u64::MAX written in this base, followed by one more digit, cannot fit.
    let too_long = format!("{}{}", base.gen(u64::MAX), top);
    assert_eq!(base.try_decimal(&too_long), Err(DecimalError::Overflow));
    let max = base.gen(u64::MAX);
    assert_eq!(base.try_decimal(&max), Ok(u64::MAX));
  }
}
