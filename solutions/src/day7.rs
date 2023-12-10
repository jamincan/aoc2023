use anyhow::{bail, Context, Result};

use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 7);

solution!(INPUT, pt1, pt2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse_card<const PART: u8>(input: char) -> Result<Card> {
    assert!(PART >= 1 && PART <= 2);

    use Card::*;

    Ok(match input {
        '2' => Two,
        '3' => Three,
        '4' => Four,
        '5' => Five,
        '6' => Six,
        '7' => Seven,
        '8' => Eight,
        '9' => Nine,
        'T' => Ten,
        'J' if PART == 1 => Jack,
        'J' if PART == 2 => Joker,
        'Q' => Queen,
        'K' => King,
        'A' => Ace,
        _ => bail!("invalid input '{input}'"),
    })
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: i64,
}

impl Hand {
    pub fn rank(&self, other: &Hand) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        let self_type = self.best_hand();
        let other_type = other.best_hand();
        match self_type.cmp(&other_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            result => result,
        }
    }

    fn best_hand(&self) -> HandType {
        use HandType::*;

        let Hand { cards, .. } = *self;
        let base_hand: Vec<Card> = cards
            .into_iter()
            .filter(|card| *card != Card::Joker)
            .collect();
        let joker_count = 5 - base_hand.len();
        let base_hand_type = HandType::from(&base_hand[..]);
        match (base_hand_type, joker_count) {
            (kind, 0) => kind,
            (FourKind, 1) => FiveKind,
            (ThreeKind, 2) => FiveKind,
            (ThreeKind, 1) => FourKind,
            (TwoPair, 1) => FullHouse,
            (Pair, 1) => ThreeKind,
            (Pair, 2) => FourKind,
            (Pair, 3) => FiveKind,
            (HighCard, 1) => Pair,
            (HighCard, 2) => ThreeKind,
            (HighCard, 3) => FourKind,
            (HighCard, 4) => FiveKind,
            (_, 5) => FiveKind,
            _ => unreachable!("max 5 cards"),
        }
    }
}

fn parse_hand<const PART: u8>(input: &str) -> Result<Hand> {
    let (cards, bid) = input
        .trim()
        .split_once(char::is_whitespace)
        .context("unable to split hand from bid")?;
    let cards: Vec<Card> = cards
        .trim()
        .chars()
        .map(|ch| parse_card::<PART>(ch))
        .collect::<Result<_>>()?;
    if cards.len() != 5 {
        bail!("hand only has {} cards", input.len())
    }
    let cards: [Card; 5] = cards.try_into().expect("vec has 5 cards");

    let bid = bid.trim().parse()?;
    Ok(Hand { cards, bid })
}

fn parse_hands<const PART: u8>(input: &str) -> Result<Vec<Hand>> {
    let hands = input.trim().lines();
    let hands = hands.map(parse_hand::<PART>).collect::<Result<_>>()?;
    Ok(hands)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<&[Card]> for HandType {
    fn from(cards: &[Card]) -> Self {
        use HandType::*;

        // Get a count of each card in the hand, using an array as a simple map
        let mut counts = [0; 14];
        for card in cards.iter().copied() {
            let idx = card as usize;
            counts[idx] += 1;
        }

        // Sort the counts from high to low, which allows hand type to be determined from highest two counts
        counts.sort_unstable_by(|a, b| b.cmp(a));
        match counts[..2] {
            [5, _] => FiveKind,
            [4, _] => FourKind,
            [3, 2] => FullHouse,
            [3, _] => ThreeKind,
            [2, 2] => TwoPair,
            [2, _] => Pair,
            _ => HighCard,
        }
    }
}

fn calculate_winnings<const PART: u8>(input: &str) -> Result<i64> {
    let mut hands = parse_hands::<PART>(input)?;
    hands.sort_by(|a, b| a.rank(&b));
    Ok(hands
        .into_iter()
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| rank * bid)
        .sum::<i64>())
}

fn pt1(input: &str) -> Result<i64> {
    calculate_winnings::<1>(input)
}

fn pt2(input: &str) -> Result<i64> {
    calculate_winnings::<2>(input)
}

#[cfg(test)]
mod test {
    const INPUT: &str = "32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483";

    const INPUT2: &str = "2345A 1
    Q2KJJ 13
    Q2Q2Q 19
    T3T3J 17
    T3Q33 11
    2345J 3
    J345A 2
    32T3K 5
    T55J5 29
    KK677 7
    KTJJT 34
    QQQJA 31
    JJJJJ 37
    JAAAA 43
    AAAAJ 59
    AAAAA 61
    2AAAA 23
    2JJJJ 53
    JJJJ2 41"; // Pt1: 6592, Pt2: 6839

    #[test]
    fn handtype_pt1_ranking() {
        use super::{Card::*, Hand};
        let mut hands = super::parse_hands::<1>(INPUT).unwrap();
        hands.sort_unstable_by(|a, b| a.rank(&b));
        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: [Three, Two, Ten, Three, King],
                    bid: 765
                },
                Hand {
                    cards: [King, Ten, Jack, Jack, Ten],
                    bid: 220
                },
                Hand {
                    cards: [King, King, Six, Seven, Seven],
                    bid: 28
                },
                Hand {
                    cards: [Ten, Five, Five, Jack, Five],
                    bid: 684
                },
                Hand {
                    cards: [Queen, Queen, Queen, Jack, Ace],
                    bid: 483
                },
            ]
        );
    }

    #[test]
    fn handtype_pt2_ranking() {
        use super::{Card::*, Hand};
        let mut hands = super::parse_hands::<2>(INPUT).unwrap();
        hands.sort_unstable_by(|a, b| a.rank(&b));
        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: [Three, Two, Ten, Three, King],
                    bid: 765
                }, // Two of a kind
                Hand {
                    cards: [King, King, Six, Seven, Seven],
                    bid: 28
                }, // Two pair
                Hand {
                    cards: [Ten, Five, Five, Joker, Five],
                    bid: 684
                }, // Four of a kind
                Hand {
                    cards: [Queen, Queen, Queen, Joker, Ace],
                    bid: 483
                }, // Four of a kind
                Hand {
                    cards: [King, Ten, Joker, Joker, Ten],
                    bid: 220
                }, // Four of a kind
            ]
        );
    }

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 6440);
        assert_eq!(super::pt1(INPUT2).unwrap(), 6592);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 5905);
        assert_eq!(super::pt2(INPUT2).unwrap(), 6839);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 255048101);
    }
}
