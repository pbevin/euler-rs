use fixedbitset::FixedBitSet;
use itertools::Itertools;
use lazy_static::lazy_static;

pub fn all_primes() -> AllPrimes {
    AllPrimes { seen: vec![] }
}

pub struct AllPrimes {
    seen: Vec<i64>,
}

impl Iterator for AllPrimes {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let next_prime = match self.seen.len() {
            0 => 2,
            1 => 3,
            2 => 5,
            3 => 7,
            4 => 11,
            5 => 13,
            _ => {
                let mut n = *self.seen.last().unwrap();
                loop {
                    if n % 6 == 1 {
                        n += 4;
                    } else {
                        n += 2;
                    }
                    if self.seen.iter().all(|&p| n % p != 0) {
                        break;
                    }
                }
                n
            }
        };
        self.seen.push(next_prime);
        Some(next_prime)
    }
}

pub fn primes_upto(n: usize) -> impl Iterator<Item = i64> {
    [2, 3, 5].into_iter().chain(Sieve30::new(n))
}

#[derive(Clone, Copy)]
struct Residue {
    index: usize,
    delta: usize,
}

const RESIDUES30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
lazy_static! {
    static ref DELTA30: [usize; 900] = build_deltas();
    static ref Z30: [Option<Residue>; 30] = build_residues();
}

fn build_residues() -> [Option<Residue>; 30] {
    let mut z30 = [None; 30];

    let rs = RESIDUES30.into_iter().chain(std::iter::once(31));
    for (i, (a, b)) in rs.tuple_windows().enumerate() {
        z30[a].replace(Residue {
            index: i,
            delta: b - a,
        });
    }

    z30
}

fn build_deltas() -> [usize; 900] {
    let mut deltas = [0; 900];
    for j in RESIDUES30 {
        for i in RESIDUES30 {
            let n = j + 30;
            let p = i + 30;
            let n2 = npp_slow(n, p);
            let delta = (n2 - n) / p;
            deltas[i + j * 30] = delta;
        }
    }
    deltas
}

fn npp_slow(mut n: usize, p: usize) -> usize {
    loop {
        n += p;
        if [1, 7, 11, 13, 17, 19, 23, 29].contains(&(n % 30)) {
            return n;
        }
    }
}

struct Sieve30 {
    max: usize,
    bits: FixedBitSet,
}

impl Sieve30 {
    fn new(max: usize) -> Self {
        let mut bits = FixedBitSet::with_capacity(max);
        bits.insert_range(1..);
        Self { max, bits }
    }

    fn to_pos(p: usize) -> usize {
        let n = Z30[p % 30].map(|r| r.index).unwrap();
        (p / 30) * 8 + n
    }

    fn from_pos(i: usize) -> usize {
        let r = RESIDUES30[i % 8];
        30 * (i / 8) + r
    }

    fn first(&self) -> Option<usize> {
        self.bits.ones().next().map(Self::from_pos)
    }

    fn len(&self) -> usize {
        self.max
    }

    fn mark(&mut self, p: usize) {
        self.bits.set(Self::to_pos(p), false);
    }

    fn next_possible_prime(n: usize, p: usize) -> usize {
        let i = p % 30;
        let j = n % 30;
        let delta = DELTA30[i + 30 * j];
        debug_assert!(delta != 0);
        n + p * delta
    }
}

impl Iterator for Sieve30 {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let p = self.first()?;
        let max = self.len();
        if p > max {
            return None;
        }
        self.mark(p);
        let mut n = p * p;
        while n < max {
            self.mark(n);
            n = Self::next_possible_prime(n, p);
        }

        Some(p as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_primes_upto_20() {
        let xs = primes_upto(20).collect::<Vec<i64>>();
        assert_eq!(xs, [2, 3, 5, 7, 11, 13, 17, 19]);
    }

    #[test]
    fn count_primes_upto_1000() {
        assert_eq!(primes_upto(1000).count(), 168);
    }

    #[test]
    fn count_primes_upto_10k() {
        assert_eq!(primes_upto(10_000).count(), 1229);
    }

    #[test]
    fn next_possible_prime() {
        for p in [7, 11, 13, 17, 19, 23, 29, 31] {
            // n^2 % 30 is 1 or 19
            // let mut n = p * p;
            let mut n = 61;
            let mut ps1 = vec![n];
            let mut ps2 = vec![n];
            let mut ps3 = vec![];
            for _ in 1..30 {
                n = npp_slow(n, p);
                ps1.push(n);
            }
            // n = p * p;
            let mut n = 61;
            for _ in 1..30 {
                let n2 = Sieve30::next_possible_prime(n, p);
                let delta = n2 - n;
                n = n2;
                ps2.push(n);
                ps3.push(delta / p);
            }
            assert_eq!(ps1, ps2);
            println!("{:4} {:3}: {:?}", p, (p * p) % 30, ps3);
        }
    }

    fn npp_slow(mut n: usize, p: usize) -> usize {
        loop {
            n += p;
            if [1, 7, 11, 13, 17, 19, 23, 29].contains(&(n % 30)) {
                return n;
            }
        }
    }
}
