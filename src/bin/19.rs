use aoc_utils::debug::PrintBytes;
use aoc_utils::ResultExt;
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
    patterns.reverse();
    (patterns, goals)
}

fn find_prefixes_binary<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
) -> impl 'a + Iterator<Item = Pattern<'a>> {
    let last = patterns.binary_search_by(|&el| el.cmp(goal).reverse());
    // match last {
    //     Ok(index) => &patterns[index..index + 1],
    //     Err(index) => {
    //         let mut end = index;
    //         while end > 0 && !goal.starts_with(patterns[end - 1]) {
    //             end -= 1;
    //         }
    //         let mut k = end;
    //         while k > 0 && goal.starts_with(patterns[k - 1]) {
    //             k -= 1;
    //         }
    //         &patterns[k..end]
    //     }
    // }

    let mut search = last.into_inner();
    core::iter::from_fn(move || {
        if search >= patterns.len() {
            return None;
        }
        if patterns[search][0] != goal[0] {
            return None;
        }
        let to_return = patterns[search];
        search += 1;

        Some(to_return)
    })
    .filter(|&x| goal.starts_with(x))
}
fn find_prefixes_linear<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
) -> impl 'a + Iterator<Item = Pattern<'a>> {
    patterns
        .iter()
        .filter(move |x| goal.starts_with(x))
        .copied()
}

fn find_prefixes<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
) -> impl 'a + Iterator<Item = Pattern<'a>> {
    // find_prefixes_linear(goal, patterns)
    find_prefixes_binary(goal, patterns)
}

fn find_prefixes_collect<'a>(goal: Pattern<'a>, patterns: &'a [Pattern<'a>]) -> Vec<Pattern<'a>> {
    find_prefixes(goal, patterns).collect_vec()
}

fn p_hash(memo: &mut HashMap<&'_ [Colour], Out2<'_>>) {
    (memo
        .iter()
        .map(|(&k, v)| {
            // inspect(k, v.as_ref());
            println!("{}: {}", k.display(), v.unwrap_or(0))
        })
        .count());
}

type Out<'a> = Option<Vec<Pattern<'a>>>;

fn is_producable<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
    memo: &mut HashMap<&'a [Colour], Out<'a>>,
) -> Out<'a> {
    if goal.len() == 0 {
        Some(vec![])
    } else {
        if let Some(existing) = memo.get(goal) {
            return existing.clone();
        }
        let res = find_prefixes(goal, patterns)
            .map(|prefix| is_producable(&goal[prefix.len()..], patterns, memo).map(|x| (prefix, x)))
            .flatten()
            .map(|(prefix, mut vec)| {
                vec.push(prefix);
                vec
            })
            .next();
        memo.insert(goal, res.clone());
        res
    }
}

type Out2<'a> = Option<u64>;

fn is_producable2<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
    memo: &mut HashMap<&'a [Colour], Out2<'a>>,
) -> Out2<'a> {
    if goal.len() == 0 {
        Some(1)
    } else {
        if let Some(&existing) = memo.get(goal) {
            return existing;
        }
        let res = find_prefixes(goal, patterns)
            .map(|prefix| {
                is_producable2(&goal[prefix.len()..], patterns, memo).map(|x| (prefix, x))
            })
            .flatten()
            .map(|(prefix, count)| count)
            .sum1();
        memo.insert(goal, res);
        res
    }
}

type Out3<'a> = Option<()>;

fn is_producable3<'a>(
    goal: Pattern<'a>,
    patterns: &'a [Pattern<'a>],
    memo: &mut HashMap<&'a [Colour], Out3<'a>>,
) -> Out3<'a> {
    if goal.len() == 0 {
        Some(())
    } else {
        if let Some(&existing) = memo.get(goal) {
            return existing;
        }
        let res = find_prefixes(goal, patterns)
            .map(|prefix| {
                is_producable3(&goal[prefix.len()..], patterns, memo).map(|x| (prefix, x))
            })
            .flatten()
            .map(|(prefix, existing)| existing)
            .next();
        memo.insert(goal, res);
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
            // .inspect(|(goal, x)| {
            //     inspect(goal, x.as_ref());
            // })
            .map(|x| x.1)
            .flatten()
            .count() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, goals) = parse(input);
    let mut memo = HashMap::new();
    Some(
        goals
            .iter()
            .map(|goal| (goal, is_producable2(goal, &patterns, &mut memo)))
            // .inspect(|(goal, x)| println!("{}: {}", goal.display(), x.unwrap_or(0)))
            .map(|x| x.1)
            .flatten()
            .sum(),
    )
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

    // #[test]
    // fn test_hash() {
    //     assert_ne!(hash(b"bwrgwb"), hash(b"bwrgww"));
    // }
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
        assert_eq!(result, Some(16));
    }

    #[test]
    fn test_real_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(327));
    }
}
