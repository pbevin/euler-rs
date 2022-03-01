use primal::Sieve;

euler::solution!(p127, "abc-hits", 18407904);

fn p127() -> usize {
    let max = 120_000;
    // let max = 1000;
    let mut sum = 0;
    let sieve = Sieve::new(max);

    // Precompute values of rad(n) that we will need. Also -- and this is
    // the most important optimization -- create a list of (rad(n), n) and
    // sort it on rad(n).
    let mut rads = Vec::with_capacity(max);
    let mut sorted_rads = Vec::with_capacity(max);
    rads.push(0);
    rads.push(1);
    for n in 2..max {
        let r = sieve
            .factor(n)
            .unwrap()
            .into_iter()
            .map(|(p, _)| p)
            .product();

        rads.push(r);
        sorted_rads.push((r, n));
    }
    sorted_rads.sort_unstable();

    // Process a = 1 separately, because some optimizations apply:
    for c in 3..max {
        let b = c - 1;
        // No need to check if a, b, and c are coprime!
        if rads[b] * rads[c] < c {
            sum += c;
        }
    }

    for c in 3..max {
        let radc = rads[c];

        // a and b are now both greater than 1, so rad(a) * rad(b)
        // has to be at least 2 * 3 == 6, so we can cut out a lot
        // of values right here:
        if radc * 6 > c {
            continue;
        }
        let c_over_radc = c / radc;

        // Here's where we use that important optimization.  The insight
        // is that if rad(a)*rad(b)*rad(c) < c, then rad(a) < c/rad(c).
        for &(rada, a) in sorted_rads
            .iter()
            .take_while(|&&(rada, _)| rada < c_over_radc)
        {
            if !(2..c / 2).contains(&a) {
                continue;
            }
            let b = c - a;
            let radb = rads[b];
            // If a and c are coprime and a + b == c, then (a,b) and (b,c)
            // are also coprime.
            if gcd::binary_usize(rada, radb) != 1 {
                continue;
            }
            if rada * radb < c_over_radc {
                sum += c;
            }
        }
    }

    sum
}
