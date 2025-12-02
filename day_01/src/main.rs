fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input.txt");

    let mut position: i32 = 50;
    let mut count: i32 = 0;

    for line in input.lines() {
        let direction = &line[0..1];
        let amount: i32 = line[1..].parse().expect("Failed to parse number");

        if direction == "R" {
            position += amount;
        } else {
            position -= amount;
        }

        if position % 100 == 0 {
            count += 1;
        }
    }

    println!("{}", count);
}
