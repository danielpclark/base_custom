extern crate base_custom;

#[cfg(test)]
mod u8 {
  use base_custom::BaseCustom;
  #[test]
  #[should_panic(expected = "Too few numeric units! Provide two or more.")]
  fn it_must_have_minimal_characters() {
    BaseCustom::<u8>::new(&[]);
    BaseCustom::<u8>::new(&[0x00]);
  }

  #[test]
  fn nth_should_return_none_for_out_of_bounds() {
    let base3 = BaseCustom::<u8>::new(b"ABC");
    assert_eq!(base3.nth(12), None);
  }

  #[test]
  fn nth_should_return_zero_for_zero() {
    let base3 = BaseCustom::<u8>::new(b"ABC");
    assert_eq!(base3.nth(0), Some(b'A'));
  }

  #[test]
  fn it_shows_zero_one_and_nth() {
    let base3 = BaseCustom::<u8>::new(b"ABC");
    assert_eq!(base3.zero(), b'A');
    assert_eq!(base3.one(), b'B');
    assert_eq!(base3.nth(2), Some(b'C'));
  }

  #[test]
  fn nth_works(){
    let b = BaseCustom::<u8>::new(
      &[0xE8,0xB0,0x88,0x6D,0x97,0x18,0x4F,0x1F,0x65,0xC7,0x67,0xF7,0x93,0x99,0x65,0xBF]
    );

    assert_eq!(b.nth(0), Some(0xE8));
  }

  #[test]
  fn it_works_with_binary() {
    let base2 = BaseCustom::<u8>::new(b"01");
    assert_eq!(base2.decimal(b"00001"), 1_u64);
    assert_eq!(base2.decimal(b"100110101"), 309_u64);
    assert_eq!(base2.gen(340), b"101010100");
    assert_eq!(base2.gen(0xF45), b"111101000101");
    assert_eq!(base2.gen(0b111), b"111");
  }

  #[test]
  fn it_works_with_binary_with_duplicates() {
    let base2 = BaseCustom::<u8>::new(b"00011");
    assert_eq!(base2.decimal(b"00001"), 1_u64);
    assert_eq!(base2.decimal(b"100110101"), 309_u64);
    assert_eq!(base2.gen(340), b"101010100");
    assert_eq!(base2.gen(0xF45), b"111101000101");
    assert_eq!(base2.gen(0b111), b"111");
  }

  #[test]
  fn it_works_with_trinary() {
    let base3 = BaseCustom::<u8>::new(b"ABC");
    assert_eq!(base3.decimal(b"ABC"), 5);
    assert_eq!(base3.gen(123), b"BBBCA");
  }

  #[test]
  fn it_works_with_decimal() {
    let base9 = BaseCustom::<u8>::new(b"0123456789");
    assert_eq!(base9.gen(123), b"123");
    assert_eq!(base9.decimal(b"123"), 123);
  }

  #[test]
  fn it_works_with_special_characters() {
    let base_sc = BaseCustom::<u8>::new(b"\n01\t");
    assert_eq!(base_sc.gen(12345), b"\t\n\n\n\t10");
  }

  #[test]
  fn it_implements_parital_equality() {
    let base3a = BaseCustom::<u8>::new(b"ABC");
    let base3b = BaseCustom::<u8>::new(b"ABC");
    assert_eq!(base3a == base3b, true);
  
    let base3e = BaseCustom::<u8>::new(b"ABC");
    let base3f = BaseCustom::<u8>::new(b"BCA");
    assert_eq!(base3e == base3f, false);
  }
}

#[cfg(test)]
mod char {
  use base_custom::BaseCustom;
  #[test]
  #[should_panic(expected = "Too few numeric units! Provide two or more.")]
  fn it_must_have_minimal_characters() {
    BaseCustom::<char>::new("".chars().collect());
    BaseCustom::<char>::new("0".chars().collect());
  }

  #[test]
  fn nth_should_return_none_for_out_of_bounds() {
    let base3 = BaseCustom::<char>::new("ABC".chars().collect());
    assert_eq!(base3.nth(12), None);
  }

  #[test]
  fn nth_should_return_zero_for_zero() {
    let base3 = BaseCustom::<char>::new("ABC".chars().collect());
    assert_eq!(base3.nth(0), Some(&'A'));
  }

