use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage: day_04 <input_file>");
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    let mut lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let num_rows = lines.len() as i32;
    let num_cols = lines[0].len() as i32;

    let mut part1: Option<usize> = None;
    let mut part2 = 0;

    let mut first_iter: bool = true;

    loop {
        let mut coords: Vec<(usize, usize)> = Vec::new();
        for r in 0..num_rows {
            for c in 0..num_cols {
                if lines[r as usize][c as usize] != '@' {
                    continue;
                }
                let mut num_neighbors = 0;
                for rr in r-1..=r+1 {
                    for cc in c-1..=c+1 {
                        if rr >= 0 && rr < num_rows && cc >= 0 && cc < num_cols && (rr != r || cc != c) {
                            num_neighbors += if lines[rr as usize][cc as usize] == '@' { 1 } else { 0 };
                        }
                    }
                }

                if num_neighbors < 4 {
                    coords.push((r as usize, c as usize));
                }
            }
        }

        if first_iter {
            part1 = Some(coords.len());
        }
        part2 += coords.len() as i32;

        for &(r, c) in &coords {
            lines[r][c] = '.';
        }

        first_iter = false;

        if coords.len() == 0 {
            break;
        }
    }

    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", part2);
}
