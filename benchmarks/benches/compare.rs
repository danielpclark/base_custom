//! base_custom 0.2.0 (crates.io) against this checkout, on identical inputs.
//!
//! Every input is one 0.2.0 handles correctly (no overflow, no unknown units),
//! and each benchmark asserts both versions agree before timing.
use base_custom::BaseCustom as New;
use base_custom_v020::BaseCustom as Old;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use std::hint::black_box;
use std::time::Duration;

const OLD: &str = "0.2.0";
const NEW: &str = "0.2.1";

const DECIMAL: &str = "0123456789";
const HEX: &str = "0123456789abcdef";
const CJK: &str = "零一二三四五六七八九";
const MUSIC: &str = "A A# B C C# D D# E F F# G G#";

fn new_bases(c: &mut Criterion) {
  let mut group = c.benchmark_group("new");
  let printable: Vec<char> = (32u8..127).map(char::from).collect();
  for (label, chars) in [
    ("char/10", DECIMAL.chars().collect::<Vec<_>>()),
    ("char/95", printable),
  ] {
    group.bench_with_input(BenchmarkId::new(OLD, label), &chars, |b, chars| {
      b.iter(|| Old::<char>::new(black_box(chars.clone())))
    });
    group.bench_with_input(BenchmarkId::new(NEW, label), &chars, |b, chars| {
      b.iter(|| New::<char>::new(black_box(chars.clone())))
    });
  }
  group.bench_function(BenchmarkId::new(OLD, "string_delimited/12"), |b| {
    b.iter(|| Old::<String>::new(black_box(MUSIC), Some(' ')))
  });
  group.bench_function(BenchmarkId::new(NEW, "string_delimited/12"), |b| {
    b.iter(|| New::<String>::new(black_box(MUSIC), Some(' ')))
  });
  let bytes: Vec<u8> = (0..=255).collect();
  group.bench_function(BenchmarkId::new(OLD, "u8/256"), |b| {
    b.iter(|| Old::<u8>::new(black_box(&bytes)))
  });
  group.bench_function(BenchmarkId::new(NEW, "u8/256"), |b| {
    b.iter(|| New::<u8>::new(black_box(&bytes)))
  });
  group.finish();
}

fn gen(c: &mut Criterion) {
  let mut group = c.benchmark_group("gen");
  for (label, alphabet, value) in [
    ("char_decimal/u64_max", DECIMAL, u64::MAX),
    ("char_decimal/12345", DECIMAL, 12_345),
    ("char_binary/u64_max", "01", u64::MAX),
    ("char_hex/u64_max", HEX, u64::MAX),
  ] {
    let (old, new) = (
      Old::<char>::new(alphabet.chars().collect()),
      New::<char>::new(alphabet.chars().collect()),
    );
    assert_eq!(old.gen(value), new.gen(value));
    group.bench_function(BenchmarkId::new(OLD, label), |b| {
      b.iter(|| old.gen(black_box(value)))
    });
    group.bench_function(BenchmarkId::new(NEW, label), |b| {
      b.iter(|| new.gen(black_box(value)))
    });
  }

  let (old, new) = (
    Old::<String>::new(DECIMAL, None),
    New::<String>::new(DECIMAL, None),
  );
  assert_eq!(old.gen(u64::MAX), new.gen(u64::MAX));
  group.bench_function(BenchmarkId::new(OLD, "string/u64_max"), |b| {
    b.iter(|| old.gen(black_box(u64::MAX)))
  });
  group.bench_function(BenchmarkId::new(NEW, "string/u64_max"), |b| {
    b.iter(|| new.gen(black_box(u64::MAX)))
  });

  let (old, new) = (
    Old::<String>::new(MUSIC, Some(' ')),
    New::<String>::new(MUSIC, Some(' ')),
  );
  assert_eq!(old.gen(u64::MAX), new.gen(u64::MAX));
  group.bench_function(BenchmarkId::new(OLD, "string_delimited/u64_max"), |b| {
    b.iter(|| old.gen(black_box(u64::MAX)))
  });
  group.bench_function(BenchmarkId::new(NEW, "string_delimited/u64_max"), |b| {
    b.iter(|| new.gen(black_box(u64::MAX)))
  });

  let (old, new) = (Old::<u8>::new(b"01"), New::<u8>::new(b"01"));
  assert_eq!(old.gen(u64::MAX), new.gen(u64::MAX));
  group.bench_function(BenchmarkId::new(OLD, "u8_binary/u64_max"), |b| {
    b.iter(|| old.gen(black_box(u64::MAX)))
  });
  group.bench_function(BenchmarkId::new(NEW, "u8_binary/u64_max"), |b| {
    b.iter(|| new.gen(black_box(u64::MAX)))
  });
  group.finish();
}

