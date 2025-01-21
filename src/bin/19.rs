use aoc_utils::debug::PrintBytes;
use std::{array, io::BufRead, ops::Index, rc::Rc, sync::Arc};

use itertools::Itertools;

advent_of_code::solution!(19);

type Pattern<'a> = &'a [Colour];

// struct Trie {}

// impl Trie {
//     fn new() {
//         //
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Colour(u8);

trait ConvertToSlice {
    fn print(&self);
    fn display(&self) -> String;
}

impl<'a> ConvertToSlice for Pattern<'a> {
    fn print(&self) {
        let k = self.display();
        println!("{k}");
    }
    fn display(&self) -> String {
        String::from_utf8(self.iter().map(|x| x.to()).collect_vec()).unwrap()
    }
}

const COLOURS: u8 = 5;
const DEFINED_CHARS: &[u8; COLOURS as usize] = b"wubrg";
const NON_COLOUR: u8 = u8::MAX;

impl Colour {
    fn from(char: u8) -> Self {
        Colour(
            DEFINED_CHARS
                .iter()
                .position(|&x| x == char)
                .map(|x| x.try_into().expect("Expected happy number"))
                .unwrap_or(u8::MAX),
        )
    }

    fn to(&self) -> u8 {
        DEFINED_CHARS[self.0 as usize]
    }
}

struct Input<'a> {
    raw: &'a [Colour],
    patterns: Vec<&'a [Colour]>,
    goals: Vec<&'a [Colour]>,
}
impl<'a> Input<'a> {
    fn new(raw: &'a [Colour], pattern_lengths: Vec<usize>, goal_lengths: Vec<usize>) -> Self {
        assert_eq!(
            raw.len(),
            pattern_lengths.iter().copied().sum::<usize>()
                + goal_lengths.iter().copied().sum::<usize>()
        );

        let mut patterns = vec![];
        let mut goals = vec![];

        let mut i = 0;
        for &len in &pattern_lengths {
            let slice = &raw[i..i + len];
            patterns.push(slice);
            i += len;
        }

        for &len in &goal_lengths {
            let slice = &raw[i..i + len];
            patterns.push(slice);
            i += len;
        }
        Input {
            raw,
            patterns,
            goals,
        }
    }
}

// impl<'a> Input<'a> {
fn write_slices<'a>(
    raw: &'a [Colour],
    patterns: &mut Vec<&'a [Colour]>,
    goals: &mut Vec<&'a [Colour]>,
    pattern_lengths: Vec<usize>,
    goal_lengths: Vec<usize>,
) {
    // for (vec, lengths) in [(&mut patterns, pattern_lengths), (&mut goals, goal_lengths)] {
    // let mut i = 0;
    // for len in lengths {
    //     vec.push(&raw[i..i + len]);
    //     i += len;
    // }
    // }
}
// }

fn parse<'a>(input: &'a str) -> (Box<[Colour]>, Vec<usize>, Vec<usize>) {
    let mut parts = input.split("\n\n");
    let mut patterns = parts
        .next()
        .expect("Expected first part of input")
        .split(", ")
        .map(|x| x.as_bytes())
        .collect_vec();
    let goals = parts
        .next()
        .expect("Expected second part of input")
        .split_whitespace()
        .map(|x| x.as_bytes())
        .collect_vec();

    patterns.sort_unstable();
    let raw: Box<[Colour]> = patterns
        .iter()
        .chain(goals.iter())
        .map(|&x| x.iter().copied().map(Colour::from))
        .flatten()
        .collect();
    (
        raw,
        patterns.iter().map(|x| x.len()).collect(),
        goals.iter().map(|x| x.len()).collect(),
    )
}

