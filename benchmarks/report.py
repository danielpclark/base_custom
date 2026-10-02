#!/usr/bin/env python3
"""Summarise base_custom `cargo bench` results from target/criterion as a markdown report.

Run from the benchmarks directory after `cargo bench`:

    python3 report.py > RESULTS.md
"""
import json
import pathlib
import platform
import subprocess
from collections import defaultdict

ROOT = pathlib.Path(__file__).resolve().parent / "target" / "criterion"
OLD, NEW = "0.2.0", "0.2.1"

DESCRIPTIONS = {
    "new": "building a base: `char` alphabets of 10 and 95 units, 12 delimited `String` units, all 256 bytes",
    "gen": "`gen` of a value (`u64::MAX` unless named)",
    "decimal": "`decimal` of the same value written in the base",
}


def fmt(ns):
    for unit, scale in (("s", 1e9), ("ms", 1e6), ("µs", 1e3)):
        if ns >= scale:
            return f"{ns / scale:.3g} {unit}"
    return f"{ns:.3g} ns"


def fmt_ratio(r):
    if r >= 100:
        return f"{r:,.0f}×"
    return f"{r:.3g}×"


def load():
    results = defaultdict(dict)
    for bench in ROOT.glob("**/new/benchmark.json"):
        meta = json.loads(bench.read_text())
        estimates = json.loads((bench.parent / "estimates.json").read_text())
        median = estimates["median"]
        results[meta["group_id"]][(meta["function_id"], meta["value_str"])] = (
            median["point_estimate"],
            median["confidence_interval"]["lower_bound"],
            median["confidence_interval"]["upper_bound"],
        )
    return results


def size_key(size):
    try:
        return (0, int(size))
    except (TypeError, ValueError):
        return (1, str(size))


def machine():
    cpu = "unknown CPU"
    try:
        for line in open("/proc/cpuinfo"):
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    rustc = subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip()
    return f"{cpu}, {platform.system()} {platform.machine()}, {rustc}"


def main():
    results = load()
    print("# Benchmark results: base_custom 0.2.0 vs 0.2.1\n")
    print(f"Machine: {machine()}.  ")
    print("Criterion medians with 95% confidence intervals, release builds, the same inputs for both versions.\n")
    for group in DESCRIPTIONS:
        rows = results.get(group)
        if not rows:
            continue
        print(f"### {group}\n\n{DESCRIPTIONS[group]}\n")
        print(f"| input | {OLD} | {NEW} | speedup |")
        print("|---:|---:|---:|---:|")
        sizes = sorted({size for (_, size) in rows}, key=size_key)
        for size in sizes:
            old, new = rows.get((OLD, size)), rows.get((NEW, size))
            if not (old and new):
                continue
            print(
                f"| {size} | {fmt(old[0])} ({fmt(old[1])} – {fmt(old[2])}) "
                f"| {fmt(new[0])} ({fmt(new[1])} – {fmt(new[2])}) | {fmt_ratio(old[0] / new[0])} |"
            )
        print()


if __name__ == "__main__":
    main()
