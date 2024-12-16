use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Neg, Sub},
};

static TEST_INPUT_PART_1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

static TEST_INPUT_PART_2: &str = TEST_INPUT_PART_1;

struct Grid {
    width: usize,
    height: usize,
    text: Box<[u8]>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Vector2d(i32, i32);

impl From<(i32, i32)> for Vector2d {
    fn from(value: (i32, i32)) -> Self {
        Vector2d(value.0, value.1)
    }
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vector2d {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Neg for Vector2d {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Sub for Vector2d {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Self {
            width,
            height,
            text: input.as_bytes().into(),
        }
    }

    fn get(&self, v: Vector2d) -> Option<u8> {
        if !(0..self.width as i32).contains(&v.0) {
            return None;
        }
        if !(0..self.height as i32).contains(&v.1) {
            return None;
        }

        Some(self.text[(v.0 + v.1 * (self.width as i32 + 1)) as usize])
    }

    fn iter(&self) -> impl Iterator<Item = (Vector2d, u8)> + use<'_> {
        (0..self.height as i32)
            .flat_map(move |y| (0..self.width as i32).map(move |x| Vector2d(x, y)))
            .map(|v| (v, self.get(v).unwrap()))
    }
}

fn part_1(input: &str) -> i64 {
    fn count_paths(grid: &Grid, position: Vector2d, depth: u8, reached: &mut HashSet<Vector2d>) {
        let Some(x) = grid.get(position) else {
            return;
        };

        if x != depth {
            return;
        }

        if x == b'9' {
            reached.insert(position);
            return;
        }

        [(1, 0), (0, -1), (-1, 0), (0, 1)]
            .into_iter()
            .map(|(x, y)| Vector2d(x, y))
            .for_each(|x| count_paths(grid, x + position, depth + 1, reached))
    }

    let grid = Grid::new(input);
    grid.iter()
        .filter(|(_, x)| *x == b'0')
        .map(|(v, _)| {
            let mut s = HashSet::new();
            count_paths(&grid, v, b'0', &mut s);
            s.len() as i64
        })
        .sum()
}

fn part_2(input: &str) -> i64 {
    fn count_paths(grid: &Grid, position: Vector2d, depth: u8) -> i64 {
        let Some(x) = grid.get(position) else {
            return 0;
        };

        if x != depth {
            return 0;
        }

        if x == b'9' {
            return 1;
        }

        [(1, 0), (0, -1), (-1, 0), (0, 1)]
            .into_iter()
            .map(|(x, y)| Vector2d(x, y))
            .map(|x| count_paths(grid, x + position, depth + 1))
            .sum()
    }

    let grid = Grid::new(input);
    grid.iter()
        .filter(|(_, x)| *x == b'0')
        .map(|(v, _)| count_paths(&grid, v, b'0'))
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 36);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 81);
}
