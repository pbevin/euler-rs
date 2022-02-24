mod check;

use std::time::Instant;

use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use euler::partitions3;
use itertools::Itertools;
use owo_colors::OwoColorize;
use primal::Sieve;
use primal::StreamingSieve;

fn main() {
    check!(p1(), "Multiples of 3 or 4", 233168);
    check!(p2(), "Even fibonacci numbers", 4613732);
    check!(p3(), "Largest prime factor", 6857);
    check!(p4(), "Largest palindrome product", 906609);
    check!(p5(), "Smallest multiple", 232792560);
    check!(p6(), "Sum-square difference", 25164150);
    check!(p7(), "10,001st prime", 104743);
    check!(p8(), "Largest product in a series", 23514624000);
    check!(p9(), "Special pythagorean triplet", 31875000);
    check!(p10(), "Summation of primes", 142913828922);
    check!(p11(), "Largest product in a grid", 70600674);
    check!(p12(), "Highly divisible triangular number", 76576500);
    check!(p13(), "Large sum", 5537376230);
    check!(p14(), "Longest Collatz sequence", 837799);

    println!("{} All good", "🗸".green());
}

fn p1() -> i64 {
    (1..1000).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}

fn p2() -> i64 {
    fibs(1, 2)
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum()
}

fn p3() -> i64 {
    factors(600851475143).last().unwrap()
}

fn p4() -> i64 {
    (100_i64..1000)
        .cartesian_product(100..1000)
        .map(|(a, b)| a * b)
        .filter(|n| is_palindrome(*n))
        .max()
        .unwrap()
}

fn p5() -> i64 {
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

fn p6() -> i64 {
    let square = |n: i64| n * n;

    let sum_of_squares: i64 = (1..=100).map(square).sum();
    let square_of_sum = square((1..=100).sum());

    square_of_sum - sum_of_squares
}

fn p7() -> i64 {
    StreamingSieve::nth_prime(10_001) as i64
}

fn p8() -> i64 {
    let digits = include_str!("p8.txt")
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .map(|digit| digit as i64)
        .collect::<Vec<i64>>();

    euler::windows(&digits, 13)
        .map(|xs| xs.iter().product())
        .max()
        .unwrap()
}

fn p9() -> i64 {
    partitions3(1000)
        .filter(|(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .max()
        .unwrap()
}

fn p10() -> i64 {
    let limit = 2_000_000;
    let sum: usize = Sieve::new(limit)
        .primes_from(2)
        .filter(move |&p| p < limit)
        .sum();
    sum.try_into().unwrap()
}

fn p11() -> i64 {
    let size = 20;
    let grid = include_str!("p11.txt")
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i64>>();

    debug_assert_eq!(grid.len(), size * size);

    let hor = max_4prod(&grid, size, |x, y| (x, y));
    let ver = max_4prod(&grid, size, |x, y| (y, x));
    let d1a = max_4prod(&grid, size, |x, y| (x + y, y));
    let d1b = max_4prod(&grid, size, |x, y| (x, x + y));
    let d2a = max_4prod(&grid, size, |x, y| (size as isize - 1 - x - y, y));
    let d2b = max_4prod(&grid, size, |x, y| (size as isize - 1 - x - y, x));

    [hor, ver, d1a, d1b, d2a, d2b].into_iter().max().unwrap()
}

fn max_4prod<F>(grid: &[i64], size: usize, f: F) -> i64
where
    F: Fn(isize, isize) -> (isize, isize),
{
    let isize = size as isize;

    (0..)
        .map(|j| {
            (0..)
                .map(|i| f(i, j))
                .take_while(|(x, y)| (0..isize).contains(x) && (0..isize).contains(y))
                .map(|(x, y)| {
                    let x: usize = x.try_into().unwrap();
                    let y: usize = y.try_into().unwrap();
                    grid[x + y * size]
                })
                .tuple_windows()
                .map(|(a, b, c, d)| a * b * c * d)
                .max()
        })
        .take_while(|max| max.is_some())
        .max()
        .unwrap()
        .unwrap()
}

fn p12() -> i64 {
    // Maps prime numbers to count of factors

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
            return (k * (k + 1) / 2) as i64;
        }
    }
    unreachable!()
}

fn p13() -> i64 {
    // Each entry in numbers[] is a vector of digits, least significant digit first.
    // This ordering makes it a little easier to do addition.
    //
    // The digit type is u16, which allows us to add 100 digits and a carry without
    // converting to a wider type.
    let numbers: Vec<Vec<u16>> = include_str!("p13.txt")
        .lines()
        .map(|line| {
            line.chars()
                .rev()
                .map(|ch| ch.to_digit(10).unwrap() as u16)
                .collect()
        })
        .collect();

    // All numbers are the same length.
    let mut sum = vec![];
    let mut carry = 0;
    for pos in 0..numbers[0].len() {
        let column_sum: u16 = numbers.iter().map(|num| num[pos]).sum();
        let total: u16 = carry + column_sum;
        let digit = total % 10;
        sum.push(digit);
        carry = total / 10;
    }

    // At this point, we have an unknown number of digits in `carry`.
    // Top it up to 10 digits by pulling from the end of `sum`.
    let mut result = carry as i64;
    while result < 1_000_000_000 {
        result *= 10;
        result += sum.pop().unwrap() as i64;
    }

    result
}

fn p14() -> i64 {
    let upto = 1_000_000;
    let mut lengths = vec![0usize; upto];
    lengths[1] = 1;
    for i in 1..upto {
        calc_collatz(i, &mut lengths);
    }

    lengths[..upto]
        .iter()
        .enumerate()
        .max_by_key(|(_, n)| **n)
        .unwrap()
        .0 as i64
}

fn calc_collatz(i: usize, lengths: &mut [usize]) -> usize {
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
