use crate::util::{check_unit_count, expect_decimal, fold_places, index_units, places};
use crate::{BaseCustom, DecimalError};
use std::fmt;

impl BaseCustom<String> {
  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<String>` requires a `String` as its first parameter and units
  /// for measuring the custom numeric base can be one character long, or many in length.
  /// The second parameter is of `Option<char>` is a delimiter option for determining whether
  /// to split the string into single character length strings or possibly multiple length
  /// if the delimiter is partitioning the string in such a way.
  ///
  /// Empty units are skipped and repeated units are ignored after their first
  /// occurrence.  This panics unless 2 to 255 distinct units remain.
  pub fn new<S>(chars: S, delim: Option<char>) -> BaseCustom<String>
  where
    S: Into<String>,
  {
    let chars = chars.into();
    let units: Vec<String> = match delim {
      Some(c) => chars
        .split(c)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect(),
      None => chars.chars().map(String::from).collect(),
    };
    let (units, mapped) = index_units(units);
    check_unit_count(units.len(), 255);
    BaseCustom::<String> {
      base: units.len() as u64,
      primitives: units,
      primitives_hash: mapped,
      delim,
      table: Vec::new(),
    }
  }

  /// `gen` returns a String computed from the character mapping and
  /// positional values the given u64 parameter evalutes to for your
  /// custom base
  ///
  /// With a delimiter every unit is followed by the delimiter, except for
  /// zero which is the zero unit alone.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<String>::new("01", None);
  /// assert_eq!(base2.gen(3), "11");
  ///
  /// let notes = BaseCustom::<String>::new("do re mi", Some(' '));
  /// assert_eq!(notes.gen(5), "re mi ");
  /// assert_eq!(notes.gen(0), "do");
  /// ```
  ///
  /// # Output
  /// ```text
  /// "11"
  /// ```
  pub fn gen(&self, input_val: u64) -> String {
    if input_val == 0 {
      return self.primitives[0].clone();
    }
    let mut units: Vec<&str> = places(input_val, self.base)
      .map(|place| self.primitives[place].as_str())
      .collect();
    units.reverse();
    let mut result = String::new();
    for unit in units {
      result.push_str(unit);
      if let Some(delim) = self.delim {
        result.push(delim);
      }
    }
    result
  }

  /// `decimal` returns a u64 value on computed from the units that form
  /// the custom base.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<String>::new("01", None);
  /// assert_eq!(base2.decimal("00011"), 3);
  /// ```
  ///
  /// # Output
  /// ```text
  /// 3
  /// ```
  ///
  /// _This panics if a unit is not part of the base or the value does not
  /// fit in a `u64`.  Use `try_decimal` to handle those cases._
  pub fn decimal<S>(&self, input_val: S) -> u64
  where
    S: Into<String>,
  {
    expect_decimal(self.try_decimal(input_val.into()))
  }

  /// `try_decimal` is `decimal` returning an error, instead of panicking,
  /// for a unit outside the base or a value larger than `u64::MAX`.
  ///
  /// With a delimiter, empty units between repeated delimiters are skipped
  /// and do not count towards an error's position.
  ///
  /// # Example
  /// ```
  /// use base_custom::{BaseCustom, DecimalError};
  ///
  /// let notes = BaseCustom::<String>::new("do re mi", Some(' '));
  /// assert_eq!(notes.try_decimal("re mi"), Ok(5));
  /// assert_eq!(notes.try_decimal("re fa"), Err(DecimalError::UnknownUnit { position: 1 }));
  /// ```
  pub fn try_decimal<S>(&self, input_val: S) -> Result<u64, DecimalError>
  where
    S: AsRef<str>,
  {
    let input = input_val.as_ref();
    match self.delim {
      Some(c) => fold_places(
        input
          .split(c)
          .filter(|s| !s.is_empty())
          .map(|unit| self.position_u8(unit)),
        self.base,
      ),
      None => {
        let mut buf = [0u8; 4];
        fold_places(
          input
            .chars()
            .map(|c| self.position_u8(c.encode_utf8(&mut buf))),
          self.base,
        )
      }
    }
  }

  /// `position` returns the place value of a single unit, the reverse of `nth`.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let notes = BaseCustom::<String>::new("do re mi", Some(' '));
  /// assert_eq!(notes.position("mi"), Some(2));
  /// assert_eq!(notes.position("fa"), None);
  /// ```
  pub fn position(&self, unit: &str) -> Option<usize> {
    self.position_u8(unit).map(usize::from)
  }

  fn position_u8(&self, unit: &str) -> Option<u8> {
    self.primitives_hash.get(unit).copied()
  }

  /// Returns the zero value of your custom base
  pub fn zero(&self) -> &str {
    &self.primitives[0]
  }

  /// Returns the one value of your custom base
  pub fn one(&self) -> &str {
    &self.primitives[1]
  }

  /// Returns the nth value of your custom base
  ///
  /// Like most indexing operations, the count starts from zero, so nth(0) returns the first value,
  /// nth(1) the second, and so on.
  pub fn nth(&self, pos: usize) -> Option<&str> {
    self.primitives.get(pos).map(String::as_str)
  }
}

impl fmt::Debug for BaseCustom<String> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}

impl PartialEq for BaseCustom<String> {
  fn eq(&self, other: &BaseCustom<String>) -> bool {
    self.primitives == other.primitives && self.base == other.base && self.delim == other.delim
  }
}

impl Eq for BaseCustom<String> {}
