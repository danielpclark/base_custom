# Changelog

## 0.2.1

A compatible release: code written for 0.2.0 builds and behaves the same, apart from the
fixes listed below.

* 0.2.0's own test suite passes unchanged against 0.2.1.
* A differential run of 1,046,114 cases against 0.2.0 found no differences in results.
* It still builds on Rust 1.30, the oldest compiler 0.2.0 supports, and no 0.2.0 signature
  changed.

It fixes inputs that hung, panicked or returned wrong values, adds a few methods, and is
faster.

### Fixed

* A base whose units were all repeats of one unit (`BaseCustom::<char>::new(vec!['a', 'a'])`,
  and the same for `u8` and delimited `String`) made `gen` loop forever, growing its output
  until memory ran out.  `gen` now panics for any value but zero.  Such bases still build,
  and reading and writing zero still works, as in 0.2.0.
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
1.97).  Every benchmark is faster except building a 10-character base, which is unchanged
within measurement noise.  Full tables with confidence intervals are in
[`benchmarks/RESULTS.md`](benchmarks/RESULTS.md).

| Operation | 0.2.0 | 0.2.1 | Speedup |
|---|---:|---:|---:|
| `char` `decimal` of `u64::MAX` (base 10) | 207 ns | 48.1 ns | 4.3× |
| `char` `decimal`, non-ASCII base 10 | 236 ns | 78.8 ns | 3.0× |
| `char` `gen` of `u64::MAX` (base 10) | 196 ns | 92 ns | 2.1× |
| `char` `gen` of 12345 (base 10) | 65.9 ns | 24.1 ns | 2.7× |
| `String` `decimal`, delimited | 987 ns | 216 ns | 4.6× |
| `String` `gen`, delimited | 1.41 µs | 189 ns | 7.4× |
| `u8` `decimal`, binary | 578 ns | 62.9 ns | 9.2× |
| `new`, 95 `char`s | 4.66 µs | 697 ns | 6.7× |
| `new`, 256 bytes | 26.6 µs | 1.65 µs | 16.1× |
| `new`, 10 `char`s | 217 ns | 210 ns | same (within noise) |

### Other

* Still builds on Rust 1.30, like 0.2.0 (checked in CI), and stays on edition 2015.
* GitHub Actions CI replaces Travis.  The nightly-only `#[bench]` benchmarks are replaced by
  Criterion benchmarks in `benchmarks/` that compare against the previous release.
* The README examples are compiled and run as tests.
