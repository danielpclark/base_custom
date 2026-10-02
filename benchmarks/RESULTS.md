# Benchmark results: base_custom 0.2.0 vs 0.2.1

Machine: Intel(R) Xeon(R) Processor @ 2.10GHz, Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07).  
Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.

### new

building a base: `char` alphabets of 10 and 95 units, 12 delimited `String` units, all 256 bytes

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char/10 | 227 ns (224 ns – 230 ns) | 212 ns (212 ns – 213 ns) | 1.07× |
| char/95 | 4.9 µs (4.87 µs – 4.94 µs) | 637 ns (625 ns – 645 ns) | 7.7× |
| string_delimited/12 | 2.23 µs (2.21 µs – 2.26 µs) | 1.06 µs (1.04 µs – 1.08 µs) | 2.11× |
| u8/256 | 21 µs (20.5 µs – 23.8 µs) | 1.63 µs (1.62 µs – 1.63 µs) | 12.9× |

### gen

`gen` of a value (`u64::MAX` unless named)

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 416 ns (414 ns – 420 ns) | 350 ns (346 ns – 355 ns) | 1.19× |
| char_decimal/12345 | 67.4 ns (67.3 ns – 67.5 ns) | 23.1 ns (23 ns – 23.4 ns) | 2.92× |
| char_decimal/u64_max | 182 ns (174 ns – 190 ns) | 95.9 ns (95.6 ns – 96.3 ns) | 1.89× |
| char_hex/u64_max | 130 ns (128 ns – 132 ns) | 67.1 ns (64.5 ns – 67.4 ns) | 1.94× |
| string/u64_max | 941 ns (938 ns – 948 ns) | 170 ns (170 ns – 171 ns) | 5.54× |
| string_delimited/u64_max | 1.45 µs (1.44 µs – 1.47 µs) | 202 ns (188 ns – 207 ns) | 7.2× |
| u8_binary/u64_max | 453 ns (434 ns – 514 ns) | 262 ns (256 ns – 277 ns) | 1.73× |

### decimal

`decimal` of the same value written in the base

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 630 ns (603 ns – 710 ns) | 119 ns (118 ns – 121 ns) | 5.3× |
| char_decimal/12345 | 63.8 ns (63.2 ns – 64.9 ns) | 24.8 ns (23.9 ns – 25.6 ns) | 2.58× |
| char_decimal/u64_max | 196 ns (191 ns – 200 ns) | 42.3 ns (41.9 ns – 43 ns) | 4.63× |
| char_hex/u64_max | 208 ns (207 ns – 210 ns) | 44.4 ns (41.3 ns – 45.4 ns) | 4.68× |
| char_non_ascii/u64_max | 247 ns (246 ns – 249 ns) | 79.6 ns (79.4 ns – 79.8 ns) | 3.1× |
| string/u64_max | 1.25 µs (1.23 µs – 1.26 µs) | 189 ns (188 ns – 190 ns) | 6.62× |
| string_delimited/u64_max | 1.13 µs (1.11 µs – 1.23 µs) | 225 ns (221 ns – 228 ns) | 5.03× |
| u8_binary/u64_max | 619 ns (602 ns – 638 ns) | 80.5 ns (80.4 ns – 80.6 ns) | 7.7× |

