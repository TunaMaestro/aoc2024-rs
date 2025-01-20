use std::collections::{BinaryHeap, HashSet, VecDeque};

use aoc_utils::{
    grid::{Grid, Point, UP_RIGHT_DOWN_LEFT},
    parse,
};
use itertools::Itertools;
use lina::{point2, vec2, Point2};

advent_of_code::solution!(18);

type I = u64;

fn parse(input: &str) -> (Grid<Option<I>>, Vec<Point>) {
    let nums = parse::nums_positive::<i32>(input);
    let coords: Vec<_> = nums
        .chunks_exact(2)
        .map(|slice| point2(slice[0], slice[1]))
        .collect();
    let size = coords
        .iter()
        .copied()
        .reduce(|acc, e| acc.zip_map(e, i32::max))
        .expect("Expects at least one point")
        + vec2(1, 1);

    let mut grid = Grid::new_with_dimensions_uniform(size.to_vec(), None);

    for (i, p) in coords.iter().rev().enumerate() {
        grid[*p] = Some(i as I);
        // grid.map(|x| if x.is_some() { "#" } else { "." }).print();
        // println!()
    }
    (grid, coords)
}

// The time offset means that find the shortest path from END assuming that END
// is reached time_offset ns before the last obstacle falls
fn dijkstra(grid: &Grid<Option<I>>, is_empty: impl Fn(Option<I>, I) -> bool) -> Option<I> {
    let mut queue = BinaryHeap::new();
    let mut dist = Grid::new_with_dimensions_uniform(grid.dimension(), I::MAX);
    let mut prev = Grid::new_with_dimensions_uniform(grid.dimension(), None);
    let end = dist.dimension().to_point() - vec2(1, 1);
    queue.push(Visit::new(end, 0));

    while let Some(Visit { p, score: k }) = queue.pop() {
        if k <= dist[p] {
            // k is the distance from the END to the current.
            // This means that there is k time before the end.
            // If the neighbour's time is 5, then 5 ns before
            // the end, a block will fall, therefore, if we are
            // more than 5 steps away from the end, the position
            // will be open.
            dist[p] = k;
            for (neighbour, &block_time) in grid.neighbours(p) {
                if !is_empty(block_time, k) {
                    continue;
                }
                if k + 1 >= dist[neighbour] {
                    continue;
                }
                let n_dist = k + 1;
                queue.push(Visit::new(neighbour, n_dist));
                dist[neighbour] = n_dist;
                prev[neighbour] = Some(p);
            }
        }
    }

    // let path: HashSet<Point> = {
    //     let mut start = point2(0, 0);
    //     core::iter::once(start)
    //         .chain(core::iter::from_fn(|| {
    //             start = prev[start]?;
    //             Some(start)
    //         }))
    //         .collect()
    // };

    // Grid::new_with_dimensions(dist.dimension(), |coord| {
    //     let mut d = match dist[coord] {
    //         I::MAX => "   #".to_owned(),
    //         x => format!("{x:>4}"),
    //     };
    //     if !is_empty(grid[coord], 0) {
    //         d = format!("\u{001b}[7m   #\u{001b}[m");
    //     }
    //     if path.contains(&coord) {
    //         // d = format!("\u{001b}[22m{d}\u{001b}[m");
    //         d = format!("\u{001b}[31m{d}\u{001b}[m");
    //     }
    //     d
    // })
    // .print();
    // println!();
    let res = dist[point2(0, 0)];
    if res != I::MAX {
        Some(res)
    } else {
        None
    }
}

fn bfs(grid: &Grid<Option<I>>, is_empty: impl Fn(Option<I>, I) -> bool) -> Option<I> {
    let mut queue = VecDeque::new();
    let mut dist = Grid::new_with_dimensions_uniform(grid.dimension(), None);
    let mut prev = Grid::new_with_dimensions_uniform(grid.dimension(), None);
    let end = dist.dimension().to_point() - vec2(1, 1);

    dist[end] = Some(0);

    queue.push_back(end);

    while let Some(v) = queue.pop_front() {
        if v == point2(0, 0) {
            break;
        }
        for (w, &time) in grid.neighbours(v) {
            if !is_empty(time, 0) {
                continue;
            }
            if dist[w].is_none() {
                dist[w] = dist[v].map(|x| x + 1);
                prev[w] = Some(v);
                queue.push_back(w);
            }
        }
    }
    dist[point2(0, 0)]
}

fn fucking_idk_indiana_jones_time_where_falling_cuts_off_current_path_takes_time_to_move(
    block_time: Option<u64>,
    k: u64,
) -> bool {
    // !block_time.map(|t| t < k + time_offset).unwrap_or(true)
    true
}

pub fn part_one(input: &str) -> Option<I> {
    const AFTER_STEPS: I = 1024;
    let (grid, coords) = parse(input);
    let total_time = coords.len() as I;
    bfs(&grid, drop_until_time(total_time - AFTER_STEPS))
}

fn drop_until_time(sim_time: I) -> impl Fn(Option<I>, I) -> bool {
    move |obstacle_time: Option<I>, k: I| !obstacle_time.map(|x| x >= sim_time).unwrap_or(false)
}

fn shit_that_i_hallucinated_being_the_question(input: &str) -> Option<I> {
    let (grid, coords) = parse(input);
    let total_time = coords.len();
    (0..coords.len())
        // .rev()
        .map(|x| dijkstra(&grid, fucking_idk_indiana_jones_time_where_falling_cuts_off_current_path_takes_time_to_move).map(|len| (x, len)))
        .flatten()
        .inspect(|x| {
            println!(
                "
                Offset:  {}
                Elapsed: {}
                Path:    {}",
                x.0,
                total_time - x.0,
                x.1
            );
        })
        .filter(|&(time_offset, len)| len == (total_time - time_offset) as I)
        .map(|(time_offset, len)| len)
        .next()
}

pub fn part_two(input: &str) -> Option<String> {
    const AFTER_STEPS: I = 1024;

    let (grid, coords) = parse(input);
    let total_time = coords.len() as I;

    let f = |n| bfs(&grid, drop_until_time(total_time - n)).is_some();
    let steps = {
        let mut size = total_time;
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let mid = base + half;

            let cmp = f(mid);

            if cmp {
                base = mid;
            }

            size -= half;
        }

        base + 1
    };
    // println!("Steps: {steps}");
    let ans = coords[(steps - 1) as usize];
    Some(format!("{},{}", ans.x, ans.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_owned()));
    }
}

#[derive(Eq, PartialEq)]
struct Visit {
    p: Point,
    score: u64,
}

impl Visit {
    fn new(p: Point, score: u64) -> Self {
        Self { p, score }
    }
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