fn find_prefixes_binary<'a, 'b>(
    goal: Pattern<'b>,
    patterns: &'a [Pattern<'a>],
) -> &'a [Pattern<'a>] {
    let last = patterns.binary_search(&goal);
    match last {
        Ok(index) => &patterns[index..index + 1],
        Err(index) => {
            let mut end = index;
            while end > 0 && !goal.starts_with(patterns[end - 1]) {
                end -= 1;
            }
            let mut k = end;
            while k > 0 && goal.starts_with(patterns[k - 1]) {
                k -= 1;
            }
            &patterns[k..end]
        }
    }
}
fn find_prefixes_linear<'a, 'b>(
    goal: Pattern<'b>,
    patterns: &'a [Pattern<'a>],
) -> impl 'a + Iterator<Item = Pattern<'a>> {
    let goal = goal.to_owned();
    patterns
        .iter()
        .filter(move |x| goal.starts_with(x))
        .copied()
}

fn find_prefixes<'a, 'b>(
    goal: Pattern<'b>,
    patterns: &'a [Pattern<'a>],
) -> impl 'a + Iterator<Item = Pattern<'a>> {
    find_prefixes_linear(goal, patterns)
}

fn find_prefixes_collect<'a, 'b>(
    goal: Pattern<'b>,
    patterns: &'a [Pattern<'a>],
) -> Vec<Pattern<'a>> {
    find_prefixes(goal, patterns).collect_vec()
}

fn is_producable<'a>(goal: Pattern<'a>, patterns: &'a [Pattern<'a>]) -> Option<Vec<Pattern<'a>>> {
    goal.print();
    if goal.len() == 0 {
        Some(vec![])
    } else {
        find_prefixes(goal, patterns)
            // .inspect(|&&pattern| pattern.print())
            .map(|prefix| is_producable(&goal[prefix.len()..], patterns).map(|x| (prefix, x)))
            .flatten()
            .map(|(prefix, mut vec)| {
                vec.push(prefix);
                vec
            })
            .next()
    }
}

// Get up to depth = 2
fn jump_points() {
    //
}

fn inspect(goal: Pattern, production: Option<&Vec<Pattern>>) {
    let process = match production {
        Some(ps) => format!(
            "{}\u{001b}[m",
            ps.iter()
                .rev()
                .enumerate()
                .map(|(i, x)| {
                    format!(
                        "\u{001b}[{}m{}",
                        if i % 2 == 0 { "31" } else { "33" },
                        x.display()
                    )
                })
                .join("")
        ),
        None => "\timpossible".to_string(),
    };
    println!("{}\n{process}\n", goal.display());
}

pub fn part_one(input: &str) -> Option<u64> {
    let (raw, pattern_lens, goal_lens) = parse(input);
    let input = Input::new(&raw, pattern_lens, goal_lens);
    Some(
        input
            .goals
            .iter()
            .map(|goal| (goal, is_producable(goal, &input.patterns)))
            .inspect(|(goal, x)| {
                inspect(goal, x.as_ref());
            })
            .map(|x| x.1)
            .flatten()
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // Input patterns are already sorted, it is different from the example txt in this way.
    const INPUT: &str = "\
b, br, bwu, g, gb, r, rb, wr

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    #[test]
    fn test_prefix() {
        let (patterns, goals) = parse(&INPUT);
        assert_eq!(goals[0], b"brwrr");
        let prefix_patterns = find_prefixes_collect(goals[0], &patterns);
        assert_eq!(prefix_patterns, &patterns[0..2]);

        assert_eq!(find_prefixes_collect(b"zzzz", &patterns), [[]; 0]);
    }

    #[test]
    fn test_producable() {
        let (patterns, goals) = parse(&INPUT);
        assert_eq!(goals[0], b"brwrr");
        assert!(is_producable(b"brwrr", &patterns).is_some());
        assert!(!is_producable(b"ubwu", &patterns).is_some());

        assert!(is_producable(b"bbbbbbbbb", &patterns).is_some());
        assert!(!is_producable(b"wrwrwrwrwrwwr", &patterns).is_some());
        assert!(is_producable(b"wr", &patterns).is_some());
        assert!(is_producable(b"", &patterns).is_some());

        assert!(is_producable(b"bbbbbbbbbb", &patterns).is_some());
        assert!(is_producable(b"bwuwrbwu", &patterns).is_some());
        assert!(is_producable(b"bgb", &patterns).is_some());

        assert!(is_producable(b"bggrb", &patterns).is_some());
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
