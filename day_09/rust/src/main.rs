fn main() {
    let input: String = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let coords: Vec<(i64, i64)> = input.lines().map(|s| {
        let mut it = s.split(',').map(|n| n.parse().unwrap());
        (it.next().unwrap(), it.next().unwrap())
    }).collect();

    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for i in 0..coords.len() {
        for j in 0..i {
            let mut cur: i64 = (coords[i].0 - coords[j].0).abs() + 1;
            cur *= (coords[i].1 - coords[j].1).abs() + 1;
            if cur > part1 {
                part1 = cur;
            }
        }
    }

    let mut largest_x: i64 = 0i64;
    let mut largest_y: i64 = 0i64;
    let mut largest_idx: usize = 0;
    for (idx, &(x, y)) in coords.iter().enumerate() {
        if x > largest_x || (x == largest_x && y > largest_y) {
            largest_x = x;
            largest_y = y;
            largest_idx = idx;
        }
    }

    let mut normalized_coords: Vec<(i64, i64)> = Vec::new();
    for i in 0..coords.len() {
        let cur_idx = largest_idx + coords.len() + i;
        let cur_x = coords[cur_idx % coords.len()].0;
        let cur_y = coords[cur_idx % coords.len()].1;
        let other_x = coords[(cur_idx + if coords[cur_idx % coords.len()].0 == coords[(cur_idx + 1) % coords.len()].0 { coords.len() - 1 } else { 1 }) % coords.len()].0;
        let other_y = coords[(cur_idx + if coords[cur_idx % coords.len()].1 == coords[(cur_idx + 1) % coords.len()].1 { coords.len() - 1 } else { 1 }) % coords.len()].1;

        if other_x < cur_x && other_y < cur_y {
            normalized_coords.push(if i % 2 == 0 { (cur_x, cur_y) } else { (cur_x - 1, cur_y - 1) });
        } else if other_x > cur_x && other_y > cur_y {
            normalized_coords.push(if i % 2 == 0 { (cur_x - 1, cur_y - 1) } else { (cur_x, cur_y) });
        } else if other_x < cur_x && other_y > cur_y {
            normalized_coords.push(if i % 2 == 0 { (cur_x - 1, cur_y) } else { (cur_x, cur_y - 1) });
        } else if other_x > cur_x && other_y < cur_y {
            normalized_coords.push(if i % 2 == 0 { (cur_x, cur_y - 1) } else { (cur_x - 1, cur_y) });
        }
    }

    let area = |x1: i64, y1: i64, x2: i64, y2: i64| -> i64 { i64::min(x1, x2) * i64::min(y1, y2) };
    for i in 0..coords.len() {
        for j in 0..i {
            let (max_x, max_y) = (i64::max(coords[i].0, coords[j].0), i64::max(coords[i].1, coords[j].1));
            let (min_x, min_y) = (i64::min(coords[i].0, coords[j].0) - 1, i64::min(coords[i].1, coords[j].1) - 1);
            let mut cur: i64 = 0;

            for (idx, &(x, y)) in normalized_coords.iter().enumerate() {
                let overlap = area(x, y, max_x, max_y) + area(x, y, min_x, min_y) - area(x, y, min_x, max_y) - area(x, y, max_x, min_y);
                cur += overlap * (if idx % 2 == 0 { 1 } else { -1 });
            }

            if cur > part2 && cur == (max_x - min_x) * (max_y - min_y) {
                part2 = cur;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
