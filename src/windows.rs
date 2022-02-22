pub fn windows<T>(slice: &'_ [T], n: usize) -> Windows<T> {
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
