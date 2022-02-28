use euler::Best;
use euler::CountOf;
use primal::Sieve;

euler::solution!(p50, "Consecutive prime sum", 997651);

pub fn p50() -> usize {
    let size = 1_000_000;
    let sieve = Sieve::new(size);

    let mut best = Best::<CountOf<usize>>::new();

    for p0 in sieve.primes_from(2) {
        if let Some(CountOf { count, .. }) = *best {
            if p0 * (count + 1) > size {
                // To make a record breaker, we'd need at least count + 1
                // primes, but that would take us over the limit. And the
                // same is true for subsequent primes.
                break;
            }
        }

        // Form partial sums starting at P_n, and record any record-breaking
        // results in the array
        let mut sum = 0;
        let mut count = 0;
        for p in sieve.primes_from(p0) {
            sum += p;
            count += 1;
            if sum >= size {
                break;
            }
            if sieve.is_prime(sum) {
                best.max(CountOf::new(count, sum));
            }
        }
    }

    best.into_inner().map(|hit| *hit).unwrap()
}
