use euler::Best;
use primal::Sieve;

euler::solution!(p51, "Prime digit replacements", 121313);

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
