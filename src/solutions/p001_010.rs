use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use euler::partitions3;
use itertools::Itertools;
use primal::Sieve;
use primal::StreamingSieve;

euler::solution!(p1, "Multiples of 3 or 4", 233168);
euler::solution!(p2, "Even fibonacci numbers", 4613732);
euler::solution!(p3, "Largest prime factor", 6857);
euler::solution!(p4, "Largest palindrome product", 906609);
euler::solution!(p5, "Smallest multiple", 232792560);
euler::solution!(p6, "Sum-square difference", 25164150);
euler::solution!(p7, "10,001st prime", 104743);
euler::solution!(p9, "Special pythagorean triplet", 31875000);
euler::solution!(p10, "Summation of primes", 142913828922);

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