  #[test]
  fn it_shows_zero_one_and_nth() {
    let base3 = BaseCustom::<char>::new("ABC".chars().collect());
    assert_eq!(base3.zero(), &'A');
    assert_eq!(base3.one(), &'B');
    assert_eq!(base3.nth(2), Some(&'C'));
  }

  #[test]
  fn it_works_with_binary() {
    let base2 = BaseCustom::<char>::new(vec!['0','1']);
    assert_eq!(base2.decimal("00001"), 1_u64);
    assert_eq!(base2.decimal("100110101"), 309_u64);
    assert_eq!(base2.gen(340), "101010100");
    assert_eq!(base2.gen(0xF45), "111101000101");
    assert_eq!(base2.gen(0b111), "111");
  }

  #[test]
  fn it_works_with_binary_with_duplicates() {
    let base2 = BaseCustom::<char>::new(vec!['0','0','0','1','1']);
    assert_eq!(base2.decimal("00001"), 1_u64);
    assert_eq!(base2.decimal("100110101"), 309_u64);
    assert_eq!(base2.gen(340), "101010100");
    assert_eq!(base2.gen(0xF45), "111101000101");
    assert_eq!(base2.gen(0b111), "111");
  }

  #[test]
  fn it_works_with_binary_from_ordinal_range() {
    let base2 = BaseCustom::<char>::from_ordinal_range(48..50);
    assert_eq!(base2.decimal("00001"), 1_u64);
    assert_eq!(base2.decimal("100110101"), 309_u64);
    assert_eq!(base2.gen(340), "101010100");
    assert_eq!(base2.gen(0xF45), "111101000101");
    assert_eq!(base2.gen(0b111), "111");
  }

  #[test]
  fn it_can_convert_base_10_on_u64_max() {
    let base10 = BaseCustom::<char>::from_ordinal_range(48..58);
    assert_eq!(base10.gen(18446744073709551615), "18446744073709551615");
    assert_eq!(base10.decimal("18446744073709551615"), 18446744073709551615);
  }

  #[test]
  fn it_works_with_binary_from_min_of_ordinal_range() {
    let base2 = BaseCustom::<char>::from_ordinal_range(0..34);
    assert_eq!(base2.decimal("!"), 1_u64);
    assert_eq!(base2.gen(340), "! ! ! !  ");
  }

  #[test]
  fn it_works_with_binary_from_max_of_ordinal_range() {
    let base2 = BaseCustom::<char>::from_ordinal_range(125..500);
    assert_eq!(base2.decimal("~"), 1_u64);
    assert_eq!(base2.gen(340), "~}~}~}~}}");
  }

  #[test]
  fn it_works_with_trinary() {
    let base3 = BaseCustom::<char>::new("ABC".chars().collect());
    assert_eq!(base3.decimal("ABC"), 5);
    assert_eq!(base3.gen(123), "BBBCA");
  }

  #[test]
  fn it_works_with_decimal() {
    let base9 = BaseCustom::<char>::new("0123456789".chars().collect());
    assert_eq!(base9.gen(123), "123");
    assert_eq!(base9.decimal("123"), 123);
  }

  #[test]
  fn it_works_with_special_characters() {
    let base_sc = BaseCustom::<char>::new("\n01\t".chars().collect());
    assert_eq!(base_sc.gen(12345), "\t\n\n\n\t10");
  }

  #[test]
  fn it_implements_parital_equality() {
    let base3a = BaseCustom::<char>::new("ABC".chars().collect());
    let base3b = BaseCustom::<char>::new("ABC".chars().collect());
    assert_eq!(base3a == base3b, true);
  
    let base3e = BaseCustom::<char>::new("ABC".chars().collect());
    let base3f = BaseCustom::<char>::new("BCA".chars().collect());
    assert_eq!(base3e == base3f, false);
  }

  #[test]
  fn it_gives_the_simple_char_from_char_mapping() {
    let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
    assert_eq!(base10.char(9), Some('9'));
  }

  #[test]
  fn it_can_provide_the_last_symbol_before_roll_over() {
    let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
    assert_eq!(base10.base, 10);
    assert_eq!(base10.char((base10.base - 1) as usize), Some('9'));
  }
}

#[cfg(test)]
mod string {
  use base_custom::BaseCustom;
  #[test]
  #[should_panic(expected = "Too few numeric units! Provide two or more.")]
  fn it_must_have_minimal_characters() {
    BaseCustom::<String>::new("", None);
    BaseCustom::<String>::new("0", None);
  }

