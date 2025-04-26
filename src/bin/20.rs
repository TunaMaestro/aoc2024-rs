use std::{
    collections::{HashMap, VecDeque},
    convert::identity,
    fmt::Display,
    ops::Deref,
};

use aoc_utils::grid::{Grid, Point, UP_RIGHT_DOWN_LEFT};
use itertools::Itertools;

advent_of_code::solution!(20);

type I = u64;
type Tile = bool;

fn parse(input: &str) -> (Grid<Tile>, Point, Point) {
    let g = Grid::read(input, identity);
    let start = g
        .position(|&x| x == 'S')
        .expect("Expected start 'S' in grid");
    let end = g.position(|&x| x == 'E').expect("Expected end 'E' in grid");

    (g.map(|&x| x == '#'), start, end)
}

fn bfs(
    grid: &Grid<Tile>,
    src: Point,
    max_depth: I,
    mut f: impl FnMut(Point, I) -> (),
    check_walls: bool,
) -> Grid<Option<I>> {
    let mut queue = VecDeque::new();
    let mut dist = Grid::new_with_dimensions_uniform(grid.dimension(), None);
    dist[src] = Some(0);
    queue.push_back((src, 0));

    while let Some((v, d_v)) = queue.pop_front() {
        f(v, d_v);
        for (w, &wall) in grid.neighbours(v) {
            if check_walls && wall {
                continue;
            }
            if dist[w].is_none() && d_v + 1 <= max_depth {
                dist[w] = Some(d_v + 1);
                queue.push_back((w, d_v + 1));
            }
        }
    }
    dist
}

const CHEAT_THRESHOLD: i64 = 100;
const PICO_DIST: I = 20;

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, start, end) = parse(input);

    let dists = bfs(&grid, end, I::MAX, |_, _| {}, true);
    Some(
        cheat_gains(&dists)
            .filter(|&x| x >= CHEAT_THRESHOLD)
            .count() as u64,
    )
}

fn group(input: &str) -> HashMap<i64, usize> {
    let (grid, start, end) = parse(input);

    let dists = bfs(&grid, end, I::MAX, |_, _| {}, true);
    cheat_gains(&dists).counts()
}

fn group2(input: &str, cheat_time: I) -> HashMap<i64, usize> {
    let (grid, start, end) = parse(input);

    let dists = bfs(&grid, end, I::MAX, |_, _| {}, true);
    dists
        .iter_coordinates()
        .map(|src| secondary_search(&grid, &dists, src, cheat_time))
        .flatten()
        .map(|x| x.1 as _)
        .counts()
}

fn cheat_gains<'a>(dists: &'a Grid<Option<u64>>) -> impl 'a + Iterator<Item = i64> {
    dists
        .iter_coordinates()
        .map(|x| UP_RIGHT_DOWN_LEFT.iter().map(move |&d| (x, x + d + d)))
        .flatten()
        .flat_map(|(from, to)| {
            let src = *dists.get(from)?.as_ref()?;
            let dest = *dists.get(to)?.as_ref()?;
            // order doesn't matter because the cheat from both directions will be covered
            Some(-2 + (dest as i64) - (src as i64))
        })
        .filter(|&x| x > 0)
    // .inspect(|x| println!("{x}"))
}

fn secondary_search(
    grid: &Grid<Tile>,
    dists: &Grid<Option<u64>>,
    src: Point,
    cheat_time: I,
) -> Vec<((Point, Point), I)> {
    let mut bounds = vec![];

    let src_cost = if let Some(src_cost) = dists[src] {
        src_cost
    } else {
        return bounds;
    };

    bfs(
        grid,
        src,
        cheat_time,
        |dest, dist| {
            if dist == 1 {
                return;
            }
            if let Some(to) = dists[dest] {
                let save = (src_cost as i64 - to as i64 - dist as i64);
                // dbg!(src, dest, dist, src_cost, to);
                // dbg!(save);
                // println!();
                if save > 0 {
                    bounds.push(((src, dest), save as _));
                }
            }
        },
        false,
    );
    bounds
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, start, end) = parse(input);

    let dists = bfs(&grid, end, I::MAX, |_, _| {}, true);

    Some(
        dists
            .iter_coordinates()
            .map(|src| secondary_search(&grid, &dists, src, PICO_DIST))
            .flatten()
            .filter(|&x| x.1 as i64 >= CHEAT_THRESHOLD)
            .count() as _,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = group(&advent_of_code::template::read_file("examples", DAY));
        let exp = grouped1();
        println!("{:?}", &result.iter().sorted());
        println!("{:?}", &exp.iter().sorted());
        assert_eq!(result, exp);
    }

    fn grouped1() -> HashMap<i64, usize> {
        HashMap::from_iter([
            (64, 1),
            (40, 1),
            (38, 1),
            (36, 1),
            (20, 1),
            (12, 3),
            (10, 2),
            (8, 4),
            (6, 2),
            (4, 14),
            (2, 14),
        ])
    }

    #[test]
    fn test_part_two() {
        let input = &advent_of_code::template::read_file("examples", DAY);

        let result = group2(input, 1);
        assert_eq!(result, HashMap::new());

        let result = group2(input, 2);

        p_counts(&result);
        p_counts(&self::grouped1());

        assert_eq!(result, self::grouped1());

        let mut result = group2(input, PICO_DIST);
        result.retain(|&a, _| a >= 50);
        let exp = grouped2();
        p_counts(&result);
        p_counts(&exp);
        assert_eq!(result, exp);

        let (grid, start, end) = parse(input);

        let dists = bfs(&grid, end, I::MAX, |_, _| {}, true);

        let k: HashMap<_, _> = dists
            .iter_coordinates()
            .map(|src| secondary_search(&grid, &dists, src, PICO_DIST))
            .flatten()
            .filter(|x| x.1 as i64 >= CHEAT_THRESHOLD)
            .collect();
    }

    fn grouped2() -> HashMap<i64, usize> {
        HashMap::from_iter([
            (50, 32),
            (52, 31),
            (54, 29),
            (56, 39),
            (58, 25),
            (60, 23),
            (62, 20),
            (64, 19),
            (66, 12),
            (68, 14),
            (70, 12),
            (72, 22),
            (74, 4),
            (76, 3),
        ])
    }

    #[test]
    fn test_input_part_one() {
        let result = group(&advent_of_code::template::read_file("inputs", DAY));
        p_counts(&result);
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1507));
    }
}

fn p_counts<A: Display + Ord, B: Display + Ord>(counts: &HashMap<A, B>) {
    counts
        .iter()
        .sorted_unstable()
        .inspect(|(a, b)| println!("{a:>4} => {b}"))
        .count();
    println!()
}
