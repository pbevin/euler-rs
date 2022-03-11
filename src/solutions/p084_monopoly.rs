use itertools::Itertools;
use lazy_static::lazy_static;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;
use std::fs::File;
use std::io::BufWriter;

euler::solution!(p084, "Monopoly odds", 101524);

const DIE: usize = 4;

// array([ 0.03113803,  0.02152422,  0.01899769,  0.02185791,  0.02350777,
//         0.02993127,  0.02285359,  0.00876027,  0.02347011,  0.02330647,
//         0.0589642 ,  0.02735991,  0.02627461,  0.02385533,  0.02467375,
//         0.02918612,  0.02776751,  0.02571807,  0.02916517,  0.03071024,
//         0.02874826,  0.02830354,  0.01047697,  0.02738577,  0.03187795,
//         0.03063697,  0.02706511,  0.02679313,  0.02810737,  0.02591185,
//        -0.        ,  0.02686592,  0.02633846,  0.0237657 ,  0.0251047 ,
//         0.02445896,  0.00871503,  0.02202178,  0.02193107,  0.02646926])

// #[derive(Debug)]
// struct State {
//     square: usize,
//     num_doubles: usize,
//     ch_cards: [Card; 16],
//     cc_cards: [Card; 16],
// }

fn p084() -> usize {
    // let rng = &mut thread_rng();
    let mut counts = [0usize; 40];
    let mut moves = [[0usize; 40]; 40];
    // let mut state = State {
    //     square: GO,
    //     num_doubles: 0,
    //     ch_cards: shuffled(&CH_CARDS, rng),
    //     cc_cards: shuffled(&CC_CARDS, rng),
    // };

    #[allow(clippy::needless_range_loop)]
    for square in 0..40 {
        for num_doubles in 0..2 {
            for ch in CH_CARDS.iter() {
                for cc in CC_CARDS.iter() {
                    for d1 in 1..=DIE {
                        for d2 in 1..=DIE {
                            let next_square = sim(square, num_doubles, ch, cc, d1, d2);
                            counts[next_square] += 1;
                            moves[square][next_square] += 1;
                        }
                    }
                }
            }
        }
    }

    let temp_dir = tempdir().unwrap();
    {
        let f = File::create(temp_dir.path().join("trans.csv")).unwrap();
        let mut writer = BufWriter::new(f);

        for row in moves {
            writeln!(writer, "{}", row.iter().map(|n| n.to_string()).join(",")).unwrap();
        }
    }
    {
        let f = File::create(temp_dir.path().join("eigen.r")).unwrap();
        let mut writer = BufWriter::new(f);
        writeln!(writer, "A <- read.csv(\"trans.csv\", header = FALSE)").unwrap();
        writeln!(writer, "v = Re(eigen(t(A))$vectors[,1])").unwrap();
        writeln!(
            writer,
            "ix = sort(v / sum(v), index.return=TRUE, decreasing=TRUE)$ix"
        )
        .unwrap();
        writeln!(writer, "cat(ix[1:3] - 1)").unwrap();
    }
    // let output = Command::new("cat")
    //     .arg("trans.csv")
    let output = Command::new("Rscript")
        .arg("eigen.r")
        .current_dir(&temp_dir)
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&output.stdout);
    let v = s
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    match &v[..] {
        [a, b, c] => a * 10_000 + b * 100 + c,
        _ => unreachable!(),
    }
}

// let data = moves.concat();
// println!("A <- matrix(c({}), 40, 40, byrow=TRUE)", data.iter().map(|n| n.to_string()).join(","));

//     let mut v = moves
//         .map(|row| {
//             let sum = row.iter().copied().sum::<usize>() as f64;
//             row.map(|n| n as f64 / sum).to_vec()
//         })
//         .to_vec();
//     v.remove(G2J);
//     for row in &mut v {
//         row.remove(G2J);
//     }
//     let m = py_matrix(v).transpose();

//     let e = eigen(&m);
//     e.eigenvector.print();

//     let ss = e.eigenvector.col(0);
//     let sum = ss.iter().copied().sum::<f64>();
//     let ss = ss
//         .into_iter()
//         .map(|p| ((p / sum) * 10000.0).round() / 100.0)
//         .collect_vec();

//     println!("{:?}", ss);

//     // for i in 0..40 {
//     //     let s = v[i].iter().sum::<f64>();
//     //     println!("{:?}", v[i]);
//     //     assert!((s - 1.0).abs() < 0.00001, "row {i}: {s}");
//     // }

//     // for _ in 1..1_000_000 {
//     //     let start = state.square;
//     //     simulate(&mut state, &mut thread_rng());
//     //     counts[state.square] += 1;

//     //     moves[state.square + start * 40] += 1;
//     // }

//     // for j in 0..40 {
//     //     for i in 0..40 {
//     //         let txt = format!("{:>4}", moves[i + 40 * j]);
//     //         if i % 2 == 0 {
//     //             print!("{}", txt);
//     //         } else {
//     //             print!("{}", txt.yellow());
//     //         }
//     //     }
//     //     println!();
//     // }

//     // println!("{:?}", &counts[0..10]);
//     // println!("{:?}", &counts[10..20]);
//     // println!("{:?}", &counts[20..30]);
//     // println!("{:?}", &counts[30..40]);
//     // println!("{:?}", &state);

//     let top3 = counts
//         .iter()
//         .enumerate()
//         .map(|(i, n)| (Reverse(n), i))
//         .k_smallest(3)
//         .map(|(_, k)| k)
//         .collect_vec();
//     top3[0] * 10_000 + top3[1] * 100 + top3[2]
// }

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

// fn simulate(state: &mut State, rng: &mut impl Rng) {
//     let d1 = rng.gen_range(1..=DIE);
//     let d2 = rng.gen_range(1..=DIE);
//     if d1 == d2 {
//         state.num_doubles += 1;
//         if state.num_doubles == 3 {
//             state.num_doubles = 0;
//             state.square = JAIL;
//             return;
//         }
//     } else {
//         state.num_doubles = 0;
//     }

//     let roll = d1 + d2;
//     state.square = (state.square + roll) % 40;

//     if state.square == G2J {
//         state.square = JAIL;
//         return;
//     }

//     // Loop here because from CH3, you can draw a "Go back 3 squares" card
//     // and land on CC3.
//     while [CC1, CC2, CC3, CH1, CH2, CH3].contains(&state.square) {
//         let card = match state.square {
//             CC1 | CC2 | CC3 => draw_card(&mut state.cc_cards),
//             CH1 | CH2 | CH3 => draw_card(&mut state.ch_cards),
//             _ => panic!(),
//         };
//         match card {
//             Card::Other => return,
//             Card::Go(n) => state.square = n,
//             Card::Railroad => state.square = next_railroad(state.square),
//             Card::Utility => state.square = next_utility(state.square),
//             Card::BackThree => state.square -= 3,
//         }
//     }
// }

// fn shuffled<const N: usize>(cards: &[Card; N], rng: &mut impl Rng) -> [Card; N] {
//     let mut cards = *cards;
//     cards.shuffle(rng);
//     cards
// }

// fn draw_card<const N: usize>(cards: &mut [Card; N]) -> Card {
//     let card = cards[0];
//     cards.copy_within(1.., 0);
//     cards[N - 1] = card;

//     card
// }

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
