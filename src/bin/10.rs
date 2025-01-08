use std::collections::HashSet;

use aoc_utils::grid::{Grid, Point, UP_RIGHT_DOWN_LEFT};
use lina::point2;

advent_of_code::solution!(10);

struct Cell {
    height: u8,
    score: usize,
}

fn parse(input: &str) -> Grid<Cell> {
    Grid::read(input, |c| Cell {
        height: c.to_digit(10).expect("expected only '0'~'9'") as u8,
        score: 0,
    })
}

fn dfs(grid: &mut Grid<Cell>, v: Point, visited: &mut HashSet<Point>) {
    visited.insert(v);
    // if grid[v].height == 9 {
    //     grid[v].score = Some(1);
    //     return 1;
    // }
    // if let Some(score) = grid[v].score {
    //     return score;
    // }
    // let mut score = 0;
    for dir in UP_RIGHT_DOWN_LEFT {
        let next = v + dir;
        if !grid.contains(next) {
            continue;
        }
        if grid[next].height + 1 != grid[v].height {
            continue;
        }
        if visited.contains(&next) {
            continue;
        }
        dfs(grid, next, visited);
        // score += dfs(grid, next, visited);
    }
    grid[v].score += 1;
    // score
}

fn dfs_2(grid: &mut Grid<Cell>, v: Point) {
    for dir in UP_RIGHT_DOWN_LEFT {
        let next = v + dir;
        if !grid.contains(next) {
            continue;
        }
        if grid[next].height + 1 != grid[v].height {
            continue;
        }
        dfs_2(grid, next);
    }
    grid[v].score += 1;
}

fn search(grid: &mut Grid<Cell>, v: Point) {
    let mut visited = HashSet::new();
    dfs(grid, v, &mut visited)
}

fn p(grid: &Grid<Cell>) {
    println!(
        "{}\n",
        grid.map(|x| {
            // if let Some(s) = x.score {
            //     if s > 0 {
            //         char::from_digit(s as u32, 16).unwrap() as u8
            //     } else {
            //         b'-'
            //     }
            // } else {
            //     b'.'
            // }
            if x.score > 0 {
                char::from_digit(x.score as u32, 16).unwrap() as u8
            } else {
                b'.'
            }
        })
        .display()
    );
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let dim = grid.dimension();
    (0..dim.y)
        .map(|y| (0..dim.x).map(move |x| point2(x as i32, y as i32)))
        .flatten()
        .for_each(|point| {
            if grid[point].height == 9 {
                // p(&grid);
                search(&mut grid, point)
            }
        });

    let res = score(&grid);

    // dbg!(grid.map(|x| x.score).0);
    // p(&grid);
    // println!();

    Some(res)
}

fn score(grid: &Grid<Cell>) -> usize {
    grid.0
        .iter()
        .map(|r| r.iter().map(|x| x.score * ((x.height == 0) as usize)))
        .flatten()
        .sum()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut grid = parse(input);
    let dim = grid.dimension();
    (0..dim.y)
        .map(|y| (0..dim.x).map(move |x| point2(x as i32, y as i32)))
        .flatten()
        .for_each(|point| {
            if grid[point].height == 9 {
                // p(&grid);
                dfs_2(&mut grid, point)
            }
        });

    let res = score(&grid);

    // dbg!(grid.map(|x| x.score).0);
    // p(&grid);
    // println!();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(1));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
