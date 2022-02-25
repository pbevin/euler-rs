mod factors;
mod poker;
mod windows;

use std::iter::from_fn;
use std::ops::Deref;

pub use factors::*;
pub use windows::*;
pub use poker::*;

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
/// The naive algorithm would be to call .to_string() on the
/// number and check whether it's equal to its reverse:
/// ```
/// fn is_palindrome(num: i64) -> bool {
///     let string = num.to_string();
///     let reverse = string.chars().rev().collect::<String>();
///     string == reverse
/// }
/// ```
///
/// This method is faster: we count the number of digits in
/// the number, then pull half the digits off the right hand
/// side, constructing the reversed RHS as we go. After half
/// the digits, we check to see if the two halves are equal.
///
/// For problem 4, this takes the time taken for the release
/// build from 49ms to about 7ms.
pub fn is_palindrome(num: i64) -> bool {
    if num % 10 == 0 {
        // 0 is a palindrome, but anything else ending in 0 can't be.
        return num == 0;
    }

    let len = count_decimal_digits(num);

    let mut lhs = num;
    let mut rhs_reversed = 0;
    for _ in 0..len / 2 {
        let d = lhs % 10;
        lhs /= 10;
        rhs_reversed *= 10;
        rhs_reversed += d;
    }
    if len % 2 == 1 {
        lhs /= 10;
    }
    lhs == rhs_reversed
}

/// Counts the number of decimal digits in a number. Only works for positive numbers.
pub fn count_decimal_digits(num: i64) -> usize {
    // edge case:
    if num == 0 {
        return 1;
    }
    let mut n = num;
    let mut count = 0;
    while n > 0 {
        n /= 10;
        count += 1;
    }
    count
}

pub const POWERS_OF_10: [usize; 13] = [
    1,
    10,
    100,
    1000,
    10_000,
    100_000,
    1_000_000,
    10_000_000,
    100_000_000,
    1_000_000_000,
    10_000_000_000,
    100_000_000_000,
    1_000_000_000_000,
];

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

/// Keeps track of the best result so far in a problem or sub-problem.
/// For example:
///
/// ```
/// use euler::Best;
///
/// // Find the largest prime that is the sum of two numbers under 100:
/// let mut best = Best::new();
/// for a in 1..100 {
///     for b in 1..100 {
///         let n = a + b;
///         if primal::is_prime(n) {
///             best.max(n);
///         }
///     }
/// }
/// assert_eq!(*best, Some(197));
/// ```
pub struct Best<T> {
    value: Option<T>,
}

impl<T: Clone + PartialOrd> Default for Best<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Best<T>
where
    T: Clone,
    T: PartialOrd,
{
    pub fn new() -> Self {
        Self { value: None }
    }

    pub fn min(&mut self, t: T) {
        match &self.value {
            Some(current_best) if t >= *current_best => (),
            _ => self.value = Some(t),
        }
    }

    pub fn max(&mut self, t: T) {
        match &self.value {
            Some(current_best) if t <= *current_best => (),
            _ => self.value = Some(t),
        }
    }

    pub fn into_inner(self) -> Option<T> {
        self.value
    }
}

impl<T> Deref for Best<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

/// Handles a common case of keeping track simultaneously of a data value
/// and a count or size that should be optimized.  The value of this is
/// that comparison is by the count field alone; the data field doesn't
/// even have to be comparable.
///
/// Examples:
///
/// ```
/// use euler::CountOf;
///
/// let a = CountOf::new(51, "magic");
/// let b = CountOf::new(1, "squeamish");
/// let c = CountOf::new(1, "ossifrage");
/// assert!(a > b);
/// assert!(a > c);
/// assert!(b == c);
/// ```
#[derive(Clone)]
pub struct CountOf<T> {
    pub count: usize,
    pub data: T,
}

impl<T> CountOf<T> {
    pub fn new(count: usize, data: T) -> Self {
        Self { count, data }
    }
}

impl<T> Deref for CountOf<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> PartialEq for CountOf<T> {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl<T> Eq for CountOf<T> {}

impl<T> PartialOrd for CountOf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl<T> Ord for CountOf<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
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
        // for n in [
        //     9, 99, 999, 909, 101, 111, 121, 1001, 1111, 1221, 12321, 123321,
        // ] {
        //     assert!(is_palindrome(n), "{}", n);
        // }

        // for n in [91, 19, 100, 110, 123, 1000, 1002, 1232, 10_000, 10_110] {
        //     assert!(!is_palindrome(n), "{}", n);
        // }

        for num in 0..99_999 {
            assert_eq!(is_palindrome(num), is_palindrome_slow(num), "{}", num);
        }
    }

    fn is_palindrome_slow(num: i64) -> bool {
        let s1 = num.to_string();
        let rev = s1.chars().rev().collect::<String>();
        s1 == rev
    }

    #[test]
    fn test_count_digits() {
        for i in 0..10 {
            assert_eq!(count_decimal_digits(i), 1, "count_decimal_digits({})", i);
        }
        for i in 10..100 {
            assert_eq!(count_decimal_digits(i), 2, "count_decimal_digits({})", i);
        }
        for i in 100..1000 {
            assert_eq!(count_decimal_digits(i), 3, "count_decimal_digits({})", i);
        }
        for i in 1000..10_000 {
            assert_eq!(count_decimal_digits(i), 4, "count_decimal_digits({})", i);
        }

        assert_eq!(count_decimal_digits(1234567890), 10);
    }

    #[test]
    fn test_min_simple() {
        let mut best: Best<usize> = Best::new();
        assert_eq!(*best, None);
        best.min(42);
        assert_eq!(*best, Some(42));
        best.min(54);
        assert_eq!(*best, Some(42));
        best.min(31);
        assert_eq!(*best, Some(31));
    }

    #[test]
    fn test_min_with_value() {
        let mut best: Best<(usize, &'static str)> = Best::new();
        best.min((42, "forty two"));
        assert_eq!(*best, Some((42, "forty two")));
        best.min((42, "meaning of life"));
        assert_eq!(*best, Some((42, "forty two")));
        best.min((54, "six times nine"));
        assert_eq!(*best, Some((42, "forty two")));
        best.min((27, "three to the power three"));
        assert_eq!(*best, Some((27, "three to the power three")));
    }
}
