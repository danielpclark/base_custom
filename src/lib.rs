// Copyright 2017 Daniel P. Clark & base_custom Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![forbid(unsafe_code)]
#![deny(
  missing_docs,
  trivial_casts,
  trivial_numeric_casts,
  missing_debug_implementations,
  missing_copy_implementations,
  unstable_features,
  unused_import_braces,
  unused_qualifications
)]
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
//! base_custom = "0.2"
//! ```
//!
//! In your rust files where you plan to use it put this at the top
//!
//! ```text
//! use base_custom::BaseCustom;
//! ```
//!
//! ## Example
//!
//! ```
//! use base_custom::BaseCustom;
//!
//! let base3 = BaseCustom::<char>::new("ABC".chars().collect());
//! assert_eq!(base3.gen(123), "BBBCA");
//! assert_eq!(base3.decimal("BBBCA"), 123);
//!
//! // Look up single units, or parse without panicking.
//! assert_eq!(base3.position('C'), Some(2));
//! assert!(base3.try_decimal("ABX").is_err());
//! ```
//!
//! `BaseCustom` is `Send + Sync`, so a base can be shared between threads.
//!
//! This is licensed under MIT or APACHE 2.0 at your option.

// Compile and run the README examples as doctests.  The attribute sits in a
// macro so compilers older than 1.54, which cannot parse `include_str!` in an
// attribute, never see it (`cfg(doctest)` is only set by rustdoc).
#[cfg(doctest)]
macro_rules! readme_doctests {
  () => {
    #[doc = include_str!("../README.md")]
    pub struct ReadmeDoctests;
  };
}
#[cfg(doctest)]
readme_doctests!();

use std::error::Error;
use std::fmt;

/// The BaseCustom struct holds the information to perform number conversions
/// via the `gen` and `decimal` methods.
///
/// A new instance of BaseCustom can be created with either
///
/// * `BaseCustom::<char>::new(Vec<char>)`
/// * `BaseCustom::<char>::from_ordinal_range(Range)`
/// * `BaseCustom::<String>::new(String, Option<char>)`
/// * `BaseCustom::<u8>::new(&[u8])`
///
/// _If you are going to provide a delimiter you need to use the `<String>` implementation.
/// A delimiter is optional._
///
/// The primitives for BaseCustom get built from the provides characters or string groups
/// and conversion methods are available to use then.  String groupings will be single character
/// strings if no delimiter is given, otherwise they may be strings of any length split only
/// by the delimiter provided.
///
/// Repeated units are ignored after their first occurrence, so `"0011"` is binary.
#[derive(Clone)]
pub struct BaseCustom<T> {
  primitives: Vec<T>,
  primitives_hash: util::UnitMap<T>,
  /// The size of the base
  pub base: u64,
  delim: Option<char>,
  // Positions of ASCII chars (for `char`) or of every byte (for `u8`), so
  // the common lookups skip hashing.  Empty for `String`.
  table: Vec<Option<u8>>,
}

/// Why a value could not be read as a number by `try_decimal`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DecimalError {
  /// A unit is not part of the base.  `position` counts units from the
  /// start of the input, beginning at zero.
  UnknownUnit {
    /// Index of the offending unit in the input.
    position: usize,
  },
  /// The value is larger than `u64::MAX`.
  Overflow,
}

impl fmt::Display for DecimalError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DecimalError::UnknownUnit { position } => {
        write!(
          f,
          "unit at position {} is not part of this numeric base",
          position
        )
      }
      DecimalError::Overflow => write!(f, "value is too large for a u64"),
    }
  }
}

impl Error for DecimalError {}

mod char;
mod string;
mod u8;
mod util;
