use anyhow::{bail, Context, Result};
use include_aoc::include_aoc;
use itertools::Itertools;

static INPUT: &str = include_aoc!(2023, 10);

solution!(INPUT, pt1, pt2);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Pipe {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
    Ground,
    Start,
}

impl std::convert::TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self> {
        Ok(match value {
            '|' => Pipe::NS,
            '-' => Pipe::EW,
            'L' => Pipe::NE,
            'J' => Pipe::NW,
            'F' => Pipe::SE,
            '7' => Pipe::SW,
            '.' => Pipe::Ground,
            'S' => Pipe::Start,
            _ => bail!("invalid pipe symbol '{value}'"),
        })
    }
}

/// Indicates whether a cell is on the loop, to the right of the path we are following along the loop, or not yet determined
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RelativePosition {
    Right,
    Left,
    Loop,
    Unknown
}

struct Map {
    pipes: Vec<(Pipe, RelativePosition)>,
    width: isize,
}

impl Map {
    /// Returns the index of the starting point, and the index of the next pipe in the loop
    fn start_paths(&self) -> Result<(isize, isize)> {
        // Get start index
        let start_idx = self
            .pipes
            .iter()
            .enumerate()
            .find(|(_, (pipe, _rel_pos))| *pipe == Pipe::Start)
            .map(|(idx, _)| idx)
            .context("no starting position found")? as isize;

        // North
        if let Some((Pipe::NS | Pipe::SE | Pipe::SW, _)) = self.pipes.get((start_idx - self.width) as usize) {
            return Ok((start_idx, start_idx - self.width));
        }
        // South
        if let Some((Pipe::NS | Pipe::NE | Pipe::NW, _)) = self.pipes.get((start_idx + self.width) as usize) {
            return Ok((start_idx, start_idx + self.width));
        }
        // East
        if start_idx + 1 % self.width != 0 && let Some((Pipe::EW | Pipe::NW | Pipe::SW, _)) = self.pipes.get(start_idx as usize + 1) {
            return Ok((start_idx, start_idx + 1));
        }
        // West
        if start_idx % self.width != 0 && let Some((Pipe::EW | Pipe::NE | Pipe::SE, _)) = self.pipes.get(start_idx as usize - 1) {
            return Ok((start_idx, start_idx - 1));
        }
        bail!("no paths found from starting position");
    }

    fn set_relative_position(&mut self, origin: isize, index: isize, relative_position: RelativePosition) {
        // Return early if east-west index shift moves it north or south
        let from_east = index - origin == -1;
        let from_west = index - origin == 1;
        if from_east && origin % self.width == 0 { return };
        if from_west && index % self.width == 0 { return };

        match self.pipes.get_mut(index as usize) {
            Some((_, RelativePosition::Loop)) => return, // If already marked as loop, don't overwrite
            Some((_, rel_pos)) => *rel_pos = relative_position,
            _ => return,
        }
    }

    // Follow the pipe and set the relative position of the pipe and any adjacent non-pipe cells
    fn follow_pipe(&mut self, from: isize, current: isize) -> Option<isize> {
        let (current_pipe, rel_pos) = self.pipes.get_mut(current as usize)?;
        let from_north = current - from == self.width;
        let from_south = current - from == -self.width;
        let from_east = current - from == -1;
        let from_west = current - from == 1;
        let next = match current_pipe {
            Pipe::NS if from_north => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - 1, RelativePosition::Right);
                self.set_relative_position(current, current + 1, RelativePosition::Left);
                current + self.width
            },
            Pipe::NS if from_south => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + 1, RelativePosition::Right);
                self.set_relative_position(current, current - 1, RelativePosition::Left);
                current - self.width
            },
            Pipe::EW if from_east => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - self.width, RelativePosition::Right);
                self.set_relative_position(current, current + self.width, RelativePosition::Left);
                current - 1
            },
            Pipe::EW if from_west => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + self.width, RelativePosition::Right);
                self.set_relative_position(current, current - self.width, RelativePosition::Left);
                current + 1
            },
            Pipe::NE if from_north => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - 1, RelativePosition::Right);
                self.set_relative_position(current, current + self.width, RelativePosition::Right);
                current + 1
            },
            Pipe::NE if from_east => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - 1, RelativePosition::Left);
                self.set_relative_position(current, current + self.width, RelativePosition::Left);
                current - self.width
            },
            Pipe::NW if from_north => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + 1, RelativePosition::Left);
                self.set_relative_position(current, current + self.width, RelativePosition::Left);
                current - 1
            },
            Pipe::NW if from_west => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + 1, RelativePosition::Right);
                self.set_relative_position(current, current + self.width, RelativePosition::Right);
                current - self.width
            },
            Pipe::SE if from_south => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - 1, RelativePosition::Left);
                self.set_relative_position(current, current - self.width, RelativePosition::Left);
                current + 1
            },
            Pipe::SE if from_east => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current - 1, RelativePosition::Right);
                self.set_relative_position(current, current - self.width, RelativePosition::Right);
                current + self.width
            },
            Pipe::SW if from_south => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + 1, RelativePosition::Right);
                self.set_relative_position(current, current - self.width, RelativePosition::Right);
                current - 1
            },
            Pipe::SW if from_west => {
                *rel_pos = RelativePosition::Loop;
                self.set_relative_position(current, current + 1, RelativePosition::Left);
                self.set_relative_position(current, current - self.width, RelativePosition::Left);
                current + self.width
            },
            _ => return None,
        };
        if next < 0 || next >= self.pipes.len() as isize { return None };
        Some(next)
    }
}

