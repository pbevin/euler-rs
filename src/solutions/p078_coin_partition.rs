euler::solution!(p78, "Coin partitions", 55374);

pub fn p78() -> i64 {
    // This is like problem 76, except that we're asked for the actual number
    // of permutations, rather than 1 less. Also, we have to search a possibly
    // unbounded space to get there.
    //
    // The numbers get too big for a usize quickly: p(1,000) > 2^100. Fortunately,
    // since we only care about the value mod N, we can work in Z_N (N = 1,000,000).
    //
    // The code below is somewhat optimized from euler_p() in p76, because we're
    // doing a much longer calculation.

    let mut p = Vec::with_capacity(300_000);
    p.push(1);
    p.push(1);

    for n in 2.. {
        let mut sum = 0;
        for k in 1..n {
            let n1 = n - k * (3 * k - 1) / 2;
            let n2 = n - k * (3 * k + 1) / 2;
            // If n1 is negative, then so is n2, and we can just stop counting now.
            if n1 < 0 {
                break;
            }

            let mut t = p[n1 as usize];
            if n2 >= 0 {
                t += p[n2 as usize]
            };

            let adding = k % 2 == 1;
            if adding {
                sum += t;
            } else {
                sum -= t;
            }
        }
        sum %= 1_000_000;
        if sum == 0 {
            return n;
        }
        p.push(sum);
        if n == 100_000 {
            panic!("missed it");
        }
    }
    unreachable!()
}
