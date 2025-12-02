use std::collections::HashSet;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day_02 <input_file>");
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");

    let mut part1: u64 = 0;
    let mut part2_ids: HashSet<u64> = HashSet::new();

    for range in input.trim().split(',') {
        if range.is_empty() {
            continue;
        }

        let parts: Vec<&str> = range.split('-').collect();
        let start: u64 = parts[0].parse().expect("Failed to parse start");
        let end: u64 = parts[1].parse().expect("Failed to parse end");

        // Split range into sub-ranges with same digit count
        let sub_ranges = split_by_digit_count(start, end);

        for (sub_start, sub_end) in sub_ranges {
            let digits = sub_start.to_string().len();

            // Part 1: only prefix length = digits/2 (exactly 2 repetitions)
            if digits % 2 == 0 {
                let half = digits / 2;
                part1 += sum_invalid_ids(sub_start, sub_end, digits, half);
            }

            // Part 2: collect all invalid IDs for all valid prefix lengths
            for prefix_len in 1..=digits / 2 {
                if digits % prefix_len != 0 {
                    continue;
                }
                collect_invalid_ids(sub_start, sub_end, digits, prefix_len, &mut part2_ids);
            }
        }
    }

    let part2: u64 = part2_ids.iter().sum();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn sum_invalid_ids(sub_start: u64, sub_end: u64, digits: usize, prefix_len: usize) -> u64 {
    let repetitions = digits / prefix_len;
    let multiplier = 10_u64.pow(prefix_len as u32);

    // Build the multiplier for repeating: e.g., for 3 reps of 2 digits: 10001
    let mut repeat_multiplier: u64 = 0;
    for i in 0..repetitions {
        repeat_multiplier += multiplier.pow(i as u32);
    }

    // Get prefix range
    let prefix_start = sub_start / multiplier.pow((repetitions - 1) as u32);
    let prefix_end = sub_end / multiplier.pow((repetitions - 1) as u32);

    let mut total: u64 = 0;

    for p in prefix_start..=prefix_end {
        // Skip prefixes with leading zeros (would result in fewer digits)
        if p < 10_u64.pow((prefix_len - 1) as u32) {
            continue;
        }

        let invalid_id = p * repeat_multiplier;

        if invalid_id >= sub_start && invalid_id <= sub_end {
            total += invalid_id;
        }
    }

    total
}

fn collect_invalid_ids(sub_start: u64, sub_end: u64, digits: usize, prefix_len: usize, ids: &mut HashSet<u64>) {
    let repetitions = digits / prefix_len;
    let multiplier = 10_u64.pow(prefix_len as u32);

    // Build the multiplier for repeating
    let mut repeat_multiplier: u64 = 0;
    for i in 0..repetitions {
        repeat_multiplier += multiplier.pow(i as u32);
    }

    // Get prefix range
    let prefix_start = sub_start / multiplier.pow((repetitions - 1) as u32);
    let prefix_end = sub_end / multiplier.pow((repetitions - 1) as u32);

    for p in prefix_start..=prefix_end {
        // Skip prefixes with leading zeros (would result in fewer digits)
        if p < 10_u64.pow((prefix_len - 1) as u32) {
            continue;
        }

        let invalid_id = p * repeat_multiplier;

        if invalid_id >= sub_start && invalid_id <= sub_end {
            ids.insert(invalid_id);
        }
    }
}

fn split_by_digit_count(start: u64, end: u64) -> Vec<(u64, u64)> {
    let mut result = Vec::new();
    let mut current = start;

    while current <= end {
        let digits = current.to_string().len();
        let max_for_digits = 10_u64.pow(digits as u32) - 1;
        let sub_end = max_for_digits.min(end);

        result.push((current, sub_end));

        if sub_end == end {
            break;
        }
        current = sub_end + 1;
    }

    result
}
