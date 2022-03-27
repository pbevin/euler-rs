use itertools::Itertools;

euler::solution!(p099, "Largest exponential", 709);

// We could generate all 1,000 numbers as multiple precision ints, but it's
// much faster to take logarithms.
fn p099() -> usize {
    let values: Vec<_> = include_str!("p099_base_exp.txt")
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(n, e)| (n.parse().unwrap(), e.parse().unwrap()))
        .map(logexp)
        .collect_vec();

    // The "1 + " is to correct our array index (starts at 0) to the
    // line number (starts at 1).
    1 + (0..values.len()).max_by_key(|i| &values[*i]).unwrap()
}

/// Given numbers a and b, finds the logarithm of a^b.
fn logexp((a, b): (u32, u32)) -> i64 {
    let a = f64::from(a);
    let b = f64::from(b);
    (a.ln() * b) as i64
}
