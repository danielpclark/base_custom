# base_custom
[![CI](https://github.com/danielpclark/base_custom/actions/workflows/ci.yml/badge.svg)](https://github.com/danielpclark/base_custom/actions/workflows/ci.yml)
[![crates.io version](https://img.shields.io/crates/v/base_custom.svg)](https://crates.io/crates/base_custom)
[![Documentation](https://docs.rs/base_custom/badge.svg)](https://docs.rs/base_custom)

Use any characters as your own numeric base and convert to and from decimal.  This can be taken advantage of in various ways:

* Mathematics: number conversion
* Brute force sequencing
* Rolling ciphers
* Moderate information concealment
* Other potential uses such as deriving music or art from numbers

_There is also a Ruby and Crystal implementation of this which this was based off of._

For numbers beyond `u64`, with arithmetic on them, see [digits](https://github.com/danielpclark/digits),
which is built on this crate.

### Installation

Add the following to your Cargo.toml file
```toml
[dependencies]
base_custom = "0.2"
```

and bring it into scope with

```rust
use base_custom::BaseCustom;
```

### Usage

```rust
use base_custom::BaseCustom;

// Binary with no delimiter
let base2 = BaseCustom::<char>::new("01".chars().collect());
assert_eq!(base2.decimal("00001"), 1_u64);
assert_eq!(base2.decimal("100110101"), 309_u64);
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

// Bytes work too
let base_bytes = BaseCustom::<u8>::new(b"01");
assert_eq!(base_bytes.gen(5), b"101");
```

When using `BaseCustom::<String>::new` the second parameter must be of `Option<char>` to
choose your optional delimiter.  With a delimiter, `gen` follows every unit with the delimiter
(except for zero, which is the zero unit alone) and `decimal` skips empty units, so
`"bb::bb::aa"` reads the same as `"bb:bb:aa:"`.

Repeated units are ignored after their first occurrence, so `"0011"` describes binary.  A base
takes at least 2 units, and at most 255 distinct ones for `char` and `String` (all 256 bytes
for `u8`).

### Looking up units and handling bad input

`decimal` panics on a unit that is not in the base or a value larger than `u64::MAX`.  Use
`try_decimal` to get an error instead, and `position` to look up a single unit (the reverse of
`nth`).

```rust
use base_custom::{BaseCustom, DecimalError};

let hex = BaseCustom::<char>::new("0123456789abcdef".chars().collect());
assert_eq!(hex.position('a'), Some(10));
assert_eq!(hex.nth(10), Some(&'a'));
assert_eq!(hex.position('g'), None);

assert_eq!(hex.try_decimal("ff"), Ok(255));
assert_eq!(hex.try_decimal("fg"), Err(DecimalError::UnknownUnit { position: 1 }));
assert_eq!(hex.try_decimal("1".repeat(17)), Err(DecimalError::Overflow));
```

`BaseCustom` is `Send + Sync`, so one base can be shared between threads.

### Benchmarks

The [`benchmarks`](benchmarks) directory compares this version with the previous release using
Criterion; see [`benchmarks/RESULTS.md`](benchmarks/RESULTS.md).  `BaseCustom<char>` and
`BaseCustom<u8>` are the fastest implementations.

## License

Licensed under either of

 * Apache License, Version 2.0, (http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([MIT-LICENSE](MIT-LICENSE) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
