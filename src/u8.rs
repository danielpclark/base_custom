use std::collections::HashMap;
use crate::util::unique;
use crate::BaseCustom;
use std::fmt;

impl BaseCustom<u8> {

  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<u8>` requires a `&[u8]` as its parameters and units
  /// for measuring the custom numeric base will only be one u8 long each.
  pub fn new(bytes: &[u8]) -> BaseCustom<u8> {
    if bytes.iter().count() < 2 { panic!("Too few numeric units! Provide two or more.") }
    let bytes = unique(bytes.to_vec());

    let mut mapped = HashMap::with_capacity(bytes.iter().count());
    for (i,b) in bytes.iter().enumerate() {
      mapped.insert(*b, i as u8);
    }

    let count = bytes.iter().count() as u64;

    BaseCustom::<u8> {
      primitives: bytes,
      primitives_hash: mapped,
      base: count,
      delim: None,
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
    if input_val == 0 {
      return vec![self.primitives[0]];
    }
    let mut number = input_val;
    let mut result = Vec::new();
    loop {
      if number == 0 { break };
      result.insert(0, self.primitives[(number % self.base) as usize]);
      number = number/self.base;
    };
    result
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
  pub fn decimal(&self, input_val: &[u8]) -> u64 {
    input_val.iter().rev().enumerate().fold(0, |sum, (i, byt)|
      sum + (self.primitives_hash[&byt] as u64) * self.base.pow(i as u32)
    )
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
    if pos < self.base as usize {
      Some(self.primitives[pos])
    } else {
      None
    }
  }
}

impl PartialEq for BaseCustom<u8> {
  fn eq(&self, other: &BaseCustom<u8>) -> bool {
    self.primitives == other.primitives &&
      self.base == other.base &&
      self.delim == other.delim
  }
}

impl fmt::Debug for BaseCustom<u8> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}
