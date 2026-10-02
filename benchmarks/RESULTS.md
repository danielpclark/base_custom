# Benchmark results: base_custom 0.2.0 vs 0.2.1

Machine: Intel(R) Xeon(R) Processor @ 2.10GHz, Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07).  
Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.

### new

building a base: `char` alphabets of 10 and 95 units, 12 delimited `String` units, all 256 bytes

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char/10 | 217 ns (201 ns – 228 ns) | 210 ns (209 ns – 212 ns) | 1.04× |
| char/95 | 4.66 µs (4.64 µs – 4.71 µs) | 697 ns (693 ns – 704 ns) | 6.68× |
| string_delimited/12 | 2.41 µs (2.4 µs – 2.43 µs) | 857 ns (849 ns – 871 ns) | 2.81× |
| u8/256 | 26.6 µs (26.3 µs – 26.7 µs) | 1.65 µs (1.64 µs – 1.66 µs) | 16.1× |

### gen

`gen` of a value (`u64::MAX` unless named)

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 500 ns (481 ns – 514 ns) | 330 ns (328 ns – 339 ns) | 1.51× |
| char_decimal/12345 | 65.9 ns (65.5 ns – 66.7 ns) | 24.1 ns (23.6 ns – 24.5 ns) | 2.74× |
| char_decimal/u64_max | 196 ns (192 ns – 202 ns) | 92 ns (88.3 ns – 94 ns) | 2.13× |
| char_hex/u64_max | 119 ns (116 ns – 121 ns) | 71 ns (68.7 ns – 71.5 ns) | 1.67× |
| string/u64_max | 891 ns (884 ns – 903 ns) | 170 ns (170 ns – 172 ns) | 5.23× |
| string_delimited/u64_max | 1.41 µs (1.39 µs – 1.42 µs) | 189 ns (186 ns – 194 ns) | 7.43× |
| u8_binary/u64_max | 480 ns (472 ns – 483 ns) | 328 ns (322 ns – 334 ns) | 1.46× |

### decimal

`decimal` of the same value written in the base

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 606 ns (588 ns – 636 ns) | 138 ns (136 ns – 140 ns) | 4.39× |
| char_decimal/12345 | 56.7 ns (56.2 ns – 57.5 ns) | 24.4 ns (23.9 ns – 25.1 ns) | 2.32× |
| char_decimal/u64_max | 207 ns (205 ns – 216 ns) | 48.1 ns (47.3 ns – 49.1 ns) | 4.31× |
| char_hex/u64_max | 180 ns (178 ns – 181 ns) | 42.3 ns (41.7 ns – 43.1 ns) | 4.24× |
| char_non_ascii/u64_max | 236 ns (209 ns – 243 ns) | 78.8 ns (76.4 ns – 79.5 ns) | 2.99× |
| string/u64_max | 1.12 µs (1.11 µs – 1.12 µs) | 167 ns (166 ns – 168 ns) | 6.68× |
| string_delimited/u64_max | 987 ns (975 ns – 1.01 µs) | 216 ns (214 ns – 219 ns) | 4.57× |
| u8_binary/u64_max | 578 ns (565 ns – 586 ns) | 62.9 ns (62.8 ns – 63.1 ns) | 9.19× |

