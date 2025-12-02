fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut total: u64 = 0;

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

            // Skip odd digit counts
            if digits % 2 != 0 {
                continue;
            }

            let half = digits / 2;
            let multiplier = 10_u64.pow(half as u32);

            // Get prefix of start and end
            let prefix_start = sub_start / multiplier;
            let prefix_end = sub_end / multiplier;

            // For each prefix P, the invalid ID is P * multiplier + P = P * (multiplier + 1)
            for p in prefix_start..=prefix_end {
                let invalid_id = p * multiplier + p;

                if invalid_id >= sub_start && invalid_id <= sub_end {
                    total += invalid_id;
                }
            }
        }
    }

    println!("{}", total);
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
