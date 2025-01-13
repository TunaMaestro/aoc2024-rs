use core::panic;
use std::collections::{BinaryHeap, HashSet};

use aoc_utils::grid::{Grid, Point, UP_RIGHT_DOWN_LEFT};
use lina::vec2;

advent_of_code::solution!(16);

enum Tile {
    Empty,
    Wall,
}

#[derive(Eq, PartialEq)]
struct Visit {
    p: Point,
    score: u64,
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score).map(|x| x.reverse())
    }
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
    //
}

const STRAIGHT_COST: u64 = 1;
const TURN_COST: u64 = 1000;

fn weight(v: Point, w: Point, prev: Option<Point>) -> u64 {
    let p = prev.unwrap_or_else(|| v - vec2(1, 0));
    STRAIGHT_COST + if w - v == v - p { 0 } else { TURN_COST }
}

fn dijkstra(g: Grid<Tile>, start: Point) -> (Grid<Option<u64>>, Grid<Vec<Point>>) {
    let mut visit: BinaryHeap<Visit> = BinaryHeap::new();
    let mut dist = Grid::new_with_dimensions_uniform(g.dimension(), None);
    let mut pred = Grid::new_with_dimensions(g.dimension(), |_| Vec::new());

    visit.push(Visit { p: start, score: 0 });
    // dist[start] = Some(0);

    while let Some(Visit { p: node, score: k }) = visit.pop() {
        // if dist[node].map(|best_dist| k > best_dist).unwrap_or(false) {
        //     continue;
        // }
        dist[node] = Some(k);
        for d in UP_RIGHT_DOWN_LEFT {
            //
            let neighbour = node + d;
            if !g.contains(neighbour) {
                continue;
            }
            if let Tile::Wall = g[neighbour] {
                continue;
            }
            let cost = weight(node, neighbour, pred[node].get(0).copied());
            let neighbour_cost = k + cost;
            let neighbour_cmp_existing = dist[neighbour]
                .map(|existing_cost| neighbour_cost.cmp(&existing_cost))
                .unwrap_or(std::cmp::Ordering::Less);
            if let (std::cmp::Ordering::Equal | std::cmp::Ordering::Less) = neighbour_cmp_existing {
                dist[neighbour] = Some(neighbour_cost);
                if neighbour_cmp_existing == std::cmp::Ordering::Less {
                    // pred[neighbour].clear();
                }
                pred[neighbour].push(node);
                visit.push(Visit {
                    p: neighbour,
                    score: neighbour_cost,
                });
            }
        }
    }
    (dist, pred)
}

// fn predecessors<'a>(pred: &'a Grid<Vec<Point>>, end: Point) -> impl Iterator<Item = Point> {
//     let p = &pred[end];
//     p.iter().map(|&prev| predecessors(pred, prev)).flatten()
// }
fn predecessors_r(
    pred: &Grid<Vec<Point>>,
    from: Point,
    mut points: HashSet<Point>,
) -> HashSet<Point> {
    println!("Predecessors of {from:?} = {:?}", &pred[from]);
    for &p in pred[from].iter() {
        // points.insert(p);
        points = predecessors_r(pred, p, points);
    }
    points
}

fn predecessors(pred: &Grid<Vec<Point>>, from: Point) -> HashSet<Point> {
    predecessors_r(pred, from, HashSet::new())
}

fn solve(input: &str) -> (u64, u64) {
    let grid = Grid::read(input, |x| x);
    let start = grid
        .position(|&c| c == 'S')
        .expect("Expected start and end in input grid");
    let end = grid
        .position(|&c| c == 'E')
        .expect("Expected start and end in input grid");
    let grid = grid.map(|x| match x {
        '.' | 'S' | 'E' => Tile::Empty,
        '#' => Tile::Wall,
        _ => panic!("Unexpected character in input"),
    });
    let (dist, pred) = dijkstra(grid, start);
    dist.map(|x| format!("{:>6}", x.map(|i| i.to_string()).unwrap_or("∞".to_owned())))
        .print();

    assert!(dist[end].is_some(), "A distance should have been found.");

    (
        dist[end].expect("Expected path to Exit"),
        predecessors(&pred, end).len() as u64,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input).0)
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(16));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
