mod check;
mod solutions;

use owo_colors::OwoColorize;
use std::time::Instant;

use solutions::*;

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
    check!(p76(), "Counting summations", 190569291);
    check!(p78(), "Coin partitions", 55374);
    check!(p81(), "Path sum 2 ways", 427337);
    check!(p82(), "Path sum 3 ways", 260324);
    check!(p83(), "Path sum 4 ways", 425185);

    println!("{} All good", "🗸".green());
}
