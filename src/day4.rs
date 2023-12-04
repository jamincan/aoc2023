use anyhow::{Context, Result};

pub const INPUT: &str = include_str!("input/day4.txt");

solution!(INPUT, pt1, pt2);

struct Card {
    winners: Vec<u8>,
    chosen: Vec<u8>,
}

fn parse_card(input: &str) -> Result<Card> {
    // ID
    let rest = input
        .strip_prefix("Card")
        .with_context(|| format!("missing prefix for card `{input}`"))?;
    let (_id, rest) = rest.split_once(':').with_context(|| {
        format!("missing : separator between id and contents for card '{input}'")
    })?;

    // Winning numbers
    let (winners, chosen) = rest.split_once('|').with_context(|| {
        format!("missing | separator between winning numbers and chosen numbers for card '{input}'")
    })?;
    let winners: Vec<_> = winners
        .split_whitespace()
        .map(|num| {
            num.parse::<u8>()
                .with_context(|| format!("invalid winning number {num} for card '{input}'"))
        })
        .collect::<Result<_>>()?;

    // Chosen numbers
    let chosen: Vec<_> = chosen
        .split_whitespace()
        .map(|num| {
            num.parse::<u8>()
                .with_context(|| format!("invalid chosen number {num} for card '{input}'"))
        })
        .collect::<Result<_>>()?;

    Ok(Card { winners, chosen })
}

fn winning_numbers_count(card: &Card) -> u32 {
    let Card {
        winners, chosen, ..
    } = card;
    chosen.iter().filter(|num| winners.contains(*num)).count() as u32
}

fn pt1(input: &str) -> Result<u32> {
    let cards: Vec<_> = input.lines().map(parse_card).collect::<Result<_>>()?;
    let winning_cards = cards
        .iter()
        .map(winning_numbers_count)
        .filter(|count| *count > 0);
    Ok(winning_cards.map(|count| 2u32.pow(count - 1)).sum::<u32>())
}

fn pt2(input: &str) -> Result<u32> {
    let initial_cards: Vec<_> = input.lines().map(parse_card).collect::<Result<_>>()?;
    let mut card_counts = vec![1u32; initial_cards.len()];

    for (index, card) in initial_cards.into_iter().enumerate() {
        let card_count = card_counts[index];
        let winning_count = winning_numbers_count(&card);
        let next_card_indices = index + 1..=index + winning_count as usize;
        for next_card_index in next_card_indices {
            card_counts[next_card_index] += card_count;
        }
    }

    Ok(card_counts.iter().sum::<u32>())
}

#[cfg(test)]
mod test {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT).unwrap(), 13);
        assert_eq!(super::pt1(super::INPUT).unwrap(), 18519);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT).unwrap(), 30);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 11787590);
    }
}
