// Copyright 2017 Daniel P. Clark & base_custom Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![deny(missing_docs,trivial_casts,trivial_numeric_casts,
        missing_debug_implementations, missing_copy_implementations,
        unsafe_code,unstable_features,unused_import_braces,unused_qualifications)
]
//! # base_custom
//!
//! allows you to use any set of characters as your own numeric base and convert
//! to and from decimal.  This can be taken advantage of in various ways:
//!
//! * Mathematics: number conversion
//!
//! * Brute force sequencing
//!
//! * Rolling ciphers
//!
//! * Moderate information concealment
//!
//! * Other potential uses such as deriving music or art from numbers
//!
//! ## To Include It
//!
//! Add `base_custom` to your dependencies section of your `Cargo.toml` file.
//!
//! ```text
//! [dependencies]
//! base_custom = "^0.1"
//! ```
//!
//! In your rust files where you plan to use it put this at the top
//!
//! ```text
//! extern crate base_custom;
//! use base_custom::BaseCustom;
//! ```
//!
//! This is licensed under MIT or APACHE 2.0 at your option.

use std::collections::HashMap;
use std::string::String;
use std::fmt;

/// The BaseCustom struct holds the information to perform number conversions
/// via the `gen` and `decimal` methods.
///
/// A new instance of BaseCustom can be created with either
///
/// * `BaseCustom::<char>::new(Vec<char>)`
/// * `BaseCustom::<String>::new(String, Option<char>)`
///
/// _If you are going to provide a delimiter you need to use the `<String>` implementation.
/// A delimiter is optional._
///
/// The primitives for BaseCustom get built from the provides characters or string groups
/// and conversion methods are available to use then.  String groupings will be single character
/// strings if no delimiter is given, otherwise they may be strings of any length split only
/// by the delimiter provided.
pub struct BaseCustom<T> {
  primitives: Vec<T>,
  primitives_hash: HashMap<T, u32>,
  base: u32,
  delim: Option<char>,
}

impl fmt::Debug for BaseCustom<char> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f,
      "BaseCustom\n\tprimitives: {:?}\n\tprimitives_hash: {:?}\n\tbase: {}\n\tdelim: {:?}",
      self.primitives, self.primitives_hash, self.base, self.delim
    )
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

impl BaseCustom<char> {

  /// 'new' creates a new BaseCustom instance and propogates the values for converting
  /// numeric bases.
  ///
  /// `new` for `BaseCustom<char>` requires a `Vec<char>` as its parameters and units
  /// for measuring the custom numeric base will only be one character long each.
  pub fn new(chars: Vec<char>) -> BaseCustom<char> {
    if chars.iter().count() < 2 { panic!("Too few numeric units! Provide two or more.") }
    let mut mapped = HashMap::with_capacity(chars.iter().count());
    for (i,c) in chars.iter().enumerate() {
      mapped.insert(c.clone(), i as u32);
    }
    BaseCustom::<char> {
      primitives: chars.clone(),
      primitives_hash: mapped,
      base: chars.iter().count() as u32,
      delim: None,
    }
  }

  /// `gen` returns a String computed from the character mapping and 
  /// positional values the given u32 parameter evalutes to for your
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
  pub fn gen(&self, input_val: u32) -> String {
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


  /// `decimal` returns a u32 value on computed from the units that form
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
  pub fn decimal<S>(&self, input_val: S) -> u32
    where S: Into<String> {
    let input_val = input_val.into();

    input_val.chars().rev().enumerate().fold(0, |sum, (i, chr)|
      sum + self.primitives_hash[&chr] * self.base.pow(i as u32)
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
    if pos > 0 && pos < self.base as usize {
      Some(&self.primitives[pos])
    } else {
      None
    }
  }
}

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
      Some(c) => chars.split(c).map(|c| format!("{}", c)).collect(),
      None => chars.chars().map(|c| format!("{}", c)).collect(),
    };
    if strings.iter().count() < 2 { panic!("Too few numeric units! Provide two or more.") }
    let mut enumerator = strings.iter().enumerate();
    loop {
      match enumerator.next() {
        Some((i,c)) => mapped.insert(format!("{}", c), i as u32),
        None => break,
      };
    }
    BaseCustom::<String> {
      primitives: strings.iter().map(|s| format!("{}", s)).collect(),
      primitives_hash: mapped,
      base: strings.len() as u32,
      delim: delim,
    }
  }

  /// `gen` returns a String computed from the character mapping and 
  /// positional values the given u32 parameter evalutes to for your
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
  pub fn gen(&self, input_val: u32) -> String {
    if input_val == 0 {
      return format!("{}", self.primitives[0]);
    }
    let mut number = input_val;
    let mut result = String::new();
    loop {
      if number == 0 { break };
      if self.delim != None { result.insert(0, self.delim.unwrap()) };
      result.insert_str(0, &self.primitives[(number % self.base) as usize][..]);
      number = number/self.base;
    };
    format!("{}", result)
  }

  /// `decimal` returns a u32 value on computed from the units that form
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
  pub fn decimal<S>(&self, input_val: S) -> u32
    where S: Into<String> {
    let input_val = input_val.into();
    let strings: Vec<String> = match self.delim {
      Some(c) => input_val.split(c).filter(|c| !c.is_empty()).map(|c| format!("{}", c)).collect(),
      None => input_val.chars().map(|c| format!("{}", c)).collect(),
    };

    strings.iter().rev().enumerate().fold(0, |sum, (i, chr)|
      sum + self.primitives_hash[&chr[..]] * self.base.pow(i as u32)
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
