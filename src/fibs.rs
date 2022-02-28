pub fn fibs(a: i64, b: i64) -> Fibs {
    Fibs { a, b }
}

pub struct Fibs {
    a: i64,
    b: i64,
}

impl Iterator for Fibs {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.a.checked_add(self.b)?;
        let ans = self.a;
        self.a = self.b;
        self.b = c;
        Some(ans)
    }
}
