extern crate base_custom;
use base_custom::BaseCustom;

#[test]
fn it_works_with_binary_for_char() {
  let base2 = BaseCustom::<char>::new(vec!['0','1']);
  assert_eq!(base2.decimal("00001"), 1_u32);
  assert_eq!(base2.decimal("100110101"), 309_u32);
  assert_eq!(base2.gen(340), "101010100");
  assert_eq!(base2.gen(0xF45), "111101000101");
  assert_eq!(base2.gen(0b111), "111");
}

#[test]
fn it_works_with_binary_for_string() {
  let base2 = BaseCustom::<String>::new("01", None);
  assert_eq!(base2.decimal("00001"), 1_u32);
  assert_eq!(base2.decimal("100110101"), 309_u32);
  assert_eq!(base2.gen(340), "101010100");
  assert_eq!(base2.gen(0xF45), "111101000101");
  assert_eq!(base2.gen(0b111), "111");
}

#[test]
fn it_works_with_trinary_for_char() {
  let base3 = BaseCustom::<char>::new("ABC".chars().collect());
  assert_eq!(base3.decimal("ABC"), 5);
  assert_eq!(base3.gen(123), "BBBCA");
}

#[test]
fn it_works_with_trinary_for_string() {
  let base3 = BaseCustom::<String>::new("ABC", None);
  assert_eq!(base3.decimal("ABC"), 5);
  assert_eq!(base3.gen(123), "BBBCA");
}  

#[test]
fn it_works_with_decimal_for_char() {
  let base10 = BaseCustom::<char>::new("0123456789".chars().collect());
  assert_eq!(base10.gen(123), "123");
  assert_eq!(base10.decimal("123"), 123);
}

#[test]
fn it_works_with_decimal_for_string() {
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

//  it "delim music" do
//    baseMusic = BaseCustom.new(%w[A A# B C C# D D# E F F# G G#], ' ')
//    baseMusic.base( (Math::PI * 100000000).to_i ).should eq("F F# B D# D A# D# F# ")
//    baseMusic.base( "F F# B D# D A# D# F# " ).should eq((Math::PI * 100000000).to_i)
//  end
//  
//  it "multi with delim" do
//    baseMND = BaseCustom.new(%w(aa bb cc), ':')
//    baseMND.base(12).should eq("bb:bb:aa:")
//    baseMND.base("bb:bb:aa:").should eq(12)
//  end
//  
//  it "multi in string with delim" do
//    baseMND = BaseCustom.new("aa:bb:cc", ':')
//    baseMND.base(12).should eq("bb:bb:aa:")
//    baseMND.base("bb:bb:aa:").should eq(12)
//  end

#[test]
fn it_works_with_special_characters() {
  let base_sc = BaseCustom::<char>::new("\n01\t".chars().collect());
  assert_eq!(base_sc.gen(12345), "\t\n\n\n\t10");
}
