use euler::factors;
use euler::fibs;
use owo_colors::OwoColorize;

fn main() {
    assert_eq!(p1(), 233168);
    assert_eq!(p2(), 4613732);
    assert_eq!(p3(), 6857);

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
