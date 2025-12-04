use std::env;

fn max_joltage_n(digits: &[u32], n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    // Find the largest digit in the range that leaves enough digits for the rest
    // We can pick from index 0 to (len - n), inclusive
    let search_range = &digits[..=digits.len() - n];
    let max_digit = *search_range.iter().max().unwrap();

    // Find the first occurrence of this max digit in the search range
    let idx = search_range.iter().position(|&d| d == max_digit).unwrap();

    // Recursively find the rest from the suffix
    let rest = max_joltage_n(&digits[idx + 1..], n - 1);

    (max_digit as u64) * 10u64.pow((n - 1) as u32) + rest
}

fn max_joltage(line: &str, n: usize) -> u64 {
    let digits: Vec<u32> = line.chars().map(|c| c.to_digit(10).unwrap()).collect();
    max_joltage_n(&digits, n)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day_03 <input_file>");
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");

    let part1: u64 = input.lines().map(|line| max_joltage(line, 2)).sum();
    let part2: u64 = input.lines().map(|line| max_joltage(line, 12)).sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
