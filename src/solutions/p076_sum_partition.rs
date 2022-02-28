use memoize::memoize;

euler::solution!(p76, "Counting summations", 190569291);

pub fn p76() -> usize {
    // The number of ways to write a number as a sum of smaller numbers is
    // called the _partition function_ P(n), and has been studied extensively.
    // Hardy and Wright present the material in chapter 19; in particular,
    // section 19.10, which shows this recurrence:

    #[memoize]
    fn euler_p(n: usize) -> usize {
        if n <= 1 {
            return 1;
        }
        let mut sum: usize = 0;
        for k in 1..=n {
            let mut t = 0;
            let d1 = k * (3 * k - 1) / 2;
            if n >= d1 {
                // println!("{} => {}", n, n - d1);
                t += euler_p(n - d1);
            }
            let d2 = k * (3 * k + 1) / 2;
            if n >= d2 {
                // println!("{} => {}", n, n - d2);
                t += euler_p(n - d2);
            }

            if k % 2 == 1 {
                sum += t;
            } else {
                debug_assert!(sum > t);
                sum -= t;
            }
        }
        sum
    }

    debug_assert_eq!(euler_p(1), 1);
    debug_assert_eq!(euler_p(2), 2);

    // ...and the problem asks for the number of ways to partition a number
    // into 2 or more groups. That's easy, because there is only one way to
    // have a single group, so the difference is 1.
    euler_p(100) - 1
}
