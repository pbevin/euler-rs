
pub fn p8() -> u64 {
    let digits = include_str!("p008.txt")
        .bytes()
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .map(|digit| digit.into())
        .collect::<Vec<_>>();

    windows(&digits, 13)
        .map(|xs| xs.iter().product())
        .max()
        .unwrap()
}

// Given an array slice and a window size N, returns all size-N windows
// as slices of the original.
fn windows<T>(slice: &'_ [T], n: usize) -> Windows<T> {
    Windows { slice, n, i: 0 }
}

pub struct Windows<'a, T> {
    slice: &'a [T],
    i: usize,
    n: usize,
}

impl<'a, T> Iterator for Windows<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.i;
        let n = self.n;
        let slice = self.slice;
        if i + n > slice.len() {
            None
        } else {
            let s = &slice[i..i + n];
            self.i += 1;
            Some(s)
        }
    }
}
