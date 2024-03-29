use std::iter::from_fn;

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

#[cfg(test)]
mod tests {
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
}
