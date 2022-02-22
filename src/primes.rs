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
