extern crate base_custom;

// Regression tests for bugs fixed in 0.2.1.  Each failed (or hung) on 0.2.0.
use base_custom::{BaseCustom, DecimalError};

#[test]
fn one_unit_bases_still_build_and_gen_no_longer_hangs() {
  // As in 0.2.0, repeats count towards the two-unit minimum, so these build.
  let chars = BaseCustom::<char>::new(vec!['a', 'a']);
  let bytes = BaseCustom::<u8>::new(&[1, 1]);
  let strings = BaseCustom::<String>::new("x x x", Some(' '));
  assert_eq!((chars.base, bytes.base, strings.base), (1, 1, 1));
  // Reading and writing zero work, as before.
  assert_eq!(chars.decimal("aaa"), 0);
  assert_eq!(bytes.decimal(&[1, 1]), 0);
  assert_eq!(strings.decimal("x x"), 0);
  assert_eq!(chars.gen(0), "a");
  assert_eq!(bytes.gen(0), vec![1]);
  assert_eq!(strings.gen(0), "x");
}

#[test]
#[should_panic(expected = "1 cannot be written in a base of a single unit")]
fn one_unit_char_gen_panics_instead_of_hanging() {
  // 0.2.0 looped forever, growing its output until memory ran out
  BaseCustom::<char>::new(vec!['a', 'a']).gen(1);
}

#[test]
#[should_panic(expected = "cannot be written in a base of a single unit")]
fn one_unit_u8_gen_panics_instead_of_hanging() {
  BaseCustom::<u8>::new(&[1, 1]).gen(5);
}

#[test]
#[should_panic(expected = "Too few numeric units! Provide two or more.")]
fn fewer_than_two_units_still_panics() {
  BaseCustom::<char>::new(vec!['a']);
}

#[test]
#[should_panic(expected = "Too few numeric units! Provide two or more.")]
fn undelimited_string_repeats_still_count_once() {
  // 0.2.0 removed repeats before counting when there was no delimiter
  BaseCustom::<String>::new("aa", None);
}

#[test]
fn unit_limit_counts_distinct_units() {
  // 300 characters, 100 distinct: 0.2.0 checked the limit before removing duplicates.
  let chars: Vec<char> = (0..300u32)
    .map(|i| char::from_u32(65 + i % 100).unwrap())
    .collect();
  assert_eq!(BaseCustom::<char>::new(chars).base, 100);
}

#[test]
fn char_past_the_end_is_none() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  // 0.2.0 panicked with an index out of bounds
  assert_eq!(base10.char(10), None);
  assert_eq!(base10.char(usize::MAX), None);
}

#[test]
fn string_nth_zero_is_the_zero_unit() {
  let base = BaseCustom::<String>::new("0123456789", None);
  // 0.2.0 returned None
  assert_eq!(base.nth(0), Some("0"));
  assert_eq!(base.nth(9), Some("9"));
  assert_eq!(base.nth(10), None);
}

#[test]
fn delimited_duplicates_are_removed() {
  // 0.2.0 kept both "0"s, making a base of 3 where decimal("0") == 2
  let base = BaseCustom::<String>::new("0 1 0", Some(' '));
  assert_eq!(base.base, 2);
  assert_eq!(base.decimal("0"), 0);
  assert_eq!(base.gen(2), "1 0 ");
}

#[test]
fn decimal_allows_any_number_of_leading_zeros() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  // 0.2.0 overflowed computing 10^24 even though the value is 1
  assert_eq!(base10.decimal(format!("{}1", "0".repeat(100))), 1);
  let bytes = BaseCustom::<u8>::new(b"01");
  assert_eq!(bytes.decimal(&[b'0'; 200]), 0);
  let strings = BaseCustom::<String>::new("0 1", Some(' '));
  assert_eq!(strings.decimal(format!("{}1", "0 ".repeat(100))), 1);
}

#[test]
fn try_decimal_reports_overflow() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  assert_eq!(base10.try_decimal("18446744073709551615"), Ok(u64::MAX));
  // 0.2.0 wrapped silently in release builds and panicked in debug builds
  assert_eq!(
    base10.try_decimal("18446744073709551616"),
    Err(DecimalError::Overflow)
  );
}

#[test]
#[should_panic(expected = "value is too large for a u64")]
fn decimal_panics_clearly_on_overflow() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  base10.decimal("99999999999999999999");
}

#[test]
#[should_panic(expected = "unit at position 2 is not part of this numeric base")]
fn decimal_panics_clearly_on_unknown_units() {
  // 0.2.0 panicked with "no entry found for key"
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  base10.decimal("12x");
}

#[test]
fn non_ascii_characters_still_work() {
  let base = BaseCustom::<char>::new("零一二三四五六七八九".chars().collect());
  assert_eq!(base.gen(2024), "二零二四");
  assert_eq!(base.decimal("二零二四"), 2024);
  assert_eq!(base.position('九'), Some(9));
  assert_eq!(
    base.try_decimal("二0"),
    Err(DecimalError::UnknownUnit { position: 1 })
  );
}

#[test]
fn bases_are_send_and_sync() {
  fn assert_send_sync<T: Send + Sync>() {}
  assert_send_sync::<BaseCustom<char>>();
  assert_send_sync::<BaseCustom<String>>();
  assert_send_sync::<BaseCustom<u8>>();
}

#[test]
fn full_byte_alphabet() {
  let all: Vec<u8> = (0..=255).collect();
  let base = BaseCustom::<u8>::new(&all);
  assert_eq!(base.base, 256);
  assert_eq!(base.gen(u64::MAX), vec![255; 8]);
  assert_eq!(base.decimal(&[255; 8]), u64::MAX);
  assert_eq!(base.position(255), Some(255));
}
