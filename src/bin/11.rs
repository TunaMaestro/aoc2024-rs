#![feature(map_try_insert)]
use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn parse(input: &str) -> Vec<u64> {
    let vals: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().expect("Expect integer"))
        .collect();
    vals
}

fn blink(s: u64) -> (u64, Option<u64>) {
    if s == 0 {
        (1, None)
    } else {
        let digits = s.ilog10() + 1;
        if digits % 2 == 0 && true {
            let base_10_mask = 10u64.pow(digits / 2);
            let new_left_val = s / base_10_mask;
            let new_right_val = s % base_10_mask;
            (new_left_val, Some(new_right_val))
        } else {
            (s * 2024, None)
        }
    }
}

fn upsert(m: &mut HashMap<u64, u64>, k: u64, c: u64) {
    // p(m);
    match m.try_insert(k, c) {
        Ok(_) => (),
        Err(mut entry) => {
            *entry.entry.get_mut() += c;
        }
    }
    // p(m);
}

fn blink_all(
    main: HashMap<u64, u64>,
    mut aux: HashMap<u64, u64>,
) -> (HashMap<u64, u64>, HashMap<u64, u64>) {
    aux.clear();
    for (&k, &c) in main.iter() {
        let (a, b) = blink(k);
        // dbg!(k, c, a, b);
        upsert(&mut aux, a, c);
        if let Some(snd) = b {
            upsert(&mut aux, snd, c);
        }
    }
    (aux, main)
}

fn p(m: &HashMap<u64, u64>) {
    let s = m.iter().map(|(&k, &v)| format!("({} {})", k, v)).join(", ");
    println!("{s}\n",);
}

fn iterate(input: &str, turns: i32) -> u64 {
    let mut main: HashMap<u64, u64> = HashMap::new();
    for k in parse(input) {
        main.insert(k, *main.get(&k).unwrap_or(&0) + 1);
    }
    let mut aux: HashMap<u64, u64> = HashMap::new();

    for _ in 0..turns {
        // dbg!(main.len(), main.values().map(|&x| x).sum::<u64>());
        // p(&main);
        (main, aux) = blink_all(main, aux);
    }

    main.values().map(|&x| x).sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(iterate(input, 25))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(iterate(input, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_upsert() {
        let mut m = HashMap::new();

        upsert(&mut m, 5, 2);
        assert_eq!(m, HashMap::from([(5, 2)]));
        upsert(&mut m, 5, 4);
        assert_eq!(m, HashMap::from([(5, 6)]));
        upsert(&mut m, 5, 4);
        assert_eq!(m, HashMap::from([(5, 10)]));
        upsert(&mut m, 6, 4);
        assert_eq!(m, HashMap::from([(5, 10), (6, 4)]));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
