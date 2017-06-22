use std::collections::HashMap;
use std::string::String;
// use std::cmp::Eq;
// use std::hash::Hash;
// use std::clone::Clone;
// use std::fmt::Display;

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
    let mut i = 0;
    let mut i_out = 0;
    let input_val = input_val.into();
    let mut rchars = input_val.chars().rev();
    loop {
      match rchars.next() {
        Some(chr) => {
          let place = self.base.pow(i) as u32;
          i_out += self.primitives_hash[&chr] * place;
          i += 1;
        },
        None => break,
      }
    }
    i_out
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
        Some((i,c)) => {
          mapped.insert(format!("{}", c), i as u32)
        },
        None => break,
      };
    }
    BaseCustom::<String> {
      primitives: strings.iter().map(|s| format!("{}", s)).collect(),
      primitives_hash: mapped,
      base: chars.len() as u32,
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
    let mut i = 0;
    let mut i_out = 0;
    let input_val = input_val.into();
    let strings: Vec<String> = match self.delim {
      Some(c) => input_val.split(c).map(|c| format!("{}", c)).collect(),
      None => input_val.chars().map(|c| format!("{}", c)).collect(),
    };
    let mut rchars = strings.iter().rev();
    loop {
      match rchars.next() {
        Some(chr) => {
          if self.delim != None {
            if chr == &format!("{}", self.delim.unwrap()) { continue }
          }
          let place = self.base.pow(i) as u32;
          i_out += self.primitives_hash[&chr[..]] * place;
          i += 1;
        },
        None => break,
      }
    }
    i_out
  }
}
