use std::{ops::Deref, rc::Rc};

use itertools::Itertools;

advent_of_code::solution!(19);

type Pattern = Rc<[Colour]>;

trait ColourTrait {
    fn display(&self) -> String;
}

// struct Trie {}

// impl Trie {
//     fn new() {
//         //
//     }
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Colour(u8);

const COLOURS: u8 = 5;
const DEFINED_CHARS: &[u8; COLOURS as usize] = b"wubrg";
const NON_COLOUR: u8 = u8::MAX;

impl ColourTrait for &[Colour] {
    fn display(&self) -> String {
        String::from_utf8(self.iter().map(Colour::to).collect_vec()).unwrap()
    }
}

impl Colour {
    fn from(char: u8) -> Self {
        Colour(
            DEFINED_CHARS
                .iter()
                .position(|&x| x == char)
                .expect("all input characters should be one of the provided colours")
                .try_into()
                .unwrap(),
        )
    }

    fn to(&self) -> u8 {
        DEFINED_CHARS[self.0 as usize]
    }
}

fn parse(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
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

    let patterns = patterns
        .iter()
        .map(|&x| x.iter().map(|&x| Colour::from(x)).collect::<Pattern>())
        .collect();
    let goals = goals
        .iter()
        .map(|&x| x.iter().map(|&x| Colour::from(x)).collect::<Pattern>())
        .collect();
    (patterns, goals)
}

fn find_prefixes_binary<'a>(goal: Pattern, patterns: &'a [Pattern]) -> &'a [Pattern] {
    let last = patterns.binary_search(&goal);
    match last {
        Ok(index) => &patterns[index..index + 1],
        Err(index) => {
            let mut end = index;
            while end > 0 && !goal.starts_with(&patterns[end - 1]) {
                end -= 1;
            }
            let mut k = end;
            while k > 0 && goal.starts_with(&patterns[k - 1]) {
                k -= 1;
            }
            &patterns[k..end]
        }
    }
}
fn find_prefixes_linear<'a>(
    goal: &[Colour],
    patterns: &'a [Pattern],
) -> impl 'a + Iterator<Item = Pattern> {
    let goal = goal.to_owned();
    patterns
        .iter()
        .filter(move |x| goal.starts_with(x))
        .cloned()
}

fn find_prefixes<'a>(
    goal: &[Colour],
    patterns: &'a [Pattern],
) -> impl 'a + Iterator<Item = Pattern> {
    find_prefixes_linear(goal, patterns)
}

fn find_prefixes_collect<'a>(goal: &[Colour], patterns: &'a [Pattern]) -> Vec<Pattern> {
    find_prefixes(goal, patterns).collect_vec()
}

fn is_producable<'a>(goal: &[Colour], patterns: &'a [Pattern]) -> Option<Vec<Pattern>> {
    println!("{}", goal.display());
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

fn inspect(goal: &[Colour], production: Option<&Vec<Pattern>>) {
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
                        x.deref().display()
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
    Some(
        goals
            .iter()
            .map(|goal| (goal, is_producable(goal, &patterns)))
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
