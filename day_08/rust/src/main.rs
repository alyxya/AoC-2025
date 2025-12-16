fn dsu_root(dsu_parent: &mut Vec<usize>, idx: usize) -> usize {
    if dsu_parent[idx] != dsu_parent[dsu_parent[idx]] {
        dsu_parent[idx] = dsu_root(dsu_parent, dsu_parent[idx]);
    }
    return dsu_parent[idx];
}

fn dsu_merge(dsu_parent: &mut Vec<usize>, dsu_size: &mut Vec<i64>, a: usize, b: usize) {
    let a_root: usize = dsu_root(dsu_parent, a);
    let b_root: usize = dsu_root(dsu_parent, b);
    if a_root == b_root {
        return;
    }
    let (a_root, b_root) = if dsu_size[a_root] < dsu_size[b_root] {
        (b_root, a_root)
    } else {
        (a_root, b_root)
    };
    dsu_parent[b_root] = a_root;
    dsu_size[a_root] += dsu_size[b_root];
    dsu_size[b_root] = 0;
}

fn main() {
    let input: String = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let coords: Vec<(i64, i64, i64)> = input.lines().map(|s| {
        let mut it = s.split(',').map(|n| n.parse().unwrap());
        (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
    }).collect();
    let num_connections: usize = std::env::args().nth(2).unwrap().parse().unwrap();

    let mut dists: Vec<(i64, usize, usize)> = vec![];

    for i in 0..coords.len() {
        for j in 0..i {
            dists.push(((coords[j].0 - coords[i].0) * (coords[j].0 - coords[i].0) + (coords[j].1 - coords[i].1) * (coords[j].1 - coords[i].1) + (coords[j].2 - coords[i].2) * (coords[j].2 - coords[i].2), j, i));
        }
    }
    dists.sort();

    let mut dsu_parent: Vec<usize> = (0..coords.len()).collect();
    let mut dsu_size: Vec<i64> = vec![1; coords.len()];

    for i in 0..num_connections {
        dsu_merge(&mut dsu_parent, &mut dsu_size, dists[i].1, dists[i].2);
    }
    let mut sorted_size: Vec<i64> = dsu_size.clone();
    sorted_size.sort_by(|a, b| b.cmp(a));

    let part1: i64 = sorted_size[0] * sorted_size[1] * sorted_size[2];
    let mut part2: i64 = 0;

    for i in num_connections..dists.len() {
        dsu_merge(&mut dsu_parent, &mut dsu_size, dists[i].1, dists[i].2);
        if dsu_size[dsu_root(&mut dsu_parent, dists[i].1)] == coords.len() as i64 {
            part2 = coords[dists[i].1].0 * coords[dists[i].2].0;
            break;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