impl std::str::FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let first_line = input.lines().next().context("no input found")?;
        let width = first_line.len() as isize;
        let pipes: Vec<_> = input
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(Pipe::try_from)
            .map_ok(|pipe| (pipe, RelativePosition::Unknown))
            .collect::<Result<_>>()?;
        Ok(Map { pipes, width })
    }
}

fn pt1(input: &str) -> Result<i32> {
    let mut map: Map = input.parse()?;
    let (start, mut current) = map.start_paths()?;
    let mut prev = start;
    let mut count = 1;
    loop {
        count += 1;
        let Some(next) = map.follow_pipe(prev, current) else { bail!("invalid path taken")};
        if let (Pipe::Start, _) = map.pipes[next as usize] {
            break;
        } else {
            prev = current;
            current = next;
        }
    }
    Ok(count / 2)
}

fn pt2(input: &str) -> Result<i32> {
    let mut map: Map = input.parse()?;
    let (start, mut current) = map.start_paths()?;
    let mut prev = start;
    // Mark the loop
    map.pipes[start as usize].1 = RelativePosition::Loop;
    loop {
        let Some(next) = map.follow_pipe(prev, current) else { bail!("invalid path taken")};
        if let (Pipe::Start, _) = map.pipes[next as usize] {
            break;
        } else {
            prev = current;
            current = next;
        }
    };

    // Fill remaining unknowns with right/left - do a column starting from Start up and then down, and then do rows from top to bottom from column out
    let start_row = start / map.width;
    let start_col = start % map.width;
    let mut last = RelativePosition::Loop;
    let mut outer: RelativePosition = RelativePosition::Unknown;
    for col in (0..=start_col).rev().chain(start_col + 1..map.width) {
        use RelativePosition as Rel;
        'row: for row in (0..=start_row).rev().chain(start_row + 1..) {
            let idx = row * map.width + col;
            let mut last_row = false;
            match map.pipes.get_mut(idx as usize) {
                Some((_, rel_pos @ (Rel::Left | Rel::Right))) => last = *rel_pos,
                Some((_, rel_pos @ Rel::Unknown)) => *rel_pos = last,
                None => last_row = true,  // Tried to get position past the grid - done iteration over rows
                _ => (),
            };
            if row == 0 || last_row || col == 0 || col == map.width - 1 {
                if let (Rel::Unknown, Rel::Left | Rel::Right) = (outer, last) {
                    outer = last;
                }
            }
            if last_row { break 'row; }
        }
    }
    let inner = if outer == RelativePosition::Left { RelativePosition::Right } else { RelativePosition::Left };
    Ok(map.pipes.into_iter().filter(|(_, rel_pos)| *rel_pos == inner).count() as i32)
}

#[cfg(test)]
mod test {
    const INPUT1: &str = ".....
    .S-7.
    .|.|.
    .L-J.
    .....";

    const INPUT2: &str = "..F7.
    .FJ|.
    SJ.L7
    |F--J
    LJ...";

    const INPUT3: &str = "...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

    const INPUT4: &str = ".F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";

    const INPUT5: &str = "FF7FSF7F7F7F7F7F---7
    L|LJ||||||||||||F--J
    FL-7LJLJ||||||LJL-77
    F--JF--7||LJLJ7F7FJ-
    L---JF-JLJ.||-FJLJJ7
    |F|F-JF---7F7-L7L|7|
    |FFJF7L7F-JF7|JL---7
    7-L-JL7||F7|L7F-7F7|
    L.L7LFJ|||||FJL7||LJ
    L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn pt1() {
        assert_eq!(super::pt1(INPUT1).unwrap(), 4);
        assert_eq!(super::pt1(INPUT2).unwrap(), 8);
    }

    #[test]
    fn pt2() {
        assert_eq!(super::pt2(INPUT3).unwrap(), 4);
        assert_eq!(super::pt2(INPUT4).unwrap(), 8);
        assert_eq!(super::pt2(INPUT5).unwrap(), 10);
    }

    #[test]
    fn real_input() {
        assert_eq!(super::pt1(super::INPUT).unwrap(), 7005);
        assert_eq!(super::pt2(super::INPUT).unwrap(), 417);
    }
}
