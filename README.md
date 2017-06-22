# base_custom
[![Build Status](https://travis-ci.org/danielpclark/base_custom.svg)](https://travis-ci.org/danielpclark/base_custom)
[![crates.io version](https://img.shields.io/crates/v/base_custom.svg)](https://crates.io/crates/base_custom)
[![License](https://img.shields.io/crates/l/base_custom.svg)]()

Use any characters as your own numeric base and convert to and from decimal.

_There is also a Ruby and Crystal implementation of this which this was based off of._

### Installation

Add the following to your Cargo.toml file
```toml
[dependencies]
base_custom = "^0.1"
```

To include it for usage add

```rust
extern crate base_custom;
use base_custom::BaseCustom;
```

to your file.

### Usage

```rust
// Binary with no delimiter
let base2 = BaseCustom::<char>::new("01".chars().collect());
assert_eq!(base2.decimal("00001"), 1_u32);
assert_eq!(base2.decimal("100110101"), 309_u32);
assert_eq!(base2.gen(340), "101010100");
assert_eq!(base2.gen(0xF45), "111101000101");
assert_eq!(base2.gen(0b111), "111");

// Trinary with no delimiter
let base3 = BaseCustom::<char>::new("ABC".chars().collect());
assert_eq!(base3.decimal("ABC"), 5);
assert_eq!(base3.gen(123), "BBBCA");

// Custom base like Musical Chords and a space delimiter
let base_music = BaseCustom::<String>::new("A A# B C C# D D# E F F# G G#", Some(' '));
assert_eq!(base_music.decimal("F F# B D# D A# D# F# "), 314159265);
assert_eq!(base_music.gen(314159265), "F F# B D# D A# D# F# ");
```

When using `BaseCustom::<String>::new` the second parameter must be of `Option<char>` to
choose your optional delimiter.


## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
