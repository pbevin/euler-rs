use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::hash_map::RandomState;

euler::solution!(p81, "Path sum 2 ways", 427337);
euler::solution!(p82, "Path sum 3 ways", 260324);
euler::solution!(p83, "Path sum 4 ways", 425185);

const SIZE: usize = 80;

pub fn p81() -> usize {
    p081_solve(read_matrix())
}

fn p081_solve(matrix: Matrix) -> usize {
    let start_nodes = [(0, 0)];
    let end_nodes = [(matrix.width - 1, matrix.width - 1)];
    min_path_sum(matrix, &[Dir::E, Dir::S], &start_nodes, &end_nodes)
}

pub fn p82() -> usize {
    p082_solve(read_matrix())
}

fn p082_solve(matrix: Matrix) -> usize {
    let dirs = [Dir::E, Dir::S, Dir::N];
    let start_nodes: Vec<_> = (0..matrix.width).map(|j| (0, j)).collect();
    let end_nodes: Vec<_> = (0..matrix.width).map(|j| (matrix.width - 1, j)).collect();
    min_path_sum(matrix, &dirs, &start_nodes, &end_nodes)
}

pub fn p83() -> usize {
    let dirs = [Dir::N, Dir::E, Dir::S, Dir::W];
    let start_nodes = [(0, 0)];
    let end_nodes = [(SIZE - 1, SIZE - 1)];
    min_path_sum(read_matrix(), &dirs, &start_nodes, &end_nodes)
}

fn read_matrix() -> Matrix {
    Matrix::read(include_str!("p081_matrix.txt"), 80)
}

enum Dir {
    N,
    E,
    S,
    W,
}

impl Dir {
    pub fn try_move(&self, from: &(usize, usize), size: usize) -> Option<(usize, usize)> {
        let &(x, y) = from;
        let (nx, ny) = match self {
            Dir::N => (x, y.checked_sub(1)?),
            Dir::E => (x.checked_add(1)?, y),
            Dir::S => (x, y.checked_add(1)?),
            Dir::W => (x.checked_sub(1)?, y),
        };
        if nx < size && ny < size {
            Some((nx, ny))
        } else {
            None
        }
    }
}

fn min_path_sum(
    matrix: Matrix,
    dirs: &[Dir],
    start_nodes: &[(usize, usize)],
    end_nodes: &[(usize, usize)],
) -> usize {
    let size = matrix.width;

    let mut costs = vec![usize::MAX; matrix.data.len()];
    for &(i, j) in start_nodes {
        costs[i + j * size] = matrix.get(i, j);
    }

    let mut queue: PriorityQueue<(usize, usize), Reverse<usize>, RandomState> =
        PriorityQueue::from_iter(
            start_nodes
                .iter()
                .copied()
                .map(|node @ (x, y)| (node, Reverse(matrix.get(x, y)))),
        );

    while let Some((current, _)) = queue.pop() {
        let (x, y) = current;
        let unfinished_neighbours = dirs.iter().filter_map(|dir| dir.try_move(&current, size));
        for neighbour in unfinished_neighbours {
            let (nx, ny) = neighbour;
            let cost = costs[x + y * size] + matrix.get(nx, ny);
            let p = &mut costs[nx + ny * size];
            if cost < *p {
                *p = cost;
                queue.push(neighbour, Reverse(cost));
            }
        }
    }
    // for j in 0..5 {
    //     print!("{:2}:", j);
    //     for i in 0..5 {
    //         print!(" {:4}", costs[i + j * size]);
    //     }
    //     println!();
    // }
    return end_nodes
        .iter()
        .map(|&(x, y)| costs[x + y * size])
        .min()
        .unwrap();
}

pub struct Matrix {
    data: Vec<usize>,
    width: usize,
}

impl Matrix {
    fn new(data: Vec<usize>, width: usize) -> Self {
        debug_assert_eq!(data.len(), width * width);
        Self { data, width }
    }
    pub fn read(input: &str, width: usize) -> Self {
        let mut data = vec![];
        for line in input.lines() {
            for num in line.split(',') {
                data.push(num.trim().parse().unwrap());
            }
        }
        Self::new(data, width)
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        debug_assert!(i < self.width, "{:?}", (i, j));
        debug_assert!(i + j * self.width < self.data.len(), "{:?}", (i, j));
        self.data[i + j * self.width]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_matrix() -> Matrix {
        let data = [
            "131,673,234,103,18",
            "201,96,342,965,150",
            "630,803,746,422,111",
            "537,699,497,121,956",
            "805,732,524,37,331",
        ]
        .join("\n");
        Matrix::read(&data, 5)
    }

    #[test]
    fn p081_small() {
        assert_eq!(p081_solve(test_matrix()), 2427);
    }

    #[test]
    fn p082_small() {
        assert_eq!(p082_solve(test_matrix()), 994);
    }
}
