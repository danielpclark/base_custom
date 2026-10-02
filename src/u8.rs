use crate::util::{
  check_unit_count, expect_decimal, fill_places, fold_places, index_units, small_table,
};
use crate::{BaseCustom, DecimalError};
use std::fmt;

impl BaseCustom<u8> {
  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<u8>` requires a `&[u8]` as its parameters and units
  /// for measuring the custom numeric base will only be one u8 long each.
  ///
  /// Repeated bytes are ignored after their first occurrence.  This panics
  /// unless at least 2 distinct bytes remain.
  pub fn new(bytes: &[u8]) -> BaseCustom<u8> {
    let (bytes, mapped) = index_units(bytes.to_vec());
    check_unit_count(bytes.len(), 256);
    let table = small_table(
      bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| (usize::from(b), i as u8)),
      256,
    );
    BaseCustom::<u8> {
      base: bytes.len() as u64,
      primitives: bytes,
      primitives_hash: mapped,
      delim: None,
      table,
    }
  }

  /// `gen` returns a byte sequence computed from positional values
  /// the given u64 parameter evalutes to for your custom base
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<u8>::new(&[0x00, 0x01]);
  /// assert_eq!(base2.gen(3), vec![0x01, 0x01]);
  /// ```
  ///
  /// # Output
  /// ```text
  /// vec![0x01, 0x01]
  /// ```
  pub fn gen(&self, input_val: u64) -> Vec<u8> {
    let mut buf = [0; 64];
    fill_places(input_val, self.base, &mut buf)
      .iter()
      .map(|&place| self.primitives[usize::from(place)])
      .collect()
  }

  /// `decimal` returns a u64 value on computed from the units that form
  /// the custom base.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<u8>::new(b"01");
  /// assert_eq!(base2.decimal(b"00011"), 3);
  /// ```
  ///
  /// # Output
  /// ```text
  /// 3
  /// ```
  ///
  /// _This panics if a byte is not part of the base or the value does not
  /// fit in a `u64`.  Use `try_decimal` to handle those cases._
  pub fn decimal(&self, input_val: &[u8]) -> u64 {
    expect_decimal(self.try_decimal(input_val))
  }

  /// `try_decimal` is `decimal` returning an error, instead of panicking,
  /// for a byte outside the base or a value larger than `u64::MAX`.
  ///
  /// # Example
  /// ```
  /// use base_custom::{BaseCustom, DecimalError};
  ///
  /// let base2 = BaseCustom::<u8>::new(b"01");
  /// assert_eq!(base2.try_decimal(b"011"), Ok(3));
  /// assert_eq!(base2.try_decimal(b"012"), Err(DecimalError::UnknownUnit { position: 2 }));
  /// ```
  pub fn try_decimal(&self, input_val: &[u8]) -> Result<u64, DecimalError> {
    fold_places(
      input_val.iter().map(|&b| self.table[usize::from(b)]),
      self.base,
    )
  }

  /// `position` returns the place value of a single byte, the reverse of `nth`.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base3 = BaseCustom::<u8>::new(b"ABC");
  /// assert_eq!(base3.position(b'C'), Some(2));
  /// assert_eq!(base3.position(b'D'), None);
  /// ```
  pub fn position(&self, unit: u8) -> Option<usize> {
    self.table[usize::from(unit)].map(usize::from)
  }

  /// Returns the zero value of your custom base
  pub fn zero(&self) -> u8 {
    self.primitives[0]
  }

  /// Returns the one value of your custom base
  pub fn one(&self) -> u8 {
    self.primitives[1]
  }

  /// Returns the nth value of your custom base
  ///
  /// Like most indexing operations, the count starts from zero, so nth(0) returns the first value,
  /// nth(1) the second, and so on.
  pub fn nth(&self, pos: usize) -> Option<u8> {
    self.primitives.get(pos).copied()
  }
}

impl PartialEq for BaseCustom<u8> {
  fn eq(&self, other: &BaseCustom<u8>) -> bool {
    self.primitives == other.primitives && self.base == other.base && self.delim == other.delim
  }
}

impl Eq for BaseCustom<u8> {}

impl fmt::Debug for BaseCustom<u8> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}
