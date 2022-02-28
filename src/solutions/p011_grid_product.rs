use itertools::Itertools;

euler::solution!(p011, "Largest product in a grid", 70600674);

pub fn p011() -> i64 {
    let size = 20;
    let grid = include_str!("p011.txt")
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    debug_assert_eq!(grid.len(), size * size);

    let hor = max_4prod(&grid, size, |x, y| (x, y));
    let ver = max_4prod(&grid, size, |x, y| (y, x));
    let d1a = max_4prod(&grid, size, |x, y| (x + y, y));
    let d1b = max_4prod(&grid, size, |x, y| (x, x + y));
    let d2a = max_4prod(&grid, size, |x, y| (size as isize - 1 - x - y, y));
    let d2b = max_4prod(&grid, size, |x, y| (size as isize - 1 - x - y, x));

    [hor, ver, d1a, d1b, d2a, d2b].into_iter().max().unwrap()
}

fn max_4prod<F>(grid: &[i64], size: usize, f: F) -> i64
where
    F: Fn(isize, isize) -> (isize, isize),
{
    let isize = size as isize;

    (0..)
        .map(|j| {
            (0..)
                .map(|i| f(i, j))
                .take_while(|(x, y)| (0..isize).contains(x) && (0..isize).contains(y))
                .map(|(x, y)| {
                    let x: usize = x.try_into().unwrap();
                    let y: usize = y.try_into().unwrap();
                    grid[x + y * size]
                })
                .tuple_windows()
                .map(|(a, b, c, d)| a * b * c * d)
                .max()
        })
        .take_while(|max| max.is_some())
        .max()
        .unwrap()
        .unwrap()
}
