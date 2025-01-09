use std::io::Empty;

use aoc_utils::grid::{Grid, Point};

advent_of_code::solution!(15);

enum Tile {
    Wall,
    Box,
    Empty,
}

impl TryFrom<u8> for Tile {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Tile::*;
        match value {
            b'#' => Ok(Wall),
            b'.' | b'@' => Ok(Empty),
            b'O' => Ok(Box),
            _ => Err(()),
        }
    }
}

enum Instruction {
    N,
    E,
    S,
    W,
}

impl TryFrom<u8> for Instruction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use Instruction::*;
        match value {
            b'^' => Ok(N),
            b'>' => Ok(E),
            b'v' => Ok(S),
            b'<' => Ok(W),
            _ => Err(()),
        }
    }
}

impl std::convert::Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Self::Wall => '#',
            Self::Box => 'O',
            Self::Empty => '.',
        }
    }
}

fn parse(input: &str) -> (Grid<Tile>, Point, Vec<Instruction>) {
    let mut input = input.split("\n\n");
    let grid = Grid::read(input.next().expect("Expected a grid"), |x| x as u8);
    let robot = grid
        .position(|x| *x == b'@')
        .expect("Expected a @ for the robot");
    let grid = grid.map(|&x| Tile::try_from(x).unwrap());
    let instructions = input
        .next()
        .expect("Expected instructions")
        .as_bytes()
        .iter()
        .filter_map(|&x| Instruction::try_from(x).ok())
        .collect();

    (grid, robot, instructions)
}

pub fn part_one(input: &str) -> Option<u64> {
    None
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
