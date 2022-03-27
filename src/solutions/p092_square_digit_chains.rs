euler::solution!(p092, "Square digit chains", 8_581_146);

const N: usize = 10_000_000;

fn p092() -> usize {
    // Maximum square digit sum up to 10_000_000 is for 9_999_999,
    // which is 81 * 7 = 567. So we can make a lookup table from
    // square sum of digits to loop type.
    let types = gen_loop_types();
    let mut count = 0;
    for n in 2..N {
        let sum = square_digit_sum(n);
        if types[sum] == 89 {
            count += 1;
        }
    }
    count
}

fn gen_loop_types() -> [usize; 600] {
    let mut types = [0; 600];
    for (n, p) in types.iter_mut().enumerate() {
        *p = loop_type(n);
    }
    types
}

fn loop_type(mut n: usize) -> usize {
    while n != 0 && n != 1 && n != 89 {
        n = square_digit_sum(n);
    }
    n
}

fn square_digit_sum(mut n: usize) -> usize {
    let mut sum = 0;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        sum += d * d;
    }
    sum
}
