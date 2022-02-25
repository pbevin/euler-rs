mod factors;
mod windows;

use std::iter::from_fn;
use std::ops::Deref;

pub use factors::*;
pub use windows::*;

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

/// Determines whether a number is a palindrome in decimal.
///
/// We pull decimal digits off the RHS of the number, and
/// use them to build another number in reverse:
///
/// N        R
/// 123456   0
/// 12345    6
/// 1234    65
/// 123    654
/// 12    6543
/// 1    65432
/// 0   654321
///
/// The trick is that we can interrupt the loop as
/// soon as N >= R, which happens about half way
/// through the process. If the comparison is ==, then
/// the number is a palindrome; if it's >, then it's not.
///
/// Compared to the naive method (call to_string() and
/// compare the string with its reversal), this takes
/// the release build from 38ms to 5ms on problem 4.
pub fn is_palindrome(num: i64) -> bool {
    let mut n = num;
    let mut reversed = 0;
    while n > 0 {
        let d = n % 10;
        n /= 10;
        reversed *= 10;
        reversed += d;
        match reversed.cmp(&n) {
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => return false,
        }
    }

    num == reversed
}

/// Iterator over all triples (a, b, c) where a + b + c == n and
/// a < b < c.
pub fn partitions3(n: i64) -> impl Iterator<Item = (i64, i64, i64)> {
    let mut a = 1;
    let mut b = 2;

    from_fn(move || {
        let c = n - b - a;
        if c <= b {
            return None;
        }
        let triple = (a, b, c);
        if b + 1 < c - 1 {
            // e.g., (1, 3, 7) -> (1, 4, 6)
            b += 1;
        } else {
            // e.g., (1, 4, 6) -> (2, 3, 5)
            a += 1;
            b = a + 1;
        }
        Some(triple)
    })
}

#[derive(Default)]
pub struct Min<T> {
    value: Option<T>,
}

impl<T> Min<T>
where
    T: Clone,
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn push(&mut self, t: T) {
        match &self.value {
            Some(current_best) if t >= *current_best => (),
            _ => self.value = Some(t),
        }
    }
}

impl<T> Deref for Min<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    use super::*;

    #[test]
    fn test_partition3() {
        let mut p5 = partitions3(5);
        assert!(p5.next().is_none());

        let mut p6 = partitions3(6);
        assert_eq!(p6.next().unwrap(), (1, 2, 3));
        assert!(p6.next().is_none());

        let mut p7 = partitions3(7);
        assert_eq!(p7.next().unwrap(), (1, 2, 4));
        assert!(p7.next().is_none());

        let mut p8 = partitions3(8);
        assert_eq!(p8.next().unwrap(), (1, 2, 5));
        assert_eq!(p8.next().unwrap(), (1, 3, 4));
        assert!(p8.next().is_none());

        let mut p11 = partitions3(11);
        assert_eq!(p11.next().unwrap(), (1, 2, 8));
        assert_eq!(p11.next().unwrap(), (1, 3, 7));
        assert_eq!(p11.next().unwrap(), (1, 4, 6));
        assert_eq!(p11.next().unwrap(), (2, 3, 6));
        assert_eq!(p11.next().unwrap(), (2, 4, 5));
        assert!(p11.next().is_none());
    }

    #[test]
    fn test_is_palindrome() {
        for n in [
            9, 99, 999, 909, 101, 111, 121, 1001, 1111, 1221, 12321, 123321,
        ] {
            assert!(is_palindrome(n), "{}", n);
        }

        for n in [91, 19, 100, 110, 123, 1000, 1002, 1232] {
            assert!(!is_palindrome(n), "{}", n);
        }
    }
}
