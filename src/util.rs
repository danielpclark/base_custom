use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hash, Hasher};
use crate::DecimalError;

/// A small multiplicative hasher (the FxHash scheme) for unit lookups.  Keys
/// only ever come from the base's own units, so SipHash's resistance to
/// collision attacks buys nothing here, while its cost dominated lookups.
#[derive(Clone, Copy, Default)]
pub struct UnitHasher(u64);

impl UnitHasher {
  fn add(&mut self, word: u64) {
    self.0 = (self.0.rotate_left(5) ^ word).wrapping_mul(0x517c_c1b7_2722_0a95);
  }
}

impl Hasher for UnitHasher {
  fn write(&mut self, bytes: &[u8]) {
    for &byte in bytes { self.add(u64::from(byte)); }
  }
  fn write_u8(&mut self, i: u8) { self.add(u64::from(i)); }
  fn write_u32(&mut self, i: u32) { self.add(u64::from(i)); }
  fn write_u64(&mut self, i: u64) { self.add(i); }
  fn write_usize(&mut self, i: usize) { self.add(i as u64); }
  fn finish(&self) -> u64 { self.0 }
}

/// Map from a unit to its position.
pub type UnitMap<T> = HashMap<T, u8, BuildHasherDefault<UnitHasher>>;

/// Removes repeated units, keeping the first occurrence of each, and maps
/// every remaining unit to its position.
pub fn index_units<T: Clone + Eq + Hash>(units: Vec<T>) -> (Vec<T>, UnitMap<T>) {
  let capacity = std::cmp::min(units.len(), 256);
  let mut unique = Vec::with_capacity(capacity);
  let mut positions = UnitMap::with_capacity_and_hasher(capacity, Default::default());
  for unit in units {
    if !positions.contains_key(&unit) {
      // Positions past 255 are truncated, but `new` rejects such bases.
      positions.insert(unit.clone(), unique.len() as u8);
      unique.push(unit);
    }
  }
  (unique, positions)
}

/// A table from small unit values (ASCII `char`s, or any `u8`) to positions.
pub fn small_table<I: Iterator<Item = (usize, u8)>>(entries: I, size: usize) -> Vec<Option<u8>> {
  let mut table = vec![None; size];
  for (unit, position) in entries {
    if unit < size { table[unit] = Some(position); }
  }
  table
}

/// Writes the place values of `value` into the end of `buf` and returns them,
/// most significant first.  Zero has a single place.
pub fn fill_places(mut value: u64, base: u64, buf: &mut [u8; 64]) -> &[u8] {
  // A base whose units were all repeats of one unit can only write zero.
  if base == 1 && value != 0 {
    panic!("{} cannot be written in a base of a single unit", value)
  }
  let mut start = buf.len();
  loop {
    start -= 1;
    buf[start] = (value % base) as u8;
    value /= base;
    if value == 0 { return &buf[start..]; }
  }
}

/// Folds place values, most significant first, into a `u64` (Horner's
/// method).  `None` marks a unit that is not in the base.
pub fn fold_places<I: Iterator<Item = Option<u8>>>(places: I, base: u64) -> Result<u64, DecimalError> {
  let mut total: u64 = 0;
  for (position, place) in places.enumerate() {
    let place = place.ok_or(DecimalError::UnknownUnit { position: position })?;
    total = total.checked_mul(base)
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
