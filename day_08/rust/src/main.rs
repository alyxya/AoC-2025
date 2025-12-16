struct Dsu {
    parent: Vec<usize>,
    size: Vec<i64>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        Dsu {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn root(&mut self, idx: usize) -> usize {
        if self.parent[idx] != self.parent[self.parent[idx]] {
            let p = self.parent[idx];
            self.parent[idx] = self.root(p);
        }
        self.parent[idx]
    }

    fn merge(&mut self, a: usize, b: usize) {
        let a_root = self.root(a);
        let b_root = self.root(b);
        if a_root == b_root {
            return;
        }
        let (a_root, b_root) = if self.size[a_root] < self.size[b_root] {
            (b_root, a_root)
        } else {
            (a_root, b_root)
        };
        self.parent[b_root] = a_root;
        self.size[a_root] += self.size[b_root];
        self.size[b_root] = 0;
    }
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

    let mut dsu = Dsu::new(coords.len());

    for i in 0..num_connections {
        dsu.merge(dists[i].1, dists[i].2);
    }
    let mut sorted_size = dsu.size.clone();
    sorted_size.sort_by(|a, b| b.cmp(a));

    let part1: i64 = sorted_size[0] * sorted_size[1] * sorted_size[2];
    let mut part2: i64 = 0;

    for i in num_connections..dists.len() {
        dsu.merge(dists[i].1, dists[i].2);
        let root = dsu.root(dists[i].1);
        if dsu.size[root] == coords.len() as i64 {
            part2 = coords[dists[i].1].0 * coords[dists[i].2].0;
            break;
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
