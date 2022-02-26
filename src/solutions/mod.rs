mod p008_series_product;
mod p011_grid_product;
mod p012_triangle_divisors;
mod p013_large_sum;
mod p014_collatz;
mod p054_poker;
mod p081_path_sum;

use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use euler::partitions3;
use euler::Best;
use euler::CountOf;
use itertools::Itertools;
use memoize::memoize;
use primal::Sieve;
use primal::StreamingSieve;

pub use p008_series_product::*;
pub use p011_grid_product::*;
pub use p012_triangle_divisors::*;
pub use p013_large_sum::*;
pub use p014_collatz::*;
pub use p054_poker::*;
pub use p081_path_sum::*;



pub fn p1() -> i64 {
    (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

pub fn p2() -> i64 {
    fibs(1, 2)
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

pub fn p3() -> i64 {
    factors(600851475143).last().unwrap()
}

pub fn p4() -> i64 {
    (100_i64..1000)
        .cartesian_product(100..1000)
        .map(|(a, b)| a * b)
        .filter(|n| is_palindrome(*n))
        .max()
        .unwrap()
}

pub fn p5() -> i64 {
    // Find the least common multiple of 1..=20.
    // Could also do this by finding the GCD using
    // Euclid's method, and calculating 20!/gcd.
    let mut n = 1;
    for k in 2..=20 {
        n *= k;
        for p in factors(k) {
            if (n / p) % k == 0 {
                n /= p;
            }
        }
    }
    n
}

pub fn p6() -> u32 {
    fn square(n: u32) -> u32 {
        n * n
    }

    let sum_of_squares = (1..=100).map(square).sum::<u32>();
    let square_of_sum = square((1..=100).sum::<u32>());

    square_of_sum - sum_of_squares
}

pub fn p7() -> usize {
    StreamingSieve::nth_prime(10_001)
}


pub fn p9() -> i64 {
    partitions3(1000)
        .filter(|(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .max()
        .unwrap()
}

pub fn p10() -> i64 {
    let limit = 2_000_000;
    let sum: usize = Sieve::new(limit)
        .primes_from(2)
        .filter(move |&p| p < limit)
        .sum();
    sum.try_into().unwrap()
}



pub fn p15() -> usize {
    let size = 21;

    let mut num_routes = vec![0; size * size];

    // Initialize the top and left edges - each vertex has exactly one
    // way to get to it.
    for x in 0..size {
        num_routes[x] = 1;
        num_routes[x * size] = 1;
    }

    for y in 1..size {
        for x in 1..size {
            // Vertex (x, y) could come from (x-1, y) or (x, y-1).
            let pos1 = x - 1 + y * size;
            let pos2 = x + (y - 1) * size;
            num_routes[x + y * size] = num_routes[pos1] + num_routes[pos2];
        }
    }

    num_routes[size * size - 1]
}

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

pub fn p51() -> usize {
    let sieve = Sieve::new(1_000_000);
    // There are 10 possible digit replacements, and none of them
    // can be divisible by 3. So if we are to get 8 primes, there
    // must be a multiple of 3 replaced digits.  Assuming the answer
    // is under 1,000,000, that means there are exactly 3 repeated
    // digits.
    //
    // The final digit can't be in the repeating group, since half
    // of those numbers are even. Also, for 5 digit numbers, the
    // first digit can't be part of the repeating group, since the
    // given lower bound of 56,003 means the digit has to be 5..9.
    //
    // So for 5 digit numbers, the repeating group can only be the
    // middle 3: we have x***y, where (x+y) isn't divisible by 3,
    // x is in 6..9, and y is in {1,3,7,9}. That's 16 possibilities,
    // which we can search by hand:
    //      6***1: 3  6***7: 3
    //      7***1: 4  7***3: 4  7***7: 4  7***9: 5  8***3: 3
    //      8***9: 3
    //      9***1: 5  9***7: 7
    // (notice the group of 7 for 9***7 - close but no cigar!)
    //
    // We can something similar for 6 digit numbers as well.
    //   - There are three fixed and three repeating digits
    //   - The final digit is not repeating, so it is fixed
    //   - That leaves 5C2 = 10 places for the other 2 fixed digits
    //
    // So a reasonable algorithm would be:
    //  - For each of 1,000 3-digit combinations
    //    - For each of the 10 patterns of fixed/repeating digits
    //      - Generate the pattern using all 10 repeating digits,
    //        and count how many are prime.
    //      - If the answer is 8 or more, record the smallest prime
    //        in the group
    // - Find the minimum over all such smallest primes

    // These are the 10 possible places for the 2 fixed digits that
    // aren't in position 0 (units).
    let masks = [
        (2, 1), // ***abc
        (3, 1), // **a*bc
        (3, 2), // **ab*c
        (4, 1), // *a**bc
        (4, 2), // *a*b*c
        (4, 3), // *ab**c
        (5, 1), // a***bc
        (5, 2), // a**b*c
        (5, 3), // a*b**c
        (5, 4), // ab***c
    ];
    let powers10 = euler::POWERS_OF_10;

    let mut min = Best::new();
    for n in 0..1000 {
        let (a, b, c) = (n / 100, (n / 10) % 10, n % 10);

        // Quick optimization: if c, the final fixed digit, is 0, 2, 4, 5, 6, or 8,
        // then none of the generated numbers are going to be prime.
        if ![1, 3, 7, 9].contains(&c) {
            continue;
        }
        for (k2, k1) in masks {
            let fixed_digits = a * powers10[k1] + b * powers10[k2] + c;
            let mask = 111_111 - (powers10[k1] + powers10[k2] + 1);
            let mut count = 0;
            let mut first_prime = Best::new();
            for j in 0..10 {
                let num = fixed_digits + j * mask;
                if num > 100_000 && sieve.is_prime(num) {
                    count += 1;
                    first_prime.min(num);
                }
            }
            if count >= 8 {
                min.min(first_prime.unwrap());
            }
        }
    }
    min.unwrap()
}


pub fn p76() -> usize {
    // The number of ways to write a number as a sum of smaller numbers is
    // called the _partition function_ P(n), and has been studied extensively.
    // Hardy and Wright present the material in chapter 19; in particular,
    // section 19.10, which shows this recurrence:

    #[memoize]
    fn euler_p(n: usize) -> usize {
        if n <= 1 {
            return 1;
        }
        let mut sum: usize = 0;
        for k in 1..=n {
            let mut t = 0;
            let d1 = k * (3 * k - 1) / 2;
            if n >= d1 {
                // println!("{} => {}", n, n - d1);
                t += euler_p(n - d1);
            }
            let d2 = k * (3 * k + 1) / 2;
            if n >= d2 {
                // println!("{} => {}", n, n - d2);
                t += euler_p(n - d2);
            }

            if k % 2 == 1 {
                sum += t;
            } else {
                debug_assert!(sum > t);
                sum -= t;
            }
        }
        sum
    }

    debug_assert_eq!(euler_p(1), 1);
    debug_assert_eq!(euler_p(2), 2);

    // ...and the problem asks for the number of ways to partition a number
    // into 2 or more groups. That's easy, because there is only one way to
    // have a single group, so the difference is 1.
    euler_p(100) - 1
}

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
