use std::time::Duration;
use std::time::Instant;

use euler::factors;
use euler::fibs;
use euler::is_palindrome;
use euler::partitions3;
use itertools::Itertools;
use owo_colors::OwoColorize;
use primal::Sieve;
use primal::StreamingSieve;

macro_rules! check {
    ($fn:expr, $title:expr, $ans:expr) => {
        let start = Instant::now();
        let x: i64 = $fn;
        let duration = start.elapsed();
        show_check_result(stringify!($fn), $title, x, $ans, duration);
    };
}

fn show_check_result(expr: &str, title: &str, x: i64, ans: i64, time_taken: Duration) {
    if x == ans {
        let millis = time_taken.as_millis();
        let timing = if millis == 0 {
            String::new()
        } else {
            let mut text = format!(" {}ms", millis);
            if millis < 1000 {
                text = format!("{}", text.yellow());
            } else {
                text = format!("{}", text.red());
            }
            text
        };
        println!("{} {} {}{}", "🗸".green(), expr, title, timing);
    } else {
        println!(
            "❌{} = {} (should be {})",
            expr,
            x.to_string().red(),
            ans.to_string().green()
        );
    }
}

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
