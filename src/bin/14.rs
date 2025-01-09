use std::io;

use aoc_utils::{grid::Grid, parse};
use lina::{point2, vec2, Point2, Vec2};

advent_of_code::solution!(14);

type I = i32;

#[derive(Debug)]
struct Robot {
    pub p: Point2<I>,
    pub v: Vec2<I>,
}

impl Robot {
    fn new(px: I, py: I, vx: I, vy: I) -> Self {
        Self {
            p: point2(px, py),
            v: vec2(vx, vy),
        }
    }

    pub fn from_line(line: &str) -> Option<Self> {
        let mut nums = parse::nums_signed(line).into_iter();
        Some(Robot::new(
            nums.next()?,
            nums.next()?,
            nums.next()?,
            nums.next()?,
        ))
    }

    pub fn step(&self, dim: Vec2<I>, steps: I) -> Self {
        let unbounded = self.p + self.v * steps;
        let bounded = unbounded.zip_map(dim.to_point(), |a, b| a.rem_euclid(b));
        Robot {
            p: bounded,
            v: self.v,
        }
    }

    pub fn quadrant(&self, dim: Vec2<I>) -> Option<usize> {
        // 0 1
        // 2 3
        let x = if self.p.x < dim.x / 2 {
            0
        } else if self.p.x > dim.x / 2 {
            1
        } else {
            return None;
        };
        let y = if self.p.y < dim.y / 2 {
            0
        } else if self.p.y > dim.y / 2 {
            1
        } else {
            return None;
        };
        Some(x + 2 * y)
    }
}

fn parse(input: &str) -> (Vec<Robot>, Vec2<I>) {
    let robots = input
        .trim()
        .split("\n")
        .map(Robot::from_line)
        .collect::<Option<Vec<_>>>()
        .expect("Expected all robots to parse");

    let dim = vec2(
        robots.iter().map(|r| r.p.x).max().unwrap() + 1,
        robots.iter().map(|r| r.p.y).max().unwrap() + 1,
    );

    (robots, dim)
}

const STEPS: I = 100;

pub fn part_one(input: &str) -> Option<u64> {
    let (r, dim) = parse(input);
    let after = r.iter().map(|r| r.step(dim, STEPS)).collect::<Vec<_>>();
    let quads = after.iter().filter_map(|r| r.quadrant(dim));

    let mut counts: [I; 4] = [0; 4];
    for q in quads {
        counts[q] += 1;
    }

    let mut view = Grid::new_with_dimensions_uniform(dim, 0);
    for r in after {
        view[r.p] += 1;
    }
    println!(
        "{}",
        view.map(|x| char::from_digit(*x as u32, 16).unwrap() as u8)
            .display()
    );
    Some(counts.into_iter().product::<I>() as u64)
}

fn wait() -> bool {
    let mut b = String::new();
    match io::stdin().read_line(&mut b) {
        Ok(n) => n != 0,
        Err(error) => {
            println!("error: {error}");
            false
        }
    }
}

fn filter_possible_tree(robots: &Vec<Robot>) -> bool {
    let mut points: Vec<_> = robots.iter().map(|x| x.p).collect();

    points.sort_by_key(|p| (p.y, p.x));

    points
        .chunk_by(|a, b| a.y == b.y)
        // .inspect(|x| {
        //     dbg!(x);
        // })
        .any(|row| row.chunk_by(|a, b| a.x + 1 == b.x).any(|x| x.len() >= 8))
}

fn step(mut robots: Vec<Robot>, dim: Vec2<I>) -> Vec<Robot> {
    for robot in robots.iter_mut() {
        *robot = robot.step(dim, 1);
    }
    robots
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut r, dim) = parse(input);

    let mut view = Grid::new_with_dimensions_uniform(dim, 0);
    for seconds in 1.. {
        println!("{seconds}");
        r = step(r, dim);
        if filter_possible_tree(&r) {
            print_grid(&r, &mut view);
            if !wait() {
                return Some(seconds);
            }
        }
    }

    println!(
        "{}",
        view.map(|x| char::from_digit(*x as u32, 16).unwrap() as u8)
            .display()
    );
    None
}

fn print_grid(r: &Vec<Robot>, view: &mut Grid<i32>) {
    view.0
        .iter_mut()
        .for_each(|x| x.iter_mut().for_each(|y| *y = 0));
    for robot in r.iter() {
        view[robot.p] = 1;
    }
    println!("{}", view.map(|&x| if x > 0 { '#' } else { ' ' }).display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
