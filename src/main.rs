use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use itertools::Itertools;
use owo_colors::OwoColorize;

fn main() {
    assert_eq!(p1(), 233168);
    assert_eq!(p2(), 4613732);
    assert_eq!(p3(), 6857);
    assert_eq!(p4(), 906609);
    assert_eq!(p5(), 232792560);
    assert_eq!(p6(), 25164150);

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
    (100_i64..1000).cartesian_product(100..1000)
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
