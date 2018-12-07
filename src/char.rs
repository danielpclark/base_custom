use std::collections::HashMap;
use crate::util::unique;
use crate::BaseCustom;
use std::ops::Range;
use std::fmt;

impl BaseCustom<char> {

  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<char>` requires a `Vec<char>` as its parameters and units
  /// for measuring the custom numeric base will only be one character long each.
  pub fn new(chars: Vec<char>) -> BaseCustom<char> {
    if chars.iter().count() < 2 { panic!("Too few numeric units! Provide two or more.") }
    if chars.iter().count() > 255 { panic!("Too many numeric units!") }

    let chars = unique(chars);

    let mut mapped = HashMap::with_capacity(chars.iter().count());
    for (i,c) in chars.iter().enumerate() {
      mapped.insert(c.clone(), i as u8);
    }
    BaseCustom::<char> {
      primitives: chars.clone(),
      primitives_hash: mapped,
      base: chars.iter().count() as u64,
      delim: None,
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
    if input_val == 0 {
      return format!("{}", self.primitives[0]);
    }
    let mut number = input_val;
    let mut result = String::new();
    loop {
      if number == 0 { break };
      result.insert(0, self.primitives[(number % self.base) as usize]);
      number = number/self.base;
    };
    format!("{}", result)
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
  /// ```
  ///
  /// # Output
  /// ```text
  /// '9'
  /// ```
  pub fn char(&self, input_val: usize) -> Option<char> {
    if input_val > self.primitives.len() { return None }
    Some(self.primitives[input_val])
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
  pub fn decimal<S>(&self, input_val: S) -> u64
    where S: Into<String> {
    let input_val = input_val.into();

    input_val.chars().rev().enumerate().fold(0, |sum, (i, chr)|
      sum + (self.primitives_hash[&chr] as u64) * self.base.pow(i as u32)
    )
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
    if pos < self.base as usize {
      Some(&self.primitives[pos])
    } else {
      None
    }
  }

  /// Create a custom numeric base from an ascii range of ordinal values
  ///
  /// This method currently restricts the ascii character range of the
  /// 95 typical characters starting from 32 and ending with 127.  If you'd
  /// like to use characters outside of this range please use the `new` method.
  pub fn from_ordinal_range(range: Range<u32>) -> BaseCustom<char> {
    let min = std::cmp::max(32, range.start);
    let max = std::cmp::min(127, range.end);
    let mut chars: Vec<char> = Vec::with_capacity(std::cmp::min(range.len(), 95));
    for chr in min..max {
      chars.push(std::char::from_u32(chr).unwrap());
    }
    BaseCustom::<char>::new(chars)
  }
}

impl PartialEq for BaseCustom<char> {
  fn eq(&self, other: &BaseCustom<char>) -> bool {
    self.primitives == other.primitives &&
      self.base == other.base &&
      self.delim == other.delim
  }
}

impl fmt::Debug for BaseCustom<char> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}
