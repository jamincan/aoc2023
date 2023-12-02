pub const INPUT: &str = include_str!("input/day2.txt");

solution!(INPUT, pt1, pt2);

#[derive(Clone, Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<(u32, u32, u32)>,
}

fn parse_game(input: &str) -> Option<Game> {
    let input = input.trim();
    let input = if input.starts_with("Game ") {
        &input[5..]
    } else {
        return None;
    };
    let (id, input) = input.split_once(": ")?;
    let id = id.parse::<u32>().ok()?;
    let sets: Vec<(u32, u32, u32)> = input.split("; ").map(parse_set).collect::<Option<_>>()?;
    Some(Game { id, sets })
}

fn parse_set(input: &str) -> Option<(u32, u32, u32)> {
    let (mut r, mut g, mut b) = (0, 0, 0);
    let cubes = input.split(", ");
    for cube in cubes {
        let (count, colour) = cube.split_once(' ')?;
        let count = count.parse::<u32>().ok()?;
        match colour {
            "red" => r += count,
            "green" => g += count,
            "blue" => b += count,
            _ => return None,
        }
    }

    Some((r, g, b))
}

fn pt1(input: &str) -> Option<u32> {
    const R_MAX: u32 = 12;
    const G_MAX: u32 = 13;
    const B_MAX: u32 = 14;
    let games: Vec<Game> = input.lines().map(parse_game).collect::<Option<_>>()?;
    Some(
        games
            .into_iter()
            .filter(|Game { sets, .. }| {
                sets.iter()
                    .all(|(r, g, b)| *r <= R_MAX && *g <= G_MAX && *b <= B_MAX)
            })
            .map(|Game { id, .. }| id)
            .sum(),
    )
}

fn pt2(input: &str) -> Option<u32> {
    let games: Vec<Game> = input.lines().map(parse_game).collect::<Option<_>>()?;
    games
        .iter()
        .map(|Game { sets, .. }| {
            let min = sets
                .iter()
                .copied()
                .reduce(|(max_r, max_g, max_b), (r, g, b)| {
                    (r.max(max_r), g.max(max_g), b.max(max_b))
                });
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn pt2() {
        let powers: Vec<_> = INPUT.lines().map(super::pt2).collect();
        assert_eq!(
            powers,
            vec![Some(48), Some(12), Some(1560), Some(630), Some(36)]
        );
        let result = super::pt2(INPUT);
        assert_eq!(result, Some(2286))
    }
}
