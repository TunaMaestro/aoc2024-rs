use aoc_utils::grid::{orthogonal_to_index, Grid, Point, UP_RIGHT_DOWN_LEFT};
use lina::{point2, Vec2};

advent_of_code::solution!(12);

trait Fence: Sized + std::fmt::Debug + std::ops::Add<Self, Output = Self> {
    fn new() -> Self;
    fn cost(&self) -> usize;
    fn add_perimeter(&mut self, point: Point, direction: Vec2<i32>);
}

#[derive(Debug)]
struct SimpleFence {
    perimeter: usize,
    area: usize,
}

impl Fence for SimpleFence {
    fn new() -> SimpleFence {
        SimpleFence {
            perimeter: 0,
            area: 1,
        }
    }

    fn cost(&self) -> usize {
        self.perimeter * self.area
    }

    fn add_perimeter(&mut self, _point: Point, _direction: Vec2<i32>) {
        self.perimeter += 1;
    }
}

impl std::ops::Add for SimpleFence {
    type Output = SimpleFence;

    fn add(self, rhs: Self) -> Self::Output {
        SimpleFence {
            perimeter: self.perimeter + rhs.perimeter,
            area: self.area + rhs.area,
        }
    }
}

#[derive(Debug)]
struct HardFence {
    edges: Vec<Edge>,
    area: usize,
}

impl Fence for HardFence {
    fn new() -> Self {
        HardFence {
            edges: vec![],
            area: 1,
        }
    }

    fn cost(&self) -> usize {
        let edge_groups = group_by_dir(&self.edges);
        let sides: usize = edge_groups.into_iter().map(find_straights).sum();
        sides * self.area
    }

    fn add_perimeter(&mut self, point: Point, direction: Vec2<i32>) {
        self.edges.push(Edge {
            p: point,
            n: direction,
        });
    }
}

impl std::ops::Add for HardFence {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut edges = self.edges;
        edges.extend(rhs.edges);
        HardFence {
            edges,
            area: self.area + rhs.area,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Edge {
    p: Point,
    n: Vec2<i32>,
}

impl Edge {
    // Normalise so the edge's normal is facing RIGHT n=(1, 0)
    pub fn normalise(&self) -> Self {
        let mut e = *self;
        if e.n.y != 0 {
            // Rotate 90 deg
            (e.p.x, e.p.y) = (-e.p.y, e.p.x);
            (e.n.x, e.n.y) = (-e.n.y, e.n.x);
        }
        if e.n.x < 0 {
            (e.p.x, e.p.y) = (-e.p.x, -e.p.y);
            (e.n.x, e.n.y) = (-e.n.x, -e.n.y);
        }
        e
    }
}

fn group_by_dir(edges: &Vec<Edge>) -> [Vec<Edge>; 4] {
    let mut dirns: [Vec<Edge>; 4] = core::array::from_fn(|_| Vec::new());

    for e in edges {
        dirns[orthogonal_to_index(e.n)
            .expect("Expected all normals to be an orthogonal normal vector")]
        .push(e.normalise());
    }

    dirns
}

fn find_straights(mut edges: Vec<Edge>) -> usize {
    // Sort so that same x are contiguous, then ys are ordered.
    edges.sort_by_key(|x| (x.p.x, x.p.y));

    edges
        .chunk_by(|a, b| a.p.x == b.p.x)
        .map(|vertical_slice| vertical_slice.chunk_by(|a, b| a.p.y + 1 == b.p.y).count())
        .sum()
}
fn flood<T: Fence>(
    grid: &Grid<u8>,
    regions: &mut Grid<Option<u64>>,
    start: Point,
    label: u64,
) -> T {
    assert!(regions[start].is_none());
    regions[start] = Some(label);
    let mut f = T::new();
    let plant = grid[start];
    for d in UP_RIGHT_DOWN_LEFT {
        let next = start + d;
        if !grid.contains(next) || grid[next] != plant {
            f.add_perimeter(start, d);
            continue;
        }
        if regions[next].is_none() {
            f = f + flood(grid, regions, next, label);
        }
    }
    f
}

fn solve<T: Fence>(input: &str) -> usize {
    let grid = Grid::read(input, |x| x as u8);
    let mut regions: Grid<Option<u64>> = Grid::new_with_dimensions_uniform(grid.dimension(), None);

    let dimension = grid.dimension();
    let mut score = 0;
    let mut label = 0;
    for y in 0..dimension.y {
        for x in 0..dimension.x {
            let p = point2(x, y);
            if regions[p].is_some() {
                continue;
            }
            let s = flood::<T>(&grid, &mut regions, p, label);
            // dbg!(label, grid[p] as char, &s, s.cost());
            score += s.cost();
            label += 1;
        }
    }
    score
}
pub fn part_one(input: &str) -> Option<usize> {
    let score = solve::<SimpleFence>(input);
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let score = solve::<HardFence>(input);
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(140));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(80));
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(236));

        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
