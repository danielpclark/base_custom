# Benchmark results: base_custom 0.2.0 vs 0.2.1

Machine: Intel(R) Xeon(R) Processor @ 2.80GHz, Linux x86_64, rustc 1.97.0 (2d8144b78 2026-07-07).  
Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.

### new

building a base: `char` alphabets of 10 and 95 units, 12 delimited `String` units, all 256 bytes

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char/10 | 289 ns (282 ns – 306 ns) | 218 ns (218 ns – 219 ns) | 1.32× |
| char/95 | 5.11 µs (5.1 µs – 5.12 µs) | 1.06 µs (1.04 µs – 1.07 µs) | 4.81× |
| string_delimited/12 | 2.73 µs (2.73 µs – 2.74 µs) | 1.86 µs (1.85 µs – 1.87 µs) | 1.47× |
| u8/256 | 27.4 µs (27.4 µs – 27.4 µs) | 2.38 µs (2.37 µs – 2.38 µs) | 11.5× |

### gen

`gen` of a value (`u64::MAX` unless named)

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 541 ns (537 ns – 543 ns) | 330 ns (329 ns – 330 ns) | 1.64× |
| char_decimal/12345 | 71.5 ns (71.4 ns – 71.6 ns) | 36 ns (35.4 ns – 36.9 ns) | 1.99× |
| char_decimal/u64_max | 210 ns (209 ns – 211 ns) | 108 ns (108 ns – 109 ns) | 1.94× |
| char_hex/u64_max | 138 ns (138 ns – 139 ns) | 87.9 ns (87.8 ns – 87.9 ns) | 1.57× |
| string/u64_max | 995 ns (994 ns – 996 ns) | 190 ns (190 ns – 191 ns) | 5.23× |
| string_delimited/u64_max | 1.79 µs (1.79 µs – 1.81 µs) | 222 ns (221 ns – 223 ns) | 8.08× |
| u8_binary/u64_max | 527 ns (518 ns – 536 ns) | 293 ns (292 ns – 293 ns) | 1.8× |

### decimal

`decimal` of the same value written in the base

| input | 0.2.0 | 0.2.1 | speedup |
|---:|---:|---:|---:|
| char_binary/u64_max | 1 µs (994 ns – 1 µs) | 148 ns (147 ns – 148 ns) | 6.77× |
| char_decimal/12345 | 83.2 ns (83 ns – 83.4 ns) | 26.5 ns (26.3 ns – 26.5 ns) | 3.15× |
| char_decimal/u64_max | 313 ns (308 ns – 323 ns) | 54.3 ns (54 ns – 54.5 ns) | 5.77× |
| char_hex/u64_max | 241 ns (240 ns – 241 ns) | 46.8 ns (46.7 ns – 47 ns) | 5.14× |
| char_non_ascii/u64_max | 324 ns (323 ns – 324 ns) | 109 ns (109 ns – 109 ns) | 2.98× |
| string/u64_max | 1.47 µs (1.47 µs – 1.47 µs) | 199 ns (197 ns – 203 ns) | 7.38× |
| string_delimited/u64_max | 1.56 µs (1.55 µs – 1.56 µs) | 260 ns (260 ns – 262 ns) | 5.98× |
| u8_binary/u64_max | 961 ns (960 ns – 962 ns) | 86.3 ns (86.3 ns – 86.4 ns) | 11.1× |

