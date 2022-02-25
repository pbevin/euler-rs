mod check;

use std::time::Instant;

use euler::eval_poker_hand;
use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use euler::partitions3;
use euler::Best;
use euler::Card;
use euler::CountOf;
use euler::PokerHand;
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
    check!(p15(), "Lattice paths", 137846528820);
    check!(p50(), "Consecutive prime sum", 997651);
    check!(p51(), "Prime digit replacements", 121313);
    check!(p54(), "Poker hands", 376);

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

fn p6() -> u32 {
    fn square(n: u32) -> u32 {
        n * n
    }

    let sum_of_squares = (1..=100).map(square).sum::<u32>();
    let square_of_sum = square((1..=100).sum::<u32>());

    square_of_sum - sum_of_squares
}

fn p7() -> usize {
    StreamingSieve::nth_prime(10_001)
}

fn p8() -> u64 {
    let digits = include_str!("p8.txt")
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .map(|digit| digit.into())
        .collect::<Vec<_>>();

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

fn p12() -> usize {
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

fn p13() -> usize {
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
    let mut result = carry.into();
    while result < 1_000_000_000 {
        result *= 10;
        result += sum.pop().unwrap() as usize;
    }

    result
}

fn p14() -> usize {
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

fn p15() -> usize {
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

fn p50() -> usize {
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

fn p51() -> usize {
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

fn p54() -> usize {
    include_str!("p054_poker.txt")
        .lines()
        .map(deal)
        .filter(|(p1, p2)| eval_poker_hand(p1) > eval_poker_hand(p2))
        .count()
}

fn deal(line: &str) -> (PokerHand, PokerHand) {
    let cards: Vec<Card> = line
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let cards1: [Card; 5] = [cards[0], cards[1], cards[2], cards[3], cards[4]];
    let cards2: [Card; 5] = [cards[5], cards[6], cards[7], cards[8], cards[9]];

    (PokerHand::new(cards1), PokerHand::new(cards2))
}
