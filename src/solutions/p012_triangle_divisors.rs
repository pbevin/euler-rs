use primal::Sieve;

euler::solution!(p12, "Highly divisible triangular number", 76576500);

pub fn p12() -> usize {
    // If n = p1^n1 * p2^n2 * p3^n3 * ...
    // then the number of divisors is (n1 + 1)(n2 + 1)(n3 + 1)...
    //
    // Triangle numbers are easy to calculate:
    //  T(k) = k(k+1)/2.
    //
    // In theory, we could speed this up by using the fact that
    //  T(k) = k(k+1)/2
    //       = k(k-1)(k+1)/2(k-1)
    //       = (k+1)T(k-1)/(k-1)
    // ...and since we're iterating them in order, we can keep
    // a record of the prime factors of the last two ks.
    //
    // But the record keeping turns out to be harder work than
    // just factoring each T(k) in turn (90ms vs 35ms on this problem
    // on my laptop). So we just do it the boring way.
    let sieve = Sieve::new(10_000); // guessing the limit
    for k in 10.. {
        let n = k * (k + 1) / 2;
        let num_factors: usize = sieve
            .factor(n)
            .unwrap()
            .into_iter()
            .map(|(_, n)| n + 1)
            .product();

        if num_factors > 500 {
            return k * (k + 1) / 2;
        }
    }
    unreachable!()
}
