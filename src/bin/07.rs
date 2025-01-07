advent_of_code::solution!(7);

fn add(a: usize, b: usize) -> usize {
    a + b
}

fn mul(a: usize, b: usize) -> usize {
    a * b
}
fn conc(a: usize, b: usize) -> usize {
    if b == 0 {
        return a * 10;
    }
    let digits = b.ilog10() + 1;
    a * (10usize.pow(digits)) + b
}

fn consider_eval_r(
    operators: &[fn(usize, usize) -> usize],
    goal: usize,
    current: usize,
    nums: &[usize],
) -> bool {
    if nums.len() == 0 {
        return current == goal;
    }
    if current > goal {
        return false;
    }
    for op in operators {
        let folded = op(current, nums[0]);
        if consider_eval_r(operators, goal, folded, &nums[1..]) {
            // eprint!("{} ", &"+*"[i..i + 1]);
            return true;
        }
    }
    false
}

fn consider_eval(operators: &[fn(usize, usize) -> usize], goal: usize, nums: &[usize]) -> bool {
    let r = consider_eval_r(operators, goal, nums[0], &nums[1..]);
    r
}

fn parse_line(input: &str) -> (usize, Vec<usize>) {
    let mut i = input.split(": ");
    let goal_s = i.next().expect("Expected goal; LHS of ':'");
    let nums_s = i.next().expect("Expected numbers, RHS of ':'");
    let goal: usize = goal_s
        .trim()
        .parse()
        .expect("Expected goal string to be numerical");
    let nums: Vec<usize> = nums_s
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<usize>()
                .expect("Expected all RHS nums to be parsable")
        })
        .collect::<_>();
    (goal, nums)
}

fn analyse(input: &str, operators: &[fn(usize, usize) -> usize]) -> usize {
    input
        .trim()
        .split('\n')
        .map(parse_line)
        .filter(|(goal, nums)| consider_eval(operators, *goal, nums))
        .map(|(goal, _nums)| goal)
        .sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(analyse(input, &[add, mul][..]))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(analyse(input, &[add, mul, conc][..]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluator() {
        let shim = |(goal, nums): (usize, Vec<usize>)| consider_eval(&[add, mul][..], goal, &nums);
        let r = |l| shim(parse_line(l));
        assert!(r("190: 10 19"));
        println!();
        assert!(r("3267: 81 40 27"));
        println!();
        assert!(r("13757372640: 4 21 8 6 854 4 8 9 2 66 5"));
        assert!(r("873: 8 838 9 9 1 9"));
        assert!(!r("21037: 9 7 18 13"));
        assert!(!r("80: 6 7 3 5 2"));
    }

    #[test]
    fn test_concat() {
        assert_eq!(conc(12, 16), 1216);
        assert_eq!(conc(12, 6), 126);
        assert_eq!(conc(12, 0), 120);
        assert_eq!(conc(12, 10), 1210);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
