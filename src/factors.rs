/// Simple factorization into primes.
///
/// Panics if n is negative.
pub fn factors(n: i64) -> Factors {
    if n <= 0 {
        panic!("Can only factorize positive numbers: {}", n);
    }
    Factors { n, p: 2 }
}

pub struct Factors {
    n: i64,
    p: i64,
}

impl Iterator for Factors {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n <= 1 {
            return None;
        }

        loop {
            let n = self.n;
            let p = self.p;
            if n % p == 0 {
                self.n /= p;
                return Some(p);
            }

            if p * p > n {
                // n is prime!
                self.n = 1;
                return Some(n);
            }

            // Find the next p. It might not be a prime, but we'd
            // like to skip as many obvious non-primes as possible.
            // The sequence is 2, 3, then 6n±1.
            if p == 2 {
                self.p = 3;
            } else if p == 3 {
                self.p = 5;
            } else if p % 6 == 5 {
                self.p += 2;
            } else if p % 6 == 1 {
                self.p += 4;
            } else {
                panic!("p = {}", p)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        /// Tests factorization of numbers up to 2^40-1.
        /// Our simple algorithm is very slow on large numbers,
        /// e.g., 38711993933270513 = 71_198_209 * 543_721_457,
        /// or worse, 38711993933270521 which is prime.
        /// Note that `factor` in coreutils handles it just fine
        /// because it uses number theory witchcraft.
        fn factors_ok(n: i64) {
            let n = n & ((1<<40) - 1);
            let xs = factors(n).collect::<Vec<_>>();
            let n2: i64 = xs.iter().product();
            prop_assert_eq!(n, n2);
        }
    }

    #[test]
    fn test_121() {
        assert_eq!(factors(121).collect::<Vec<_>>(), [11, 11])
    }

    #[test]
    fn test_big_prime() {
        let p: i64 = 137438953481;
        assert_eq!(factors(p).collect::<Vec<_>>(), [p]);
    }

    #[test]
    fn test_many_factors() {
        let n: i64 = 2 * 3 * 5 * 7 * 7 * 11 * 13 * 17 * 19;
        assert_eq!(
            factors(n).collect::<Vec<_>>(),
            [2, 3, 5, 7, 7, 11, 13, 17, 19]
        );
    }

    #[test]
    /// Tests the factorization of F_5 = 4294967297, the fifth
    /// Fermat number. https://en.wikipedia.org/wiki/Fermat_number
    /// Historical note: Fermat died believing F_5 was prime.
    /// It took Euler 3 years to find a factor, and he was a genius.
    /// http://eulerarchive.maa.org/hedi/HEDI-2007-03.pdf
    ///
    /// And here I am, 300 years later, doing it as a quick test of
    /// some simple code, and my computer does it in under five
    /// microseconds.
    fn test_f5() {
        let n: i64 = (1 << 32) + 1;
        assert_eq!(factors(n).collect::<Vec<_>>(), [641, 6700417]);
    }
}
