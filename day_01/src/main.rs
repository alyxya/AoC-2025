fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut position: i32 = 50;
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Failed to parse number");

        let before = position;
        if direction == "R" {
            position += amount;
        } else {
            position -= amount;
        }

        // Adjust before by +-1 based on direction to get inclusive range
        let start = if direction == "R" { before + 1 } else { before - 1 };

        // Get inclusive range [low, high]
        let low = start.min(position);
        let high = start.max(position);

        // Count multiples of 100 in [low, high]
        part2 += high.div_euclid(100) - low.div_euclid(100);
        if low.rem_euclid(100) == 0 {
            part2 += 1;
        }

        if position % 100 == 0 {
            part1 += 1;
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
