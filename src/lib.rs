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

/// The BaseCustom struct holds the information to perform number conversions
/// via the `gen` and `decimal` methods.
///
/// A new instance of BaseCustom can be created with either
///
/// * `BaseCustom::<char>::new(Vec<char>)`
/// * `BaseCustom::<char>::from_ordinal_range(Range)`
/// * `BaseCustom::<String>::new(String, Option<char>)`
///
/// _If you are going to provide a delimiter you need to use the `<String>` implementation.
/// A delimiter is optional._
///
/// The primitives for BaseCustom get built from the provides characters or string groups
/// and conversion methods are available to use then.  String groupings will be single character
/// strings if no delimiter is given, otherwise they may be strings of any length split only
/// by the delimiter provided.
#[derive(Clone)]
pub struct BaseCustom<T> {
  primitives: Vec<T>,
  primitives_hash: HashMap<T, u8>,
  /// The size of the base
  pub base: u64,
  delim: Option<char>,
}

mod u8;
mod char;
mod string;
mod util;

pub use crate::u8::*;
pub use crate::char::*;
pub use crate::string::*;
