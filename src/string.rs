use std::collections::HashMap;
use crate::util::unique;
use crate::BaseCustom;
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
  pub fn new<S>(chars: S, delim: Option<char>) -> BaseCustom<String> 
    where S: Into<String> {
    let chars = chars.into();
    let mut mapped = HashMap::with_capacity(chars.len());
    let strings: Vec<String> = match delim {
      Some(c) => chars.split(c).map(|c| format!("{}", c)).filter(|s| !s.is_empty()).collect(),
      None => unique(chars.chars().collect()).iter().map(|c| format!("{}", c)).filter(|s| !s.is_empty()).collect(),
    };
    if strings.iter().count() < 2 { panic!("Too few numeric units! Provide two or more.") }
    if strings.iter().count() > 255 { panic!("Too many numeric units!") }
    let mut enumerator = strings.iter().enumerate();
    loop {
      match enumerator.next() {
        Some((i,c)) => mapped.insert(format!("{}", c), i as u8),
        None => break,
      };
    }
    BaseCustom::<String> {
      primitives: strings.iter().map(|s| format!("{}", s)).collect(),
      primitives_hash: mapped,
      base: strings.len() as u64,
      delim: delim,
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
  /// let base2 = BaseCustom::<String>::new("01", None);
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
      if self.delim != None { result.insert(0, self.delim.unwrap()) };
      result = format!("{}{}", self.primitives[(number % self.base) as usize], result);
      number = number/self.base;
    };
    format!("{}", result)
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
  pub fn decimal<S>(&self, input_val: S) -> u64
    where S: Into<String> {
    let input_val = input_val.into();
    let strings: Vec<String> = match self.delim {
      Some(c) => input_val.split(c).filter(|c| !c.is_empty()).map(|c| format!("{}", c)).collect(),
      None => input_val.chars().map(|c| format!("{}", c)).collect(),
    };

    strings.iter().rev().enumerate().fold(0, |sum, (i, chr)|
      sum + (self.primitives_hash[&chr[..]] as u64) * self.base.pow(i as u32)
    )
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
    if pos > 0 && pos < self.base as usize {
      Some(&self.primitives[pos])
    } else {
      None
    }
  }
}

impl fmt::Debug for BaseCustom<String> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
  }
}

impl PartialEq for BaseCustom<String> {
  fn eq(&self, other: &BaseCustom<String>) -> bool {
    self.primitives == other.primitives &&
      self.base == other.base &&
      self.delim == other.delim
  }
}
