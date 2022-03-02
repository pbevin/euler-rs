use itertools::Itertools;
use primal::Sieve;

euler::solution!(p357, "Prime generating integers", 1739023853137);

fn p357() -> usize {
    let sieve = Sieve::new(100_000_000);
    let mut v = vec![1, 2];

    // Three optimizations:
    //
    // First, (1, n) will be a pair of factors for any n, so n must be 1 below a
    // prime number.
    //
    // Second, apart from 1, which is a special case, we only need to look at numbers
    // of the form n = 2k + 2:
    //
    // If n == 4k, then 2 and n/2 are both even, so 2 + n/2 is composite.
    // If n is odd, then 1 + n is composite.
    // for k in 0..25_000_000 {
    //     let n = 4 * k + 2;
    //
    // Final optimization: n must be squarefree. If n = p^2q, then (p, pq) is
    // a divisor pair, and p + pq is composite.

    for n_plus_one in sieve.primes_from(7) {
        let n = n_plus_one - 1;

        if n % 4 != 2 {
            continue;
        }

        // Since n = 4k+2, (2, n/2) will be a pair.
        if !sieve.is_prime(2 + n / 2) {
            continue;
        }

        let factors = sieve.factor(n).unwrap();
        if factors.iter().any(|&(_, n)| n > 1) {
            // n isn't squarefree. Reject!
            continue;
        }

        let mut small_divisors = factors
            .into_iter()
            .map(|(n, _)| n)
            .powerset()
            .map(|xs| xs.into_iter().product::<usize>())
            .filter(|d| d * d < n);

        if small_divisors.any(|d| !sieve.is_prime(d + n / d)) {
            continue;
        }

        v.push(n);
    }

    assert!(v.contains(&30));

    v.into_iter().sum()
}