  #[test]
  fn nth_should_return_none_for_out_of_bounds() {
    let base3 = BaseCustom::<String>::new("ABC", None);
    assert_eq!(base3.nth(12), None);
  }

  #[test]
  fn it_shows_zero_one_and_nth() {
    let base3 = BaseCustom::<String>::new("ABC", None);
    assert_eq!(base3.zero(), "A");
    assert_eq!(base3.one(), "B");
    assert_eq!(base3.nth(2), Some("C"));
  }

  #[test]
  fn it_can_convert_base_10_on_u64_max_with_delimiter() {
    let b10stringdelim = BaseCustom::<String>::new(".0.1.2.3.4.5.6.7.8.9..", Some('.'));
    assert_eq!(b10stringdelim.gen(184), "1.8.4.");
    assert_eq!(b10stringdelim.gen(18446744073709551615), "1.8.4.4.6.7.4.4.0.7.3.7.0.9.5.5.1.6.1.5.");
    assert_eq!(b10stringdelim.decimal("1.8.4.4.6.7.4.4.0.7.3.7.0.9.5.5.1.6.1.5"), 18446744073709551615);
  }

  #[test]
  fn it_works_with_binary() {
    let base2 = BaseCustom::<String>::new("01", None);
    assert_eq!(base2.decimal("00001"), 1_u64);
    assert_eq!(base2.decimal("100110101"), 309_u64);
    assert_eq!(base2.gen(340), "101010100");
    assert_eq!(base2.gen(0xF45), "111101000101");
    assert_eq!(base2.gen(0b111), "111");
  }

  #[test]
  fn it_works_with_binary_with_duplicates() {
    let base2 = BaseCustom::<String>::new("00011", None);
    assert_eq!(base2.decimal("00001"), 1_u64);
    assert_eq!(base2.decimal("100110101"), 309_u64);
    assert_eq!(base2.gen(340), "101010100");
    assert_eq!(base2.gen(0xF45), "111101000101");
    assert_eq!(base2.gen(0b111), "111");
  }

  #[test]
  fn it_works_with_trinary() {
    let base3 = BaseCustom::<String>::new("ABC", None);
    assert_eq!(base3.decimal("ABC"), 5);
    assert_eq!(base3.gen(123), "BBBCA");
  }

  #[test]
  fn it_works_with_decimal() {
    let base10 = BaseCustom::<String>::new("0123456789", None);
    assert_eq!(base10.gen(123), "123");
    assert_eq!(base10.decimal("123"), 123);
  }

  #[test]
  fn it_works_with_a_delimiter_gen() {
    let base = BaseCustom::<String>::new("a bb ccc dddd", Some(' '));
    assert_eq!(base.gen( 20 ), "bb bb a ");
  }

  #[test]
  fn it_works_with_a_delimiter_decimal() {
    let base = BaseCustom::<String>::new("a bb ccc dddd", Some(' '));
    assert_eq!(base.decimal( "bb bb a " ), 20);
  }

  #[test]
  fn it_works_with_music_and_a_delimiter() {
    let base_music = BaseCustom::<String>::new("A A# B C C# D D# E F F# G G#", Some(' '));
    assert_eq!(base_music.decimal("F F# B D# D A# D# F# "), 314159265);
    assert_eq!(base_music.gen(314159265), "F F# B D# D A# D# F# ");
  }

  #[test]
  fn it_works_with_non_space_delimiter() {
    let base_mnd = BaseCustom::<String>::new("aa:bb:cc", Some(':'));
    assert_eq!(base_mnd.gen(12), "bb:bb:aa:");
    assert_eq!(base_mnd.decimal("bb:bb:aa:"), 12);
    assert_eq!(base_mnd.decimal("bb::bb::aa"), 12);
  }

  #[test]
  fn it_implements_parital_equality() {
    let base3c = BaseCustom::<String>::new("ABC", None);
    let base3d = BaseCustom::<String>::new("ABC", None);
    assert_eq!(base3c == base3d, true);
  
    let base3g = BaseCustom::<String>::new("ABC", None);
    let base3h = BaseCustom::<String>::new("BCA", None);
    assert_eq!(base3g == base3h, false);
  }
}
