#![feature(test)]
extern crate test;
use test::Bencher;
extern crate base_custom;
use base_custom::BaseCustom;

#[bench]
fn char_gen(b: &mut Bencher){
  let b10char = BaseCustom::<char>::from_ordinal_range(48..58);
  b.iter(|| {
    b10char.gen(18446744073709551615);
  })
}

#[bench]
fn char_decimal(b: &mut Bencher){
  let b10char = BaseCustom::<char>::from_ordinal_range(48..58);
  b.iter(|| {
    b10char.decimal("18446744073709551615");
  })
}

#[bench]
fn string_gen(b: &mut Bencher){
  let b10string = BaseCustom::<String>::new("0123456789", None);
  b.iter(|| {
    b10string.gen(18446744073709551615);
  })
}

#[bench]
fn string_decimal(b: &mut Bencher){
  let b10string = BaseCustom::<String>::new("0123456789", None);
  b.iter(|| {
    b10string.decimal("18446744073709551615");
  })
}

#[bench]
fn string_delim_gen(b: &mut Bencher){
  let b10stringdelim = BaseCustom::<String>::new("0.1.2.3.4.5.6.7.8.9", Some('.'));
  b.iter(|| {
    b10stringdelim.gen(18446744073709551615);
  })
}

#[bench]
fn string_delim_decimal(b: &mut Bencher){
  let b10stringdelim = BaseCustom::<String>::new("0.1.2.3.4.5.6.7.8.9", Some('.'));
  b.iter(|| {
    b10stringdelim.decimal("1.8.4.4.6.7.4.4.0.7.3.7.0.9.5.5.1.6.1.5");
  })
}
