euler::solution!(p719, "Number Splitting", 128088830547982);

fn p719() -> usize {
    assert!(is_splittable(9, 81));
    assert!(is_splittable(82, 6724));
    assert!(is_splittable(91, 8281));
    assert!(is_splittable(99, 9801));

    // Not counting 1, 2, or 3 because the question says "...can be obtained by
    // splitting the decimal representation of n into 2 or more numbers..."
    (9..=1_000_000)
        // Optimization: splitting digits preserves the digit sum, so we need n == n^2 (mod 9).
        .filter(|&n| n % 9 <= 1)
        .filter(|&n| is_splittable(n, n * n))
        .map(|n| n * n)
        .sum()
}

// Can s be made from sums of the digits of n?
//
// The first thing I tried was putting the digits into a vec and
// trying splits from the left:
//    [9,8,0,1] => 9 + [8,0,1]
//                 98 + [0,1]
//                 980 + [1]
//                 9801 + []
// ...but that was rather slow.
//
// This approach is faster: work from the right side instead, and don't
// write out the digits explicitly:
//
//  (99, 9801) => (99 - 1, 980)
//                (99 - 1, 98)
//                (99 - 801, 9)
//                (99 - 9801, 0)
// ...of which the last two get thrown out because the LHS is negative.

fn is_splittable(s: usize, n: usize) -> bool {
    if s == n {
        return true;
    }
    if s > n {
        return false;
    }

    // b will iterate through powers of 10
    let mut b = 10;
    while b < n {
        let d = n % b;
        if s < d {
            return false;
        }
        if is_splittable(s - d, n / b) {
            return true;
        }
        b *= 10;
    }
    false
}
