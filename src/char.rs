use crate::util::{
  check_unit_count, expect_decimal, fill_places, fold_places, index_units, small_table,
};
use crate::{BaseCustom, DecimalError};
use std::fmt;
use std::ops::Range;

impl BaseCustom<char> {
  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<char>` requires a `Vec<char>` as its parameters and units
  /// for measuring the custom numeric base will only be one character long each.
  ///
  /// Repeated characters are ignored after their first occurrence.  This panics
  /// if given fewer than 2 characters or more than 255 distinct ones.  As in
  /// 0.2.0, repeats count towards the minimum, so `vec!['a', 'a']` makes a base
  /// of one unit: it can only read and write zero, and `gen` panics for any
  /// other value.
  pub fn new(chars: Vec<char>) -> BaseCustom<char> {
    let given = chars.len();
    let (chars, mapped) = index_units(chars);
    check_unit_count(given, chars.len(), 255);
    let table = small_table(
      chars
        .iter()
        .enumerate()
        .map(|(i, &c)| (c as usize, i as u8)),
      128,
    );
    BaseCustom::<char> {
      base: chars.len() as u64,
      primitives: chars,
      primitives_hash: mapped,
      delim: None,
      table,
    }
  }

  /// `gen` returns a String computed from the character mapping and
  /// positional values the given u64 parameter evalutes to for your
  /// custom base
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<char>::new(vec!['0','1']);
  /// assert_eq!(base2.gen(3), "11");
  /// ```
  ///
  /// # Output
  /// ```text
  /// "11"
  /// ```
  pub fn gen(&self, input_val: u64) -> String {
    let mut buf = [0; 64];
    let places = fill_places(input_val, self.base, &mut buf);
    let mut result = String::with_capacity(places.len());
    for &place in places {
      result.push(self.primitives[usize::from(place)]);
    }
    result
  }

  /// `char` returns a char straight from the character mapping.
  /// decimal value must be within character range for a Some result.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  /// assert_eq!(base10.char(9), Some('9'));
  /// assert_eq!(base10.char(10), None);
  /// ```
  ///
  /// # Output
  /// ```text
  /// '9'
  /// ```
  pub fn char(&self, input_val: usize) -> Option<char> {
    self.primitives.get(input_val).cloned()
  }

  /// `decimal` returns a u64 value on computed from the units that form
  /// the custom base.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base2 = BaseCustom::<char>::new(vec!['0','1']);
  /// assert_eq!(base2.decimal("00011"), 3);
  /// ```
  ///
  /// # Output
  /// ```text
  /// 3
  /// ```
  ///
  /// _This panics if a character is not part of the base or the value does
  /// not fit in a `u64`.  Use `try_decimal` to handle those cases._
  pub fn decimal<S>(&self, input_val: S) -> u64
  where
    S: Into<String>,
  {
    expect_decimal(self.try_decimal(input_val.into()))
  }

  /// `try_decimal` is `decimal` returning an error, instead of panicking,
  /// for a character outside the base or a value larger than `u64::MAX`.
  ///
  /// # Example
  /// ```
  /// use base_custom::{BaseCustom, DecimalError};
  ///
  /// let base2 = BaseCustom::<char>::new(vec!['0','1']);
  /// assert_eq!(base2.try_decimal("00011"), Ok(3));
  /// assert_eq!(base2.try_decimal("0012"), Err(DecimalError::UnknownUnit { position: 3 }));
  /// assert_eq!(base2.try_decimal("1".repeat(65)), Err(DecimalError::Overflow));
  /// ```
  pub fn try_decimal<S>(&self, input_val: S) -> Result<u64, DecimalError>
  where
    S: AsRef<str>,
  {
    fold_places(
      input_val.as_ref().chars().map(|c| self.position_u8(c)),
      self.base,
    )
  }

  /// `position` returns the place value of a single character, the reverse
  /// of `nth`.
  ///
  /// # Example
  /// ```
  /// use base_custom::BaseCustom;
  ///
  /// let base16 = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
  /// assert_eq!(base16.position('a'), Some(10));
  /// assert_eq!(base16.position('g'), None);
  /// ```
  pub fn position(&self, unit: char) -> Option<usize> {
    self.position_u8(unit).map(usize::from)
  }

  fn position_u8(&self, unit: char) -> Option<u8> {
    match self.table.get(unit as usize) {
      Some(&place) => place,
      None => self.primitives_hash.get(&unit).cloned(),
    }
  }

  /// Returns the zero value of your custom base
  pub fn zero(&self) -> &char {
    &self.primitives[0]
  }

  /// Returns the one value of your custom base
  pub fn one(&self) -> &char {
    &self.primitives[1]
  }

  /// Returns the nth value of your custom base
  ///
  /// Like most indexing operations, the count starts from zero, so nth(0) returns the first value,
  /// nth(1) the second, and so on.
  pub fn nth(&self, pos: usize) -> Option<&char> {
    self.primitives.get(pos)
  }

  /// Create a custom numeric base from an ascii range of ordinal values
  ///
  /// This method currently restricts the ascii character range of the
  /// 95 typical characters starting from 32 and ending with 127.  If you'd
  /// like to use characters outside of this range please use the `new` method.
  pub fn from_ordinal_range(range: Range<u32>) -> BaseCustom<char> {
    let min = std::cmp::max(32, range.start);
    let max = std::cmp::min(127, range.end);
    let chars: Vec<char> = (min..max).filter_map(std::char::from_u32).collect();
    BaseCustom::<char>::new(chars)
  }
}

impl PartialEq for BaseCustom<char> {
  fn eq(&self, other: &BaseCustom<char>) -> bool {
    self.primitives == other.primitives && self.base == other.base && self.delim == other.delim
  }
}

impl Eq for BaseCustom<char> {}

impl fmt::Debug for BaseCustom<char> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}
