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

#[cfg(test)]
mod tests {
    use super::*;

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
}
