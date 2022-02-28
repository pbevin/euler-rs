euler::solution!(p14, "Longest Collatz sequence", 837799);

pub fn p14() -> usize {
    let upto = 1_000_000;
    let mut lengths = vec![0; upto];
    lengths[1] = 1;
    for i in 1..upto {
        calc_collatz(i, &mut lengths);
    }

    (1..upto).max_by_key(|n| lengths[*n]).unwrap()
}

fn calc_collatz(i: usize, lengths: &mut [u32]) -> u32 {
    // If it's cached, return the cached value
    if i < lengths.len() && lengths[i] > 0 {
        return lengths[i];
    }

    // Otherwise, recurse.
    let len = if i % 2 == 0 {
        calc_collatz(i / 2, lengths)
    } else {
        calc_collatz(3 * i + 1, lengths)
    };
    if i < lengths.len() {
        lengths[i] = len + 1;
    }
    len + 1
}
