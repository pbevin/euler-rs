use euler::Best;

// The number of rectangles is T(width) * T(height), where T is the triangle function
// T(n) = n * (n + 1) / 2.

euler::solution!(p085, "Counting rectangles", 2772);

const TARGET: usize = 2_000_000;

fn p085() -> usize {
    let mut best = Best::new();
    let xs = (0..=TARGET).map(|n| n * (n + 1) / 2).collect::<Vec<_>>();
    for (i, &ti) in xs.iter().enumerate().take(TARGET / 2).skip(2) {
        // j is the unique index where (i, j) and (i, j+1) are on opposite sides of the target.
        let j = xs.partition_point(|&tj| ti * tj < TARGET);
        if j < i {
            continue;
        }
        // Check how close each side is - diff1 is above, diff2 is below.
        let tj = xs[j];
        let tj1 = xs[j - 1];
        let diff1 = ti * tj - TARGET;
        let diff2 = TARGET - ti * tj1;
        best.min((diff1, (i, j)));
        best.min((diff2, (i, j - 1)));
    }

    let (w, h) = best.into_inner().unwrap().1;
    w * h
}
