use std::collections::HashMap;

use aoc_utils::grid::{Grid, Point};
use itertools::Itertools;
use lina::point2;

advent_of_code::solution!(8);

pub fn read_antennas(input: &str) -> (Grid<u8>, HashMap<u8, Vec<Point>>) {
    let grid = Grid::read(input, |x| x as u8);

    let mut antennae: HashMap<u8, Vec<Point>> = HashMap::new();

    grid.0.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, &c)| {
            if c == b'.' || c == b'#' {
                return;
            }
            if !antennae.contains_key(&c) {
                antennae.insert(c, Vec::new());
            }
            antennae
                .get_mut(&c)
                .unwrap()
                .push(point2(x as i32, y as i32))
        })
    });

    (grid, antennae)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (grid, antennae) = read_antennas(input);

    Some(
        antennae
            .into_iter()
            .map(|(_label, antennae_group)| {
                let pairs: Vec<(Point, Point)> = antennae_group
                    .iter()
                    .enumerate()
                    .flat_map(|(i, a)| antennae_group[i..].iter().map(|b| (*a, *b)))
                    .collect::<Vec<_>>();
                pairs
                    .iter()
                    .filter(|(a, b)| a != b)
                    // .inspect(|(a, b)| {
                    //     eprintln!(
                    //         "{}: ({:>2}, {:>2}) - ({:>2}, {:>2})",
                    //         label as char,
                    //         a.x,
                    //         a.y,
                    //         b.x,
                    //         b.y + 1
                    //     )
                    // })
                    .map(|&(a, b)| {
                        let diff = a - b;
                        let n1 = a + diff;
                        let n2 = b - diff;
                        [n1, n2]
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .filter(|&x| grid.contains(x))
            .unique()
            // .inspect(|x| eprintln!("({:>2}, {:>2})\t({:>2}, {:>2})", x.x, x.y, x.x + 1, x.y + 1))
            .count(),
    )
}

fn add_group(antinodes: &mut Grid<bool>, antennae: &[Point]) {
    let pairs: Vec<(Point, Point)> = antennae
        .iter()
        .enumerate()
        .flat_map(|(i, a)| antennae[i + 1..].iter().map(|b| (*a, *b)))
        .collect::<Vec<_>>();

    for (a, b) in pairs {
        // dbg!(a, b);
        let diff = a - b;
        let mut node = a;
        while antinodes.contains(node) {
            antinodes[node] = true;
            node += diff;
        }
        node = b;
        while antinodes.contains(node) {
            antinodes[node] = true;
            node -= diff;
        }
    }
}
pub fn part_two(input: &str) -> Option<usize> {
    let (grid, antennae) = read_antennas(input);

    let mut antinode = Grid::new_with_dimensions_uniform(grid.dimension(), false);
    for (_label, antennae_group) in antennae.iter() {
        add_group(&mut antinode, &antennae_group);
    }
    Some(antinode.0.concat().iter().map(|&x| x as usize).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(12));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
