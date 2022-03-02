euler::solution!(p097, "Large non-Mersenne prime", 8739992577);

// This is just the usual binary exponentiation algorithm, done in
// Z_n where n = 10^10.
//
// I'm using a special mul() function because the modulus is 34 bits wide,
// meaning that the product of two numbers will likely overflow a u64.

const MODULUS: u64 = 10_000_000_000;

fn p097() -> u64 {
    let n = binary_exp(2, 7830457);
    1 + mul(n, 28433)
}

fn binary_exp(mut n: u64, mut k: u64) -> u64 {
    let mut m = 1;
    while k > 1 {
        if k % 2 == 1 {
            m = mul(m, n);
        }
        n = mul(n, n);
        k /= 2;
    }
    mul(m, n)
}

fn mul(m: u64, n: u64) -> u64 {
    let m = m as u128;
    let n = n as u128;
    let modulus = MODULUS as u128;
    let result = (m * n) % modulus;
    result.try_into().unwrap()
}
