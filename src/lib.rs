// Copyright 2017 Daniel P. Clark & base_custom Developers
// 
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
use std::collections::HashMap;
use std::string::String;

pub struct BaseCustom<T> {
  primitives: Vec<T>,
  primitives_hash: HashMap<T, u32>,
  base: u32,
  delim: Option<char>,
}

impl BaseCustom<char> {
  pub fn new(chars: Vec<char>) -> BaseCustom<char> {
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

  pub fn gen(&self, input_val: u32) -> String {
    if input_val == 0 {
      return format!("{}", self.primitives[0]);
    }
    let mut number = input_val;
    let mut result = String::new();
    loop {
      if number == 0 { break };
      result.insert(0, self.primitives[(number % self.base) as usize]);
      number = (number/self.base) as u32;
    };
    format!("{}", result)
  }

  pub fn decimal<S>(&self, input_val: S) -> u32
    where S: Into<String> {
    let input_val = input_val.into();

    input_val.chars().rev().enumerate().fold(0, |sum, (i, chr)|
      sum + self.primitives_hash[&chr] * self.base.pow(i as u32)
    )
  }
}

impl BaseCustom<String> {
  pub fn new<S>(chars: S, delim: Option<char>) -> BaseCustom<String> 
    where S: Into<String> {
    let chars = chars.into();
    let mut mapped = HashMap::with_capacity(chars.len());
    let strings: Vec<String> = match delim {
      Some(c) => chars.split(c).map(|c| format!("{}", c)).collect(),
      None => chars.chars().map(|c| format!("{}", c)).collect(),
    };
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
      number = (number/self.base) as u32;
    };
    format!("{}", result)
  }

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
}
