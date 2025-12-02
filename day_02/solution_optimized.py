#!/usr/bin/env python3
"""Advent of Code 2025 Day 2 - O(n * poly(d)) solution"""

import sys
import time
from math import gcd


def get_divisors(n):
    divs = []
    i = 1
    while i * i <= n:
        if n % i == 0:
            if i < n:
                divs.append(i)
            if i != n // i and n // i < n:
                divs.append(n // i)
        i += 1
    return sorted(divs)


def first_periodic_geq(lo, num_digits, k):
    reps = num_digits // k
    min_prefix, max_prefix = 10 ** (k - 1), 10 ** k - 1
    lo_prefix = int(str(lo).zfill(num_digits)[:k])

    candidate = int(str(lo_prefix) * reps)
    if candidate >= lo:
        return candidate if lo_prefix >= min_prefix else int(str(min_prefix) * reps)

    lo_prefix += 1
    if lo_prefix > max_prefix:
        return None
    return int(str(max(lo_prefix, min_prefix)) * reps)


def last_periodic_leq(hi, num_digits, k):
    reps = num_digits // k
    min_prefix, max_prefix = 10 ** (k - 1), 10 ** k - 1
    hi_prefix = int(str(hi).zfill(num_digits)[:k])

    candidate = int(str(hi_prefix) * reps)
    if candidate <= hi:
        if min_prefix <= hi_prefix <= max_prefix:
            return candidate
        return int(str(max_prefix) * reps) if hi_prefix > max_prefix else None

    hi_prefix -= 1
    return None if hi_prefix < min_prefix else int(str(hi_prefix) * reps)


def count_periodic_in_range(lo, hi, num_digits, k):
    first = first_periodic_geq(lo, num_digits, k)
    if first is None or first > hi:
        return 0, 0

    last = last_periodic_leq(hi, num_digits, k)
    if last is None or last < lo or first > last:
        return 0, 0

    reps = num_digits // k
    first_prefix, last_prefix = int(str(first)[:k]), int(str(last)[:k])
    count = last_prefix - first_prefix + 1

    multiplier = (10 ** (reps * k) - 1) // (10 ** k - 1)
    prefix_sum = (first_prefix + last_prefix) * count // 2

    return count, prefix_sum * multiplier


def sum_invalid_in_fixed_digit_range(lo, hi, num_digits):
    divs = get_divisors(num_digits)
    if not divs:
        return 0

    periodic = {k: count_periodic_in_range(lo, hi, num_digits, k) for k in divs}
    minimal_cnt, minimal_sum = {}, {}

    for k in divs:
        cnt, s = periodic[k]
        for q in divs:
            if q < k and k % q == 0:
                cnt -= minimal_cnt[q]
                s -= minimal_sum[q]
        minimal_cnt[k], minimal_sum[k] = cnt, s

    return sum(minimal_sum.values())


def split_range_by_digits(lo, hi):
    ranges = []
    current = lo
    while current <= hi:
        num_digits = len(str(current))
        range_end = min(hi, 10 ** num_digits - 1)
        ranges.append((current, range_end, num_digits))
        current = range_end + 1
    return ranges


def solve(input_str):
    ranges = []
    for part in input_str.strip().rstrip(',').split(','):
        if '-' in part:
            a, b = part.strip().split('-')
            ranges.append((int(a), int(b)))

    return sum(
        sum_invalid_in_fixed_digit_range(sub_lo, sub_hi, d)
        for lo, hi in ranges
        for sub_lo, sub_hi, d in split_range_by_digits(lo, hi)
    )


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python solution_optimized.py <input_file>")
        sys.exit(1)

    start = time.perf_counter()
    with open(sys.argv[1]) as f:
        result = solve(f.read())
    elapsed = (time.perf_counter() - start) * 1000

    print(result)
    print(f"{elapsed:.3f} ms", file=sys.stderr)
