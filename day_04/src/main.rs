use std::env;

fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let (rows, cols) = (grid.len(), grid[0].len());
    (-1..=1)
        .flat_map(|dr| (-1..=1).map(move |dc| (dr, dc)))
        .filter(|&(dr, dc)| (dr, dc) != (0, 0))
        .filter(|(dr, dc)| {
            let (nr, nc) = (r as i32 + dr, c as i32 + dc);
            nr >= 0 && nc >= 0 && (nr as usize) < rows && (nc as usize) < cols
                && grid[nr as usize][nc as usize] == '@'
        })
        .count()
}

fn find_removable(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    (0..grid.len())
        .flat_map(|r| (0..grid[0].len()).map(move |c| (r, c)))
        .filter(|&(r, c)| grid[r][c] == '@' && count_neighbors(grid, r, c) < 4)
        .collect()
}

fn main() {
    let filename = env::args().nth(1).expect("Usage: day_04 <input_file>");
    let mut grid: Vec<Vec<char>> = std::fs::read_to_string(filename)
        .expect("Failed to read input file")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut part1 = 0;
    let mut part2 = 0;
    let mut first = true;

    loop {
        let removable = find_removable(&grid);
        if removable.is_empty() {
            break;
        }
        if first {
            part1 = removable.len();
            first = false;
        }
        part2 += removable.len();
        for (r, c) in removable {
            grid[r][c] = '.';
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
