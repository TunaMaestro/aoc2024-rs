use core::panic;

use aoc_utils::grid::{Grid, Point, UP_RIGHT_DOWN_LEFT};

advent_of_code::solution!(6);

fn step_r(
    grid: &Grid<u8>,
    depth: usize,
    (player, direction): (Point, usize),
) -> Option<(Point, usize)> {
    if depth > 4 {
        eprintln!("{}", grid.display());
        dbg!(player, direction);
        panic!("Stepped too deep should not have made 4 steps in one")
    }
    let delta = UP_RIGHT_DOWN_LEFT[direction];
    let next_pos = player + delta;
    if !grid.contains(next_pos) {
        None
    } else {
        if grid[next_pos] == b'#' {
            return step_r(grid, depth + 1, (player, (direction + 1) % 4));
        }
        Some((next_pos, direction))
    }
}

fn step(grid: &Grid<u8>, (player, direction): (Point, usize)) -> Option<(Point, usize)> {
    step_r(grid, 0, (player, direction))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = Grid::read(input, |x| x as u8);
    let player = grid
        .position(|&x| x == b'^')
        .expect("expected player '^' char in grid");

    grid[player] = b'.';

    let mut seen = Grid::new_with_dimensions_uniform(grid.dimension(), false);

    let mut state = Some((player, 0));

    let mut hit = 0;

    while let Some(s) = state {
        hit += (seen[s.0] == false) as usize;
        seen[s.0] = true;
        state = step(&grid, s);
    }

    // let _view = Grid::new_with_dimensions(seen.dimension(), |x| x).map(|&x| {
    //     if grid[x] == b'#' {
    //         b'#'
    //     } else if seen[x] {
    //         b'X'
    //     } else {
    //         b'.'
    //     }
    // });

    // eprintln!("{}", view.display());

    Some(hit)
}

fn find_cycle(grid: &Grid<u8>, state: (Point, usize)) -> Option<(Point, usize)> {
    let mut fast = state;
    let mut slow = state;
    let once = |s| step(grid, s);
    slow = once(slow)?;
    fast = once(once(fast)?)?;
    while slow != fast {
        slow = once(slow)?;
        fast = once(once(fast)?)?;
    }
    return Some(slow);
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = Grid::read(input, |x| x as u8);
    let player = grid
        .position(|&x| x == b'^')
        .expect("expected player '^' char in grid");

    grid[player] = b'.';

    let init_state = (player, 0);

    let mut state = Some(init_state);
    let mut prev: Option<(Point, usize)> = None;

    let mut cycles = 0;

    // dbg
    let mut seen = Grid::new_with_dimensions_uniform(grid.dimension(), 0u8);
    // dbg

    while let Some(s) = state {
        if seen[s.0] & 1 == 0 {
            if let Some(p) = prev {
                grid[s.0] = b'#';
                let c = find_cycle(&grid, p).is_some();
                // eprintln!("{:?} {}", s.0, c);
                // cycles += (c as usize);
                cycles += (c && !seen[s.0] & 0b10 > 0) as usize;
                seen[s.0] |= (c as u8) << 1;
                grid[s.0] = b'.';
            }
        }
        seen[s.0] |= 1;
        // next step
        prev = state;
        state = step(&grid, s);
    }

    // let view = Grid::new_with_dimensions(grid.dimension(), |x| x).map(|&x| {
    //     if grid[x] == b'#' {
    //         b'#'
    //     } else if seen[x] & 0b10 > 0 {
    //         b'O'
    //     } else if seen[x] & 0b1 > 0 {
    //         b'x'
    //     } else {
    //         b'.'
    //     }
    // });
    // eprintln!("{}", view.display());

    Some(cycles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_two_parts() {
        {
            let result = part_two(&advent_of_code::template::read_file_part(
                "examples", DAY, 1,
            ));
            assert_eq!(result, Some(0));
        }
        {
            let result = part_two(&advent_of_code::template::read_file_part(
                "examples", DAY, 2,
            ));
            assert_eq!(result, Some(5));
        }
    }
}
