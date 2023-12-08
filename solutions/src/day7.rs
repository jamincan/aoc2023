use anyhow::{bail, Context, Result};

use include_aoc::include_aoc;

static INPUT: &str = include_aoc!(2023, 7);

solution!(INPUT, pt1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum Card {
    Joker = 0,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

fn parse_card<const PART: u8>(input: char) -> Result<Card> {
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

        let self_type = HandType::from(self.cards);
        let other_type = HandType::from(other.cards);
        match self_type.cmp(&other_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            result => result,
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
    HighCard(Card),
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl From<[Card; 5]> for HandType {
    fn from(cards: [Card; 5]) -> Self {
        use HandType::*;

        // Get a count of each card in the hand, using an array as a simple map
        let mut counts = [0; 15];
        for card in cards {
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
            _ => HighCard(
                cards
                    .into_iter()
                    .max()
                    .expect("cards is not empty, so there is a highest one"),
            ),
        }
    }
}

fn pt1(input: &str) -> Result<i64> {
    let mut hands = parse_hands::<1>(input)?;
    hands.sort_unstable_by(|a, b| a.rank(&b));
    Ok(hands
        .into_iter()
        .zip(1..)
        .map(|(Hand { bid, .. }, rank)| rank * bid)
        .sum::<i64>())
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
    fn hands_pt1_parsing() {
        use super::{Card::*, Hand};
        let hands = super::parse_hands::<1>(INPUT).unwrap();
        assert_eq!(
            hands,
            vec![
                Hand {
                    cards: [Three, Two, Ten, Three, King],
                    bid: 765
                },
                Hand {
                    cards: [Ten, Five, Five, Jack, Five],
                    bid: 684
                },
                Hand {
                    cards: [King, King, Six, Seven, Seven],
                    bid: 28
                },
                Hand {
                    cards: [King, Ten, Jack, Jack, Ten],
                    bid: 220
                },
                Hand {
                    cards: [Queen, Queen, Queen, Jack, Ace],
                    bid: 483
                },
            ]
        );
    }

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
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 6440);
        assert_eq!(super::pt1(INPUT2).unwrap(), 6592);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 255048101);
    }
}
