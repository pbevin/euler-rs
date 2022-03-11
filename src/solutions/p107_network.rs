use itertools::Itertools;
use partitions::PartitionVec;

euler::solution!(p107, "Minimal Network", 259679);

const NETWORK: &str = include_str!("p107_network.txt");

fn p107() -> usize {
    let matrix = NETWORK
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|word| {
                    if word == "-" {
                        None
                    } else {
                        word.parse::<usize>().ok()
                    }
                })
                .collect_vec()
        })
        .collect_vec();

    let mut edges = matrix
        .iter()
        .enumerate()
        .flat_map(|(v, xs)| {
            let it = xs.iter().enumerate().take(v);
            it.filter_map(|(u, wt)| wt.map(|w| Edge::new(u, v, w)))
                .collect_vec()
        })
        .collect_vec();

    edges.sort_unstable_by_key(|e| e.weight);

    let mut saving = 0;
    let mut p = (0..matrix.len()).collect::<PartitionVec<usize>>();
    for Edge { u, v, weight: w } in edges {
        if p.same_set(u, v) {
            saving += w;
        } else {
            p.union(u, v)
        }
    }

    saving
}

#[derive(Debug)]
struct Edge {
    u: usize,
    v: usize,
    weight: usize,
}

impl Edge {
    fn new(u: usize, v: usize, weight: usize) -> Self {
        Self { u, v, weight }
    }
}
