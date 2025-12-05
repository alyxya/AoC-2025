use std::env;

fn main() {
    let filename: String = env::args().nth(1).expect("Usage: day_05 <input_file>");
    let input = std::fs::read_to_string(filename).expect("Failed to read input file");
    let sections: Vec<&str> = input.split("\n\n").collect();
    let ranges: Vec<(i64, i64)> = sections[0].trim().split('\n').map(|range| range.split_once('-').unwrap()).map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).collect();
    let part1: i32 = sections[1].trim().split('\n').map(|val| val.parse::<i64>().unwrap()).map(|val| ranges.iter().any(|&(a, b)| a <= val && val <= b) as i32).sum();

    let mut endpoints: Vec<(i64, i32)> = ranges.iter().flat_map(|&(a, b)| [(a, 1), (b + 1, -1)]).collect();
    endpoints.sort();
    let part2: i64 = endpoints.iter().scan((0i64, 0), |(last, intervals), &(endpoint, side)| {
        let prev_endpoint = *last;
        let prev_intervals = *intervals;
        *last = endpoint;
        *intervals += side;
        Some((endpoint - prev_endpoint, prev_intervals))
    }).map(|(delta, intervals)| if intervals > 0 { delta } else { 0 }).sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