fn decimal(c: &mut Criterion) {
  let mut group = c.benchmark_group("decimal");
  for (label, alphabet, input) in [
    ("char_decimal/u64_max", DECIMAL, u64::MAX.to_string()),
    ("char_decimal/12345", DECIMAL, "12345".to_string()),
    ("char_binary/u64_max", "01", "1".repeat(64)),
    ("char_hex/u64_max", HEX, "f".repeat(16)),
    (
      "char_non_ascii/u64_max",
      CJK,
      u64::MAX
        .to_string()
        .chars()
        .map(|d| CJK.chars().nth(d.to_digit(10).unwrap() as usize).unwrap())
        .collect(),
    ),
  ] {
    let (old, new) = (
      Old::<char>::new(alphabet.chars().collect()),
      New::<char>::new(alphabet.chars().collect()),
    );
    assert_eq!(old.decimal(input.as_str()), new.decimal(input.as_str()));
    group.bench_function(BenchmarkId::new(OLD, label), |b| {
      b.iter(|| old.decimal(black_box(input.as_str())))
    });
    group.bench_function(BenchmarkId::new(NEW, label), |b| {
      b.iter(|| new.decimal(black_box(input.as_str())))
    });
  }

  let input = u64::MAX.to_string();
  let (old, new) = (
    Old::<String>::new(DECIMAL, None),
    New::<String>::new(DECIMAL, None),
  );
  assert_eq!(old.decimal(input.as_str()), new.decimal(input.as_str()));
  group.bench_function(BenchmarkId::new(OLD, "string/u64_max"), |b| {
    b.iter(|| old.decimal(black_box(input.as_str())))
  });
  group.bench_function(BenchmarkId::new(NEW, "string/u64_max"), |b| {
    b.iter(|| new.decimal(black_box(input.as_str())))
  });

  let (old, new) = (
    Old::<String>::new(MUSIC, Some(' ')),
    New::<String>::new(MUSIC, Some(' ')),
  );
  let input = old.gen(u64::MAX);
  assert_eq!(old.decimal(input.as_str()), new.decimal(input.as_str()));
  group.bench_function(BenchmarkId::new(OLD, "string_delimited/u64_max"), |b| {
    b.iter(|| old.decimal(black_box(input.as_str())))
  });
  group.bench_function(BenchmarkId::new(NEW, "string_delimited/u64_max"), |b| {
    b.iter(|| new.decimal(black_box(input.as_str())))
  });

  let input = vec![b'1'; 64];
  let (old, new) = (Old::<u8>::new(b"01"), New::<u8>::new(b"01"));
  assert_eq!(old.decimal(&input), new.decimal(&input));
  group.bench_function(BenchmarkId::new(OLD, "u8_binary/u64_max"), |b| {
    b.iter(|| old.decimal(black_box(&input)))
  });
  group.bench_function(BenchmarkId::new(NEW, "u8_binary/u64_max"), |b| {
    b.iter(|| new.decimal(black_box(&input)))
  });
  group.finish();
}

fn config() -> Criterion {
  Criterion::default()
    .warm_up_time(Duration::from_secs(1))
    .measurement_time(Duration::from_secs(3))
}

criterion_group! {
  name = benches;
  config = config();
  targets = new_bases, gen, decimal
}
criterion_main!(benches);
