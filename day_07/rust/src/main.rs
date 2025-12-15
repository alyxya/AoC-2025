fn main() {
    let input: String = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut lines: Vec<String> = input.lines().map(|s| s.to_owned()).collect();
    lines[1] = lines[0].chars().map(|c| if c == 'S' { '|' } else { c }).collect();
    let mut state: Vec<i64> = lines[0].chars().map(|c| if c == 'S' { 1 } else { 0 }).collect();

    let mut part1: i32 = 0;

    for i in (2..lines.len()).step_by(2) {
        let mut next_state: Vec<i64> = vec![0i64; state.len()];

        for j in 0..lines[i].len() {
            if lines[i].as_bytes()[j] == b'^' && lines[i - 1].as_bytes()[j] == b'|' {
                part1 += 1;
                lines[i].replace_range(j - 1..j, "|");
                lines[i].replace_range(j + 1..j + 2, "|");
                lines[i + 1].replace_range(j - 1..j, "|");
                lines[i + 1].replace_range(j + 1..j + 2, "|");
                next_state[j - 1] += state[j];
                next_state[j + 1] += state[j];
            } else if lines[i - 1].as_bytes()[j] == b'|' {
                lines[i].replace_range(j..j + 1, "|");
                lines[i + 1].replace_range(j..j + 1, "|");
                next_state[j] += state[j];
            }
        }

        state = next_state;
    }

    let part2: i64 = state.iter().sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
