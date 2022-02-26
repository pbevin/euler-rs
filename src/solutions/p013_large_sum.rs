pub fn p13() -> usize {
    // Each entry in numbers[] is a vector of digits, least significant digit first.
    // This ordering makes it a little easier to do addition.
    //
    // The digit type is u16, which allows us to add 100 digits and a carry without
    // converting to a wider type.
    let numbers: Vec<Vec<u16>> = include_str!("p013.txt")
        .lines()
        .map(|line| {
            line.chars()
                .rev()
                .map(|ch| ch.to_digit(10).unwrap() as u16)
                .collect()
        })
        .collect();

    // All numbers are the same length.
    let mut sum = vec![];
    let mut carry = 0;
    for pos in 0..numbers[0].len() {
        let column_sum: u16 = numbers.iter().map(|num| num[pos]).sum();
        let total: u16 = carry + column_sum;
        let digit = total % 10;
        sum.push(digit);
        carry = total / 10;
    }

    // At this point, we have an unknown number of digits in `carry`.
    // Top it up to 10 digits by pulling from the end of `sum`.
    let mut result = carry.into();
    while result < 1_000_000_000 {
        result *= 10;
        result += sum.pop().unwrap() as usize;
    }

    result
}
