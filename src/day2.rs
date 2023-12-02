use anyhow::{anyhow, Context, Result};

pub const INPUT: &str = include_str!("input/day2.txt");

solution!(INPUT, pt1, pt2);

#[derive(Clone, Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<(u32, u32, u32)>,
}

fn parse_game(input: &str) -> Result<Game> {
    let input = input.trim();
    let input = input
        .strip_prefix("Game ")
        .with_context(|| format!("unable to remove game prefix from '{input}'"))?;
    let (id, input) = input
        .split_once(": ")
        .with_context(|| format!("unable to split id from colours for '{input}'"))?;
    let id = id.parse::<u32>()?;
    let sets: Vec<(u32, u32, u32)> = input
        .split("; ")
        .map(parse_set)
        .collect::<Result<_>>()
        .with_context(|| format!("invalid set in {input}"))?;
    Ok(Game { id, sets })
}

fn parse_set(input: &str) -> Result<(u32, u32, u32)> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let cubes = input.split(", ");
    for cube in cubes {
        let (count, colour) = cube
            .split_once(' ')
            .with_context(|| format!("unable to split count from colour for cube '{cube}'"))?;
        let count = count.parse::<u32>()?;
        match colour {
            "red" => r += count,
            "green" => g += count,
            "blue" => b += count,
            _ => return Err(anyhow!("invalid colour '{colour}'")),
        }
    }
    Ok((r, g, b))
}

fn pt1(input: &str) -> Result<u32> {
    const R_MAX: u32 = 12;
    const G_MAX: u32 = 13;
    const B_MAX: u32 = 14;
    let games: Vec<Game> = input.lines().map(parse_game).collect::<Result<_>>()?;
    Ok(games
        .into_iter()
        .filter(|game| {
            game.sets
                .iter()
                .all(|(r, g, b)| *r <= R_MAX && *g <= G_MAX && *b <= B_MAX)
        })
        .map(|game| game.id)
        .sum())
}

fn pt2(input: &str) -> Result<u32> {
    let games: Vec<Game> = input.lines().map(parse_game).collect::<Result<_>>()?;
    games
        .iter()
        .map(|game| {
            let min = game
                .sets
                .iter()
                .copied()
                .reduce(|(max_r, max_g, max_b), (r, g, b)| {
                    (r.max(max_r), g.max(max_g), b.max(max_b))
                })
                .with_context(|| format!("no sets found for '{game:?}'"));
            min
        })
        .try_fold(0, |sum, set| set.map(|(r, g, b)| sum + (r * g * b)))
}

#[cfg(test)]
mod test {
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse() {
        let set = super::parse_set("1 blue, 2 green, 4 red").unwrap();
        assert_eq!(set, (4, 2, 1));
        let game = super::parse_game(INPUT.lines().next().unwrap()).unwrap();
        assert_eq!(
            game,
            super::Game {
                id: 1,
                sets: vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]
            }
        );
    }

    #[test]
    fn pt1() {
        let result = super::pt1(INPUT);
        assert_eq!(result.unwrap(), 8);
    }

    #[test]
    fn pt2() {
        let result = super::pt2(INPUT);
        assert_eq!(result.unwrap(), 2286);
    }
}
