euler::solution!(p089, "Roman numerals", 743);

fn p089() -> usize {
    include_str!("p089_roman.txt").lines().map(saving).sum()
}

fn saving(input: &str) -> usize {
    let n = parse_roman(input).unwrap();
    let best = emit_roman(n);
    if input.len() < best.len() {
        panic!("input = {input:?}, best = {best:?}");
    }
    input.len() - best.len()
}

const ROMAN: &[(u16, &str)] = &[
    (1000, "M"),
    (900, "CM"),
    (500, "D"),
    (400, "CD"),
    (100, "C"),
    (90, "XC"),
    (50, "L"),
    (40, "XL"),
    (10, "X"),
    (9, "IX"),
    (5, "V"),
    (4, "IV"),
    (1, "I"),
];

fn parse_roman(input: &str) -> Option<u16> {
    let mut input = input;
    let mut n = 0;

    'outer: while !input.is_empty() {
        for (k, s) in ROMAN {
            if input.starts_with(s) {
                n += k;
                input = &input[s.len()..];
                continue 'outer;
            }
        }
        panic!("Failed to parse roman numeral at {input:?}");
    }
    Some(n)

}

fn emit_roman(n: u16) -> String {
    let mut n = n;
    let mut output = String::new();
    for (k, s) in ROMAN {
        while n >= *k {
            n -= k;
            output.push_str(s);
        }
    }
    output
}
