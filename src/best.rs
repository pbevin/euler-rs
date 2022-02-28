use std::ops::Deref;

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
    use super::*;

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
