use std::{io::Empty, mem::swap};

use aoc_utils::grid::{Grid, Point, UP_RIGHT_DOWN_LEFT};
use lina::Vec2;

advent_of_code::solution!(15);

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Box,
    Empty,
}

#[derive(Clone, Copy)]
enum Tile2 {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
}

type Robot = Point;

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

impl Tile2 {
    pub fn parse(c: u8) -> Option<[Self; 2]> {
        use Tile2::*;
        match c {
            b'#' => Some([Wall, Wall]),
            b'.' | b'@' => Some([Empty, Empty]),
            b'O' => Some([BoxLeft, BoxRight]),
            _ => None,
        }
    }
}

impl std::fmt::Display for Tile2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile2::Wall => "#",
            Tile2::BoxLeft => "[",
            Tile2::BoxRight => "]",
            Tile2::Empty => ".",
        };
        write!(f, "{s}")
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    N,
    E,
    S,
    W,
}

impl Instruction {
    fn index(&self) -> usize {
        match self {
            Instruction::N => 0,
            Instruction::E => 1,
            Instruction::S => 2,
            Instruction::W => 3,
        }
    }

    fn v(&self) -> Vec2<i32> {
        UP_RIGHT_DOWN_LEFT[self.index()]
    }
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

fn parse(input: &str) -> (Grid<Tile>, Robot, Vec<Instruction>) {
    let mut input = input.split("\n\n");
    let grid = Grid::read(input.next().expect("Expected a grid"), |x| x as u8);
    let robot = grid
        .position(|x| *x == b'@')
        .expect("Expected a @ for the robot");
    let grid = grid.map(|&x| Tile::try_from(x).unwrap());
    let instructions = parse_instructions(input);

    (grid, robot, instructions)
}

fn parse2(input: &str) -> (Grid<Tile2>, Robot, Vec<Instruction>) {
    let mut input = input.split("\n\n");
    let grid = Grid::read(input.next().expect("Expected a grid"), |x| x as u8);
    let mut robot = grid
        .position(|x| *x == b'@')
        .expect("Expected a @ for the robot");
    robot.x *= 2;

    let grid = Grid::new(
        grid.0
            .iter()
            .map(|l| {
                l.iter()
                    .map(|&c| Tile2::parse(c).unwrap())
                    .flatten()
                    .collect()
            })
            .collect(),
    );
    let instructions = parse_instructions(input);

    (grid, robot, instructions)
}

fn parse_instructions(mut input: std::str::Split<'_, &str>) -> Vec<Instruction> {
    let instructions = input
        .next()
        .expect("Expected instructions")
        .as_bytes()
        .iter()
        .filter_map(|&x| Instruction::try_from(x).ok())
        .collect();
    instructions
}

fn print_state(grid: &Grid<Tile>, robot: Robot) {
    let mut char_grid = grid.char();
    char_grid[robot] = '@';
    char_grid.print();
}

fn print_state2(grid: &Grid<Tile2>, robot: Robot) {
    let mut char_grid = grid.map(|x| format!("{x}"));
    char_grid[robot] = "@".to_owned();
    char_grid.print();
}

fn execute(
    mut grid: Grid<Tile>,
    mut robot: Robot,
    instruction: Instruction,
) -> (Grid<Tile>, Robot) {
    let d = instruction.v();
    let dest = robot + d;
    let mut end = dest;

    while grid.contains(end) {
        let t = grid[end];
        match t {
            Tile::Wall => break,
            Tile::Box => end += d,
            Tile::Empty => {
                (grid[end], grid[dest]) = (grid[dest], grid[end]);
                robot = dest;
                break;
            }
        }
    }
    (grid, robot)
}

fn nudge(mut grid: Grid<Tile2>, point: Point, dir: Instruction) -> Grid<Tile2> {
    // println!(
    //     "({0}, {1}) {1}:{0} {2}",
    //     point.x + 1,
    //     point.y + 1,
    //     grid[point]
    // );
    let mut grid = match grid[point] {
        Tile2::Wall => {
            panic!("Wall should never have been nudged while writing nudges")
        }
        Tile2::BoxLeft => nudge_box(grid, point, dir, Instruction::E),
        Tile2::BoxRight => nudge_box(grid, point, dir, Instruction::W),
        Tile2::Empty => grid,
    };
    // println!("{} <- {}", grid[point], grid[point - dir.v()]);
    (grid[point], grid[point - dir.v()]) = (grid[point - dir.v()], grid[point]);
    grid
}

fn nudge_box(
    grid: Grid<Tile2>,
    point: Point,
    dir: Instruction,
    other_dir: Instruction,
) -> Grid<Tile2> {
    let grid = nudge(grid, point + dir.v(), dir);
    let grid = if let (Instruction::N | Instruction::S) = dir {
        nudge(grid, point + dir.v() + other_dir.v(), dir)
    } else {
        grid
    };
    grid
}

fn check_nudge(grid: &Grid<Tile2>, point: Point, dir: Instruction) -> bool {
    match grid[point] {
        Tile2::Wall => false,
        Tile2::BoxLeft => check_nudge_box(grid, point, dir, Instruction::E),
        Tile2::BoxRight => check_nudge_box(grid, point, dir, Instruction::W),
        Tile2::Empty => true,
    }
}

fn check_nudge_box(
    grid: &Grid<Tile2>,
    point: Point,
    dir: Instruction,
    other_dir: Instruction,
) -> bool {
    check_nudge(&grid, point + dir.v(), dir)
        && if let (Instruction::N | Instruction::S) = dir {
            check_nudge(&grid, point + dir.v() + other_dir.v(), dir)
        } else {
            true
        }
}

fn execute2(grid: Grid<Tile2>, mut robot: Robot, instruction: Instruction) -> (Grid<Tile2>, Robot) {
    if check_nudge(&grid, robot + instruction.v(), instruction) {
        let grid = nudge(grid, robot + instruction.v(), instruction);
        (grid, robot + instruction.v())
    } else {
        (grid, robot)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (mut grid, mut robot, instructions) = parse(input);
    for i in instructions {
        (grid, robot) = execute(grid, robot, i);
        // print_state(&grid, robot);
    }
    let scores = Grid::new_with_dimensions(grid.dimension(), |p| {
        if let Tile::Box = grid[p] {
            (p.x + p.y * 100) as u64
        } else {
            0
        }
    });
    Some(scores.0.into_iter().flatten().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut grid, mut robot, instructions) = parse2(input);
    // print_state2(&grid, robot);
    for i in instructions {
        // println!("{:?}", i);
        (grid, robot) = execute2(grid, robot, i);
        // print_state2(&grid, robot);
    }
    let scores = Grid::new_with_dimensions(grid.dimension(), |p| {
        if let Tile2::BoxLeft = grid[p] {
            (p.x + p.y * 100) as u64
        } else {
            0
        }
    });

    Some(scores.0.into_iter().flatten().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
