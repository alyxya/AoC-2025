fn main() {
    let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let (ranges_str, ids_str) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(i64, i64)> = ranges_str
        .lines()
        .filter_map(|l| l.split_once('-'))
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect();

    let part1 = ids_str
        .lines()
        .filter_map(|l| l.parse::<i64>().ok())
        .filter(|id| ranges.iter().any(|(lo, hi)| (lo..=hi).contains(&id)))
        .count();

    let mut events: Vec<_> = ranges
        .iter()
        .flat_map(|&(a, b)| [(a, 1), (b + 1, -1)])
        .collect();
    events.sort_unstable();

    let part2: i64 = events
        .iter()
        .scan((0i64, 0i32), |(pos, depth), &(at, delta)| {
            let span = if *depth > 0 { at - *pos } else { 0 };
            *pos = at;
            *depth += delta;
            Some(span)
        })
        .sum();

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
