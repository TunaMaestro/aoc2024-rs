use aoc_utils::parse;
use lina::{point2, vec2, Point2, Vec2};

advent_of_code::solution!(13);

type I = i128;

const PART_2_OFFSET: I = 10000000000000;

#[derive(Debug)]
struct Machine {
    a: Vec2<I>,
    b: Vec2<I>,
    prize: Point2<I>,
}

impl Machine {
    pub fn parse(input: &str) -> Option<Self> {
        let mut nums = parse::nums_positive::<I>(input).into_iter();
        Some(Machine {
            a: vec2(nums.next()?, nums.next()?),
            b: vec2(nums.next()?, nums.next()?),
            prize: point2(nums.next()?, nums.next()?),
        })
    }
    fn maximise(&self) -> Option<I> {
        let mut best_score: Option<I> = None;
        for b_count in (0 as I).. {
            if b_count * self.b.x > self.prize.x || b_count * self.b.y > self.prize.y {
                break;
            }
            let req = self.prize - (b_count * self.b);
            // dbg!(b_count, req, req.x % self.a.x == 0, req.y % self.a.y == 0);
            if req.x % self.a.x == 0
                && req.y % self.a.y == 0
                && req.x / self.a.x == req.y / self.a.y
            {
                let a_count = req.x / self.a.x;
                let cost = A_COST * a_count + B_COST * b_count;
                assert_eq!(a_count * self.a + b_count * self.b, self.prize.to_vec());
                if best_score.map(|bs| cost < bs).unwrap_or(true) {
                    assert!(
                        best_score.is_none(),
                        "There should only ever be one way of producing a vector from the basis"
                    );
                    best_score = Some(cost);
                }
            }
        }
        best_score
    }

    #[allow(non_snake_case)]
    fn maximise_fast(&self) -> Option<I> {
        /*
         A(x1, y1) + B(x2, y2) = (X, Y)
         A.x1 + B.y1 = X (1)
         A.x2 + B.y2 = Y (2)

         x1.(2) - y1.(1):
         A.x1.y1 + B.x1.y2 - A.x1.y1 - B.x2.y1 = Y.x1 - X.y1

         B (x1.y2 - x2.y1) = Y.x1 - X.y1

         Also shows us that there are one or zero solutions. Which shold have been obvious because *A* and *B* form a basis.
        */

        let x1 = self.a.x;
        let y1 = self.a.y;
        let x2 = self.b.x;
        let y2 = self.b.y;

        let X = self.prize.x;
        let Y = self.prize.y;

        assert!(self.a != self.b);
        let rhs = Y * x1 - X * y1;
        let denom = y2 * x1 - x2 * y1;

        let b = rhs / denom;
        // dbg!(rhs as f64 / denom as f64, rhs, denom, &self);
        if rhs % denom != 0 {
            return None;
        }
        let a = (X - b * x2) / x1;
        if (X - b * x2) % x1 != 0 {
            return None;
        }

        // dbg!(&self);
        assert_eq!(a * self.a + b * self.b, self.prize.to_vec(),);
        Some(a * A_COST + b * B_COST)
    }

    fn part2(&self) -> Self {
        Machine {
            a: self.a,
            b: self.b,
            prize: self.prize + vec2(1, 1) * PART_2_OFFSET,
        }
    }
}

const A_COST: I = 3;
const B_COST: I = 1;

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, |x: &Machine| x.maximise_fast().unwrap_or(0) as u64)
}

fn solve(input: &str, f: impl Fn(&Machine) -> u64) -> Option<u64> {
    let Some(machines) = input
        .split("\n\n")
        .map(Machine::parse)
        .collect::<Option<Vec<_>>>()
    else {
        panic!("All machines could not be parsed.");
    };
    let score = machines.iter().map(f).sum();
    Some(score)
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(input, |x| x.part2().maximise_fast().unwrap_or(0) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine() {
        let m = Machine::parse(
            "
                    Button A: X+94, Y+34
                    Button B: X+22, Y+67
                    Prize: X=8400, Y=5400
            ",
        )
        .unwrap();

        assert_eq!(m.maximise(), Some(280));
        assert_eq!(m.maximise_fast(), Some(280));
        let m = Machine::parse(
            "
                Button A: X+56, Y+20
                Button B: X+24, Y+48
                Prize: X=1264, Y=11536
            ",
        )
        .unwrap();

        assert_eq!(m.maximise_fast(), None);
        assert_eq!(m.maximise(), None);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
