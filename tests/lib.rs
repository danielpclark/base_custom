extern crate base_custom;
use base_custom::BaseCustom;

#[test]
fn it_works_with_binary() {
  let base2 = BaseCustom::new(vec!['0','1']);
  assert_eq!(base2.decimal("00001"), 1_u32);
  assert_eq!(base2.decimal("100110101"), 309_u32);
  assert_eq!(base2.gen(340), "101010100");
  assert_eq!(base2.gen(0xF45), "111101000101");
  assert_eq!(base2.gen(0b111), "111");
}


#[test]
fn it_works_with_trinary() {
	let base3 = BaseCustom::new("ABC".chars().collect());
	assert_eq!(base3.decimal("ABC"), 5);
	assert_eq!(base3.gen(123), "BBBCA");
}  

#[test]
fn it_works_with_decimal() {
  let base10 = BaseCustom::new("0123456789".chars().collect());
  assert_eq!(base10.gen(123), "123");
  assert_eq!(base10.decimal("123"), 123);
}
//  it "delim" do
//    base = BaseCustom.new(["a", "bb", "ccc", "dddd"], ' ')
//    base.base( 20 ).should eq("bb bb a ")
//    base.base( "bb bb a " ).should eq(20)
//  end
//  
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
//
#[test]
fn it_works_with_special_characters() {
  let base_sc = BaseCustom::new("\n01\t".chars().collect());
  assert_eq!(base_sc.gen(12345), "\t\n\n\n\t10");
}
