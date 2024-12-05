use std::ops::{Add, AddAssign, Neg};

static TEST_INPUT_PART_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
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

impl Vector2d {
    fn direction_vectors() -> impl Iterator<Item = Vector2d> {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| Vector2d(x, y)))
            .filter(|x| !(x.0 == 0 && x.1 == 0))
    }

    fn diagonals() -> impl Iterator<Item = Vector2d> {
        (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| Vector2d(x, y)))
            .filter(|x| x.0 != 0 && x.1 != 0)
    }
}

fn grid_iterator(width: i32, height: i32) -> impl Iterator<Item = Vector2d> {
    (0..height).flat_map(move |y| (0..width).map(move |x| Vector2d(x, y)))
}

impl Grid {
    fn has_xmas(&self, mut position: Vector2d, direction: Vector2d) -> bool {
        let xmas = "XMAS";
        for &letter in xmas.as_bytes() {
            if self.get(position) != Some(letter) {
                return false;
            }
            position += direction;
        }

        true
    }

    fn has_cross_mas(&self, position: Vector2d, direction: Vector2d) -> bool {
        self.get(position + -direction) == Some(b'M')
            && self.get(position) == Some(b'A')
            && self.get(position + direction) == Some(b'S')
    }
}

fn part_1(input: &str) -> i64 {
    let grid = Grid::new(input);
    grid_iterator(grid.width as i32, grid.height as i32)
        .flat_map(|position| {
            Vector2d::direction_vectors().map(move |direction| (position, direction))
        })
        .filter(|(position, direction)| grid.has_xmas(*position, *direction))
        .count() as i64
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);
    grid_iterator(grid.width as i32, grid.height as i32)
        .filter(|position| {
            Vector2d::diagonals()
                .filter(|direction| grid.has_cross_mas(*position, *direction))
                .count()
                >= 2
        })
        .count() as i64
}

static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 18);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 9);
}
