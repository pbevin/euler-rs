use itertools::Itertools;
use lazy_static::lazy_static;
use std::cmp::Reverse;
use std::fs;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

euler::solution!(p084, "Monopoly odds", 101524);

const DIE: usize = 4;


// Strategy: We could simulate this a lot of times, but I don't know how
// many times is "enough" to be reliable. So the more mathy answer is to
// build a matrix where the rows and columns represent squares, and each
// cell represents the probability of moving from R to C. We build it by
// going through each possible starting square and game state, and simulating
// one die roll to see where the player lands.
//
// Where the player lands depends on the following:
//   - The starting square
//   - What number each die landed on
//   - How many consecutive doubles have been rolled
//   - The top CC and CH cards
//
// This is a total of 40 * DIE * DIE * 2 * 16 * 16 = 368,640 (if dice are D6)
// or 163,840 for D4 dice, which is completely feasible. I suspect it would
// take at least 10 million simulations to get a reliable answer under Monte Carlo.
//
// There isn't a good numerical package for Rust as of March 2022, so I'm just
// shelling out to R for the eigenvector stuff.
fn p084() -> usize {
    let moves = build_stochastic_matrix();
    // let ps = stationary_probabilities(moves);

    let ps = stationary_probabilities(moves);
    let mut it = ps
        .iter()
        .map(|p| (p * 100_000.0) as i32)
        .enumerate()
        .sorted_by_key(|(_, p)| Reverse(*p))
        .map(|(i, _)| i)
        .take(3);
    let a = it.next().unwrap();
    let b = it.next().unwrap();
    let c = it.next().unwrap();
    a * 10_000 + b * 100 + c
}

fn build_stochastic_matrix() -> [[usize; 40]; 40] {
    let mut moves = [[0; 40]; 40];

    #[allow(clippy::needless_range_loop)]
    for square in 0..40 {
        for num_doubles in 0..2 {
            for ch in CH_CARDS.iter() {
                for cc in CC_CARDS.iter() {
                    for d1 in 1..=DIE {
                        for d2 in 1..=DIE {
                            let next_square = sim(square, num_doubles, ch, cc, d1, d2);
                            moves[square][next_square] += 1;
                        }
                    }
                }
            }
        }
    }
    moves
}


fn stationary_probabilities(moves: [[usize; 40]; 40]) -> [f64; 40] {
    let temp_dir = tempdir().unwrap();
    write_matrix(&moves, &temp_dir.path().join("trans.csv")).unwrap();
    fs::write(
        temp_dir.path().join("eigen.r"),
        include_bytes!("p084_eigen.r"),
    )
    .unwrap();
    let output = Command::new("Rscript")
        .arg("eigen.r")
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&output.stdout);
    let v = s
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<f64>>();
    if v.len() == 40 {
        let mut ps = [0.0; 40];
        for (i, p) in v.into_iter().enumerate() {
            ps[i] = p;
        }
        ps
    } else {
        panic!(
            "Stationary probability list is length {}, not 40: {v:?}",
            v.len()
        );
    }
}

fn  write_matrix<const N: usize>(m: &[[usize; N]; N], path: &Path) -> io::Result<()> {
        let f = File::create(path)?;
        let mut writer = BufWriter::new(f);

        for row in m {
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(writer, ",")?;
                }
                write!(writer, "{val}")?;
            }
            writeln!(writer)?;
        }

    Ok(())

}


fn sim(start: usize, num_doubles: usize, ch: &Card, cc: &Card, d1: usize, d2: usize) -> usize {
    if d1 == d2 && num_doubles == 2 {
        return JAIL;
    }
    let roll = d1 + d2;
    let square = {
        let square = (start + roll) % 40;
        // Special case: CH3 + back 3 = CC3
        if square == CH3 && matches!(ch, Card::BackThree) {
            CC3
        } else {
            square
        }
    };
    if square == G2J {
        return JAIL;
    }

    let card = match square {
        CH1 | CH2 | CH3 => Some(ch),
        CC1 | CC2 | CC3 => Some(cc),
        _ => None,
    };

    if let Some(card) = card {
        match card {
            Card::Other => square,
            Card::Go(dest) => *dest,
            Card::Railroad => next_railroad(square),
            Card::Utility => next_utility(square),
            Card::BackThree => square - 3,
        }
    } else {
        square
    }
}

fn next_railroad(square: usize) -> usize {
    match square {
        CH1 => R2,
        CH2 => R3,
        CH3 => R1,
        _ => unreachable!("Next railroad after {}", square),
    }
}

fn next_utility(square: usize) -> usize {
    match square {
        CH1 | CH3 => U1,
        CH2 => U2,
        _ => unreachable!("Next utility after {}", square),
    }
}

const GO: usize = 0;
const JAIL: usize = 10;
// Target of a Chance card
const C1: usize = 11;
// Target of a Chance card
const E3: usize = 24;
// Target of a Chance card
const H2: usize = 39;
// Railroad 1
const R1: usize = 5;
// Railroad 2
const R2: usize = 15;
// Railroad 3
const R3: usize = 25;
// Utility 1
const U1: usize = 12;
// Utility 2
const U2: usize = 28;
// Go to jail
const G2J: usize = 30;

// Chance 1
const CH1: usize = 7;
// Chance 2
const CH2: usize = 22;
// Chance 3
const CH3: usize = 36;

// Community Chest 1
const CC1: usize = 2;
// Community Chest 2
const CC2: usize = 17;
// Community Chest 3
const CC3: usize = 33;

lazy_static! {
    static ref CH_CARDS: [Card; 16] = gen_chance();
    static ref CC_CARDS: [Card; 16] = gen_cc();
}

fn gen_chance() -> [Card; 16] {
    let mut cards = [Card::Other; 16];
    cards[0] = Card::Go(GO);
    cards[1] = Card::Go(JAIL);
    cards[2] = Card::Go(C1);
    cards[3] = Card::Go(E3);
    cards[4] = Card::Go(H2);
    cards[5] = Card::Go(R1);
    cards[6] = Card::Railroad;
    cards[7] = Card::Railroad;
    cards[8] = Card::Utility;
    cards[9] = Card::BackThree;

    cards
}

fn gen_cc() -> [Card; 16] {
    let mut cards = [Card::Other; 16];
    cards[0] = Card::Go(GO);
    cards[1] = Card::Go(JAIL);

    cards
}

#[derive(Debug, Clone, Copy)]
enum Card {
    Other,
    Go(usize),
    Railroad,
    Utility,
    BackThree,
}
