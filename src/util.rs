use crate::DecimalError;
use std::collections::HashMap;
use std::hash::Hash;

/// Removes duplicate units, keeping the first occurrence of each, and maps
/// every remaining unit to its position.
pub fn index_units<T: Clone + Eq + Hash>(units: Vec<T>) -> (Vec<T>, HashMap<T, u8>) {
  let mut unique = Vec::with_capacity(units.len().min(256));
  let mut positions = HashMap::with_capacity(units.len().min(256));
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

/// The place values of `value` in `base`, least significant first.  Zero has
/// a single place.
pub fn places(mut value: u64, base: u64) -> impl Iterator<Item = usize> {
  let mut first = true;
  std::iter::from_fn(move || {
    if value == 0 && !first {
      return None;
    }
    first = false;
    let place = (value % base) as usize;
    value /= base;
    Some(place)
  })
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
    assert_eq!(places(0, 10).collect::<Vec<_>>(), vec![0]);
    assert_eq!(places(120, 10).collect::<Vec<_>>(), vec![0, 2, 1]);
    assert_eq!(places(u64::MAX, 2).count(), 64);
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
