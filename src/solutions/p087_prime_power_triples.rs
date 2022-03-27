use fixedbitset::FixedBitSet;
use primal::Primes;

euler::solution!(p087, "Prime power triples", 1097343);

fn p087() -> usize {
    let max = 50_000_000;
    let (squares, cubes, fourths) = gen_lists(max);
    let mut hits = FixedBitSet::with_capacity(max);
    for s in &squares {
        for c in &cubes {
            for f in &fourths {
                let n = s + c + f;
                if n < max {
                    hits.insert(s + c + f);
                }
            }
        }
    }

    hits.count_ones(..)
}

fn gen_lists(max: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut squares = vec![];
    let mut cubes = vec![];
    let mut fourths = vec![];
    for p in Primes::all().take_while(|p| p * p < max) {
        let s = p * p;
        squares.push(s);
        let c = s * p;
        if c < max {
            cubes.push(c);
        }
        let f = s * s;
        if f < max {
            fourths.push(f);
        }
    }
    (squares, cubes, fourths)
}
