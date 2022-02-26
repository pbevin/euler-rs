use std::cmp;

fn read_matrix() -> Matrix {
    Matrix::read(include_str!("p081_matrix.txt"), 80)
}

pub fn p81() -> usize {
    p081_solve(read_matrix())
}

pub fn p82() -> usize {
    p082_solve(read_matrix())
}

fn p081_solve(matrix: Matrix) -> usize {
    let size = matrix.width;
    let mut min_cost = Matrix::with_max_default(size);

    min_cost.put(0, 0, matrix.get(0, 0));
    for i in 1..size {
        min_cost.put(i, 0, min_cost.get(i - 1, 0) + matrix.get(i, 0));
    }
    for j in 1..size {
        min_cost.put(0, j, min_cost.get(0, j - 1) + matrix.get(0, j));
    }

    for j in 1..size {
        for i in 1..size {
            let a = matrix.get(i, j);
            let from_above = min_cost.get(i, j - 1);
            let from_left = min_cost.get(i - 1, j);
            let from_best = cmp::min(from_above, from_left);
            min_cost.put(i, j, from_best + a);
        }
    }

    min_cost.get(size - 1, size - 1)
}

fn p082_solve(matrix: Matrix) -> usize {
    let size = matrix.width;

    // The leftmost column is easy, because we can start anywhere. The
    // min cost is just the value.
    let mut min_cost: Vec<usize> = (0..size).map(|j| matrix.get(0, j)).collect();

    for i in 1..size {
        // For each column, we can either go up it or down it; not both.

        // First, calculate the costs in this column if we are only allowed to go down and right.
        let mut prev = usize::MAX;
        let mut down_costs = vec![];
        for (j, &from_left) in min_cost.iter().enumerate() {
            let best = matrix.get(i, j) + cmp::min(prev, from_left);
            down_costs.push(best);
            prev = best;
        }

        // We calculate up_costs in a similar way, but with more reversing.
        let mut prev = usize::MAX;
        let mut up_costs = vec![];
        for (j, &from_left) in min_cost.iter().enumerate().rev() {
            let best = matrix.get(i, j) + cmp::min(prev, from_left);
            up_costs.push(best);
            prev = best;
        }
        up_costs.reverse();

        // Now we can just read off the best paths
        min_cost = (0..size).map(|j| {
            let mut best = min_cost[j];
            if j > 0 {
                best = cmp::min(down_costs[j - 1], best);
            }
            if j < size - 1 {
                best = cmp::min(up_costs[j + 1], best);
            }
            let a = matrix.get(i, j);
            best + a
        }).collect();
    }

    // Finally, get the minimum cost of anything in the right hand column
    min_cost.into_iter().min().unwrap()
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

    pub fn with_max_default(width: usize) -> Self {
        let data = vec![usize::MAX; width * width];
        Self::new(data, width)
    }

    pub fn get(&self, i: usize, j: usize) -> usize {
        debug_assert!(i < self.width);
        debug_assert!(i + j * self.width < self.data.len());
        self.data[i + j * self.width]
    }

    pub fn put(&mut self, i: usize, j: usize, val: usize) {
        debug_assert!(i < self.width);
        self.data[i + j * self.width] = val;
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
