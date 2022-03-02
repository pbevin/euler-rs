// Find the unique positive integer whose square has the form 1_2_3_4_5_6_7_8_9_0,
// where each “_” is a single digit.

use euler::isqrt;

euler::solution!(p206, "Concealed square", 1389019170);

// The square must end in 900, so n must end in 30 or 70.
fn p206() -> usize {
    let min2 = 1020304050607080900;
    let max2 = 1929394959697989990;

    let min = isqrt(min2 / 10_000);
    let max = isqrt(max2 / 10_000);

    for i in min..max {
        let i1 = i * 100 + 30;
        let i2 = i * 100 + 70;

        if is_solution(i1 * i1) {
            return i1;
        }
        if is_solution(i2 * i2) {
            return i2;
        }
    }

    panic!("Not found")
}

macro_rules! next_digit_is {
    ($n:ident, $d:expr) => {
        if $n % 10 != $d {
            return false;
        }
        $n /= 100;
    };
}
// Does n have the form 1_2_3_4_5_6_7_8_9_0?
fn is_solution(n: usize) -> bool {
    let mut n = n / 10_000;
    next_digit_is!(n, 8);
    next_digit_is!(n, 7);
    next_digit_is!(n, 6);
    next_digit_is!(n, 5);
    next_digit_is!(n, 4);
    next_digit_is!(n, 3);
    next_digit_is!(n, 2);
    n == 1
}
