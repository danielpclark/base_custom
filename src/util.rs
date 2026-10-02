use crate::DecimalError;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash, Hasher};

/// A small multiplicative hasher (the FxHash scheme) for unit lookups.
///
/// Keys only ever come from the base's own units, so the hash-flooding
/// resistance of the standard SipHash buys nothing here, while its cost
/// dominated lookups of non-ASCII characters and string units.
#[derive(Clone, Copy, Default)]
pub struct UnitHasher(u64);

impl UnitHasher {
  fn add(&mut self, word: u64) {
    self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
  }
}

impl Hasher for UnitHasher {
  fn write(&mut self, bytes: &[u8]) {
    for &byte in bytes {
      self.add(u64::from(byte));
    }
  }

  fn write_u8(&mut self, i: u8) {
    self.add(u64::from(i));
  }

  fn write_u32(&mut self, i: u32) {
    self.add(u64::from(i));
  }

  fn write_u64(&mut self, i: u64) {
    self.add(i);
  }

  fn write_usize(&mut self, i: usize) {
    self.add(i as u64);
  }

  fn finish(&self) -> u64 {
    self.0
  }
}

/// Map from a unit to its position.
pub type UnitMap<T> = HashMap<T, u8, BuildHasherDefault<UnitHasher>>;

/// Removes duplicate units, keeping the first occurrence of each, and maps
/// every remaining unit to its position.
pub fn index_units<T: Clone + Eq + Hash>(units: Vec<T>) -> (Vec<T>, UnitMap<T>) {
  let capacity = units.len().min(256);
  let mut unique = Vec::with_capacity(capacity);
  let mut positions = UnitMap::with_capacity_and_hasher(capacity, Default::default());
  for unit in units {
    if !positions.contains_key(&unit) {
      // Truncation only matters past 256 units, which `check_unit_count`
      // rejects before the map is used.
      positions.insert(unit.clone(), unique.len() as u8);
      unique.push(unit);
    }
  }
  (unique, positions)
}

/// Panics unless the number of units is usable (`max` inclusive).
pub fn check_unit_count(count: usize, max: usize) {
  if count < 2 {
    panic!("Too few numeric units! Provide two or more.")
  }
  if count > max {
    panic!("Too many numeric units!")
  }
}

/// A lookup table from small unit values (ASCII `char`s or any `u8`) to
/// their positions, which avoids hashing on the common path.
pub fn small_table(entries: impl Iterator<Item = (usize, u8)>, size: usize) -> Vec<Option<u8>> {
  let mut table = vec![None; size];
  for (unit, position) in entries {
    if unit < size {
      table[unit] = Some(position);
    }
  }
  table
}

/// Writes the place values of `value` in `base` into the end of `buf` and
/// returns them, most significant first.  Zero has a single place.  A u64
/// has at most 64 places (in binary), and every place fits in a u8 because a
/// base has at most 256 units.
pub fn fill_places(mut value: u64, base: u64, buf: &mut [u8; 64]) -> &[u8] {
  let mut start = buf.len();
  loop {
    start -= 1;
    buf[start] = (value % base) as u8;
    value /= base;
    if value == 0 {
      return &buf[start..];
    }
  }
}

/// Folds place values, most significant first, into a `u64` with Horner's
/// method.  `None` marks a unit that is not in the base.
pub fn fold_places(
  places: impl Iterator<Item = Option<u8>>,
  base: u64,
) -> Result<u64, DecimalError> {
  let mut total: u64 = 0;
  for (position, place) in places.enumerate() {
    let place = place.ok_or(DecimalError::UnknownUnit { position })?;
    total = total
      .checked_mul(base)
      .and_then(|t| t.checked_add(u64::from(place)))
      .ok_or(DecimalError::Overflow)?;
  }
  Ok(total)
}

/// The panicking form of a conversion result, for the `decimal` methods.
pub fn expect_decimal(result: Result<u64, DecimalError>) -> u64 {
  match result {
    Ok(value) => value,
    Err(error) => panic!("{}", error),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn places_of_zero_is_a_single_zero() {
    let mut buf = [0; 64];
    assert_eq!(fill_places(0, 10, &mut buf), &[0]);
    assert_eq!(fill_places(120, 10, &mut buf), &[1, 2, 0]);
    assert_eq!(fill_places(u64::MAX, 2, &mut buf).len(), 64);
    assert_eq!(fill_places(u64::MAX, 256, &mut buf), &[255; 8]);
  }

  #[test]
  fn fold_places_detects_overflow_but_not_leading_zeros() {
    let zeros = std::iter::repeat(Some(0)).take(100).chain(Some(Some(1)));
    assert_eq!(fold_places(zeros, 10), Ok(1));
    let too_big = std::iter::repeat(Some(9)).take(20);
    assert_eq!(fold_places(too_big, 10), Err(DecimalError::Overflow));
    let unknown = vec![Some(1), None].into_iter();
    assert_eq!(
      fold_places(unknown, 10),
      Err(DecimalError::UnknownUnit { position: 1 })
    );
  }

  #[test]
  fn index_units_keeps_first_occurrence() {
    let (units, positions) = index_units(vec!['b', 'a', 'b', 'c', 'a']);
    assert_eq!(units, vec!['b', 'a', 'c']);
    assert_eq!(positions[&'c'], 2);
  }
}
