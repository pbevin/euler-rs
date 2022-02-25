use std::fmt::Display;
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn to_char(&self) -> char {
        match self {
            Suit::Clubs => 'C',
            Suit::Diamonds => 'D',
            Suit::Hearts => 'H',
            Suit::Spades => 'S',
        }
    }

    pub fn from_char(ch: char) -> Option<Self> {
        match ch {
            'C' => Some(Suit::Clubs),
            'D' => Some(Suit::Diamonds),
            'H' => Some(Suit::Hearts),
            'S' => Some(Suit::Spades),
            _ => None,
        }
    }
}

type Rank = u8;

/// The rank and suit of a Poker card.
///
/// The rank is a u8; 2-10 are pip cards, 11 is Jack, 12 is Queen, 13 is King,
/// and 14 is Ace.
///
/// Cards are compared by rank only.
#[derive(Debug, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.rank.partial_cmp(&other.rank)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Eq for Card {}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank.cmp(&other.rank)
    }
}

#[derive(Debug)]
pub struct BadCardSpec;

impl FromStr for Card {
    type Err = BadCardSpec;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.chars();
        let r = it.next().ok_or(BadCardSpec)?;
        let s = it.next().ok_or(BadCardSpec)?;

        Ok(Card {
            rank: "23456789TJQKA"
                .chars()
                .position(|ch| ch == r)
                .map(|pos| pos as u8 + 2)
                .ok_or(BadCardSpec)?,
            suit: Suit::from_char(s).ok_or(BadCardSpec)?,
        })
    }
}

#[derive(Debug)]
pub struct PokerHand {
    cards: [Card; 5],
}

impl PokerHand {
    pub fn new(mut cards: [Card; 5]) -> Self {
        cards.sort_unstable();
        Self { cards }
    }

    pub fn ranks(&self) -> [Rank; 5] {
        let [c1, c2, c3, c4, c5] = &self.cards;
        [c1.rank, c2.rank, c3.rank, c4.rank, c5.rank]
    }

    pub fn suits(&self) -> [Suit; 5] {
        let [c1, c2, c3, c4, c5] = &self.cards;
        [c1.suit, c2.suit, c3.suit, c4.suit, c5.suit]
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = "23456789TJQKA".chars().nth(self.rank as usize).unwrap();
        let suit = self.suit.to_char();
        write!(f, "{}{}", rank, suit)
    }
}

#[derive(PartialEq, PartialOrd)]
pub enum Rating {
    HighCard(Rank),
    OnePair(Rank),
    TwoPairs(Rank, Rank),
    ThreeKind(Rank),
    Straight(Rank),
    Flush(Rank),
    FullHouse(Rank, Rank),
    FourKind(Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

pub fn eval_poker_hand(hand: &PokerHand) -> Rating {
    let ranks = hand.ranks();
    let suits = hand.suits();
    if all_same(&suits) && consecutive(&ranks) {
        if ranks[0] == 10 {
            Rating::RoyalFlush
        } else {
            Rating::StraightFlush(ranks[4])
        }
    } else if all_same(&ranks[0..4]) || all_same(&ranks[1..5]) {
        Rating::FourKind(ranks[3])
    } else if all_same(&ranks[0..2])
        && all_same(&ranks[3..5])
        && (ranks[2] == ranks[1] || ranks[2] == ranks[3])
    {
        Rating::FullHouse(ranks[4], ranks[0])
    } else if all_same(&suits) {
        Rating::Flush(ranks[4])
    } else if consecutive(&ranks) {
        Rating::Straight(ranks[4])
    } else if all_same(&ranks[0..3]) || all_same(&ranks[1..4]) || all_same(&ranks[2..5]) {
        Rating::ThreeKind(ranks[2])
    } else {
        let pairs = all_pairs(&ranks);
        match pairs.len() {
            2 => Rating::TwoPairs(pairs[0], pairs[1]),
            1 => Rating::OnePair(pairs[0]),
            0 => Rating::HighCard(ranks[4]),
            _ => unreachable!()
        }
    }
}

fn all_same<T: PartialEq>(values: &[T]) -> bool {
    values.iter().tuple_windows().all(|(x, y)| x == y)
}

fn consecutive(ranks: &[Rank]) -> bool {
    ranks.iter().tuple_windows().all(|(x, y)| x + 1 == *y)
}

fn all_pairs(ranks: &[Rank; 5]) -> Vec<Rank> {
    ranks
        .iter()
        .rev()
        .copied()
        .tuple_windows()
        .filter_map(|(x, y)| if x == y { Some(x) } else { None })
        .collect()
}
