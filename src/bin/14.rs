use aoc_utils::parse;
use lina::{point2, vec2, Point2, Vec2};

advent_of_code::solution!(14);

type I = i64;

struct Robot {
    p: Point2<I>,
    v: Vec2<I>,
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
}

fn parse(input: &str) -> Vec<Robot> {
    let robots: Option<Vec<Robot>> = input.split("\n").map(Robot::from_line).collect();

    robots.expect("Expected all robots to parse")
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
