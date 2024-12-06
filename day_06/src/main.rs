use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Neg},
};

static TEST_INPUT_PART_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
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
}

fn grid_iterator(width: i32, height: i32) -> impl Iterator<Item = Vector2d> {
    (0..height).flat_map(move |y| (0..width).map(move |x| Vector2d(x, y)))
}

impl Vector2d {
    fn rotate_right(self) -> Self {
        Vector2d(-self.1, self.0)
    }
}

fn part_1(input: &str) -> i64 {
    let mut visited = HashSet::new();
    let grid = Grid::new(input);

    let start = grid_iterator(grid.width as i32, grid.height as i32)
        .find(|x| grid.get(*x) == Some(b'^'))
        .unwrap();

    let mut direction = Vector2d(0, -1);
    let mut current_position = start;

    while grid.get(current_position).is_some() {
        while grid.get(current_position + direction) == Some(b'#') {
            direction = direction.rotate_right();
        }
        visited.insert(current_position);

        current_position += direction;
    }

    visited.len() as i64
}

fn gets_stuck(grid: &Grid, start: Vector2d, extra_position: Vector2d) -> bool {
    let mut visited = HashSet::new();

    let mut direction = Vector2d(0, -1);
    let mut current_position = start;

    while grid.get(current_position).is_some() {
        while current_position + direction == extra_position
            || grid.get(current_position + direction) == Some(b'#')
        {
            direction = direction.rotate_right();
        }
        if visited.contains(&(current_position, direction)) {
            return true;
        }
        visited.insert((current_position, direction));

        current_position += direction;
    }

    false
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);

    let start = grid_iterator(grid.width as i32, grid.height as i32)
        .find(|x| grid.get(*x) == Some(b'^'))
        .unwrap();

    grid_iterator(grid.width as i32, grid.height as i32)
        .filter(|x| gets_stuck(&grid, start, *x))
        .count() as i64
}

static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 41);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 6);
}
