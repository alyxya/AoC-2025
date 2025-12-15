fn main() {
    let input: String = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let lines: Vec<&str> = input.lines().collect();

    let nums: Vec<Vec<i64>> = lines[..lines.len() - 1]
        .iter()
        .map(|s| {
            s.split_whitespace()
                .map(|t| {
                    t.parse::<i64>()
                        .expect(&format!("Failed to parse: '{}'", t))
                })
                .collect()
        })
        .collect();
    let operators: Vec<&str> = lines.last().unwrap().split_whitespace().collect();

    let nums_transposed: Vec<i64> = (0..lines[0].len()).map(|n| {
        let mut val: i64 = 0;
        for line in &lines[..lines.len() - 1] {
            let c = line.as_bytes()[n] as char;
            if c != ' ' {
                val = val * 10 + c.to_digit(10).unwrap() as i64;
            }
        }
        val
    }).collect();

    let part1: i64 = operators
        .iter()
        .enumerate()
        .map(|(idx, &op)| {
            if op == "*" {
                nums.iter().map(|row| row[idx]).product::<i64>()
            } else {
                nums.iter().map(|row| row[idx]).sum()
            }
        })
        .sum();
    let last_line: Vec<char> = lines.last().unwrap().chars().collect();
    let part2: i64 = last_line.iter().enumerate().fold((0, 0, '+'), |(total, acc, cur_op), (idx, &op)| {
        let mut next_total: i64 = total;
        let next_op = if op == ' ' { cur_op } else { op };
        let mut next_acc = if op == ' ' { acc } else { if op == '+' { 0 } else { 1 } };
        if idx + 1 == last_line.len() || last_line[idx + 1] == ' ' {
            next_acc = if next_op == '+' { next_acc + nums_transposed[idx] } else { next_acc * nums_transposed[idx] };
        }
        if idx + 1 == last_line.len() || last_line[idx + 1] != ' ' {
            next_total += next_acc;
        }
        (next_total, next_acc, next_op)
    }).0;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
