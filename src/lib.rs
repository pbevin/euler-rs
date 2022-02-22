mod factors;

pub use factors::factors;

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

pub fn is_palindrome(n: i64) -> bool {
    // Apparently, this code is "old fashioned" and
    // "should use iterators".
    let s = format!("{}", n);
    let s = s.as_bytes();
    let mut i = 0;
    let mut j = s.len() - 1;
    while i < j {
        if s[i] != s[j] {
            return false;
        }
        i += 1;
        j -= 1;
    }
    true
}
