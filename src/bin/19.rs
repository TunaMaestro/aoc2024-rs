#![feature(let_chains)]

use aoc_utils::debug::PrintBytes;
use std::{array, collections::HashMap, io::BufRead, ops::Index, rc::Rc, sync::Arc};

use itertools::Itertools;

advent_of_code::solution!(19);

type Pattern<'a> = &'a [Colour];

type Colour = u8;

const COLOURS: u8 = 5;
const DEFINED_CHARS: &[u8; COLOURS as usize] = b"bgruw";
const NON_COLOUR: u8 = u8::MAX;

fn from(char_byte: u8) -> u8 {
    DEFINED_CHARS.iter().position(|&x| x == char_byte).unwrap() as u8
}

fn to(ordinal: u8) -> u8 {
    DEFINED_CHARS[ordinal as usize]
}

fn parse<'a>(input: &'a str) -> (Vec<Pattern<'a>>, Vec<Pattern<'a>>) {
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
    (patterns, goals)
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

type Out<'a> = Option<Vec<Pattern<'a>>>;

fn hash(k: Pattern<'_>) -> u64 {
    k.iter()
        .copied()
        .map(from)
        .enumerate()
        .map(|(i, x)| (5u64).pow(i as u32) * (x as u64))
        .sum()
}

const MAX_MEMO_LEN: usize = 100;

fn p_hash(memo: &mut HashMap<&'_ [Colour], Out<'_>>) {
    (memo
        .iter()
        .map(|(&k, v)| {
            inspect(k, v.as_ref());
        })
        .count());
}

fn is_producable<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
    memo: &mut HashMap<&'a [Colour], Out<'a>>,
) -> Out<'a> {
    // if goal.len() <= MAX_MEMO_LEN {
    //     dbg!(hash(goal));
    // }
    if goal.len() == 0 {
        Some(vec![])
    } else {
        // println!("LOOKING UP:");
        // goal.print();
        if goal.len() <= MAX_MEMO_LEN
            && let Some(existing) = memo.get(goal)
        {
            // println!("ALREADY IN HASH");
            return existing.clone();
        }
        // p_hash(memo);
        let res = find_prefixes(goal, patterns)
            // .inspect(|&&pattern| pattern.print())
            .map(|prefix| is_producable(&goal[prefix.len()..], patterns, memo).map(|x| (prefix, x)))
            .flatten()
            .map(|(prefix, mut vec)| {
                vec.push(prefix);
                vec
            })
            .next();
        if goal.len() <= MAX_MEMO_LEN {
            memo.insert(goal, res.clone());
        }
        res
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
    let (patterns, goals) = parse(input);
    let mut memo = HashMap::new();
    Some(
        goals
            .iter()
            .map(|goal| (goal, is_producable(goal, &patterns, &mut memo)))
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
    fn test_hash() {
        assert_ne!(hash(b"bwrgwb"), hash(b"bwrgww"));
    }
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
        let mut hs = HashMap::new();
        assert_eq!(goals[0], b"brwrr");
        assert!(is_producable(b"brwrr", &patterns, &mut hs).is_some());
        assert!(!is_producable(b"ubwu", &patterns, &mut hs).is_some());

        assert!(is_producable(b"bbbbbbbbb", &patterns, &mut hs).is_some());
        assert!(!is_producable(b"wrwrwrwrwrwwr", &patterns, &mut hs).is_some());
        assert!(is_producable(b"wr", &patterns, &mut hs).is_some());
        assert!(is_producable(b"", &patterns, &mut hs).is_some());

        assert!(is_producable(b"bbbbbbbbbb", &patterns, &mut hs).is_some());
        assert!(is_producable(b"bwuwrbwu", &patterns, &mut hs).is_some());
        assert!(is_producable(b"bgb", &patterns, &mut hs).is_some());

        assert!(is_producable(b"bggrb", &patterns, &mut hs).is_some());
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
