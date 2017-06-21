use std::collections::HashMap;

pub struct BaseCustom {
  primitives: Vec<char>,
  primitives_hash: HashMap<char, u32>,
  base: u32,
}

impl BaseCustom {
  pub fn new(chars: Vec<char>) -> BaseCustom {
    let mut mapped = HashMap::with_capacity(chars.iter().count());
    for (i,c) in chars.iter().enumerate() {
      mapped.insert(c.clone(), i as u32);
    }
    BaseCustom {
      primitives: chars.clone(),
      primitives_hash: mapped,
      base: chars.iter().count() as u32,
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

