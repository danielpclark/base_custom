# Changelog

## 0.2.1

A compatible release.  Apart from the fixes listed below, every call gives the same result
as in 0.2.0: a differential run of 1,046,002 cases against 0.2.0 found no differences.  It
fixes inputs that hung, panicked or returned wrong values, adds a few methods, and is faster.

### Fixed

* Repeated units could leave a base with a single unit, whose `gen` then looped forever
  (`BaseCustom::<char>::new(vec!['a', 'a'])`, and the same for `u8` and `String`).  Such bases
  now panic with "Too few numeric units!" like any other base with fewer than 2 units.
* The 255-unit limit was checked before repeated units were removed, so long alphabets with
  repeats were rejected.
* `char(n)` panicked for `n == base` instead of returning `None`.
* `BaseCustom::<String>::nth(0)` returned `None` instead of the zero unit.
* Delimited `String` bases kept repeated units, so `"0 1 0"` made a base of 3 in which
  `decimal("0")` was 2.  Repeats are now ignored as they are without a delimiter.
* `decimal` overflowed on leading zeros (24 zeros before a 1 in base 10), and wrapped
  silently in release builds on values past `u64::MAX`.  It now handles any number of
  leading zeros and panics with "value is too large for a u64" on real overflow.
* An unknown unit made `decimal` panic with "no entry found for key"; the message now says
  which unit is not part of the base.

### Added

* `try_decimal`, which returns `Result<u64, DecimalError>` instead of panicking, for all
  three unit types.
* `position`, the reverse of `nth`: the place value of a single unit.
* `DecimalError`, describing an unknown unit (with its position) or an overflow.
* `Eq` for `BaseCustom<char>`, `BaseCustom<String>` and `BaseCustom<u8>`.

### Performance

`decimal` uses Horner's method instead of computing a power for every unit, `gen` writes
into a fixed buffer instead of inserting at the front of a string, `char` (ASCII) and `u8`
lookups use a table, and other lookups use a fast hash instead of SipHash (the keys are the
base's own units, so hash-flooding resistance buys nothing here).

Criterion medians on identical inputs, 0.2.0 against 0.2.1 (Intel Xeon @ 2.10GHz, rustc
1.97).  Every benchmark is faster; full tables with confidence intervals are in
[`benchmarks/RESULTS.md`](benchmarks/RESULTS.md).

| Operation | 0.2.0 | 0.2.1 | Speedup |
|---|---:|---:|---:|
| `char` `decimal` of `u64::MAX` (base 10) | 196 ns | 42.3 ns | 4.6× |
| `char` `decimal`, non-ASCII base 10 | 247 ns | 79.6 ns | 3.1× |
| `char` `gen` of `u64::MAX` (base 10) | 182 ns | 95.9 ns | 1.9× |
| `char` `gen` of 12345 (base 10) | 67.4 ns | 23.1 ns | 2.9× |
| `String` `decimal`, delimited | 1.13 µs | 225 ns | 5.0× |
| `String` `gen`, delimited | 1.45 µs | 202 ns | 7.2× |
| `u8` `decimal`, binary | 619 ns | 80.5 ns | 7.7× |
| `new`, 95 `char`s | 4.9 µs | 637 ns | 7.7× |
| `new`, 256 bytes | 21 µs | 1.63 µs | 12.9× |
| `new`, 10 `char`s (smallest gain) | 227 ns | 212 ns | 1.07× |

### Other

* Edition 2021; requires Rust 1.56 or newer.
* GitHub Actions CI replaces Travis.  The nightly-only `#[bench]` benchmarks are replaced by
  Criterion benchmarks in `benchmarks/` that compare against the previous release.
* The README examples are compiled and run as tests.
