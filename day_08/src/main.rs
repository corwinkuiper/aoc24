use std::{
    collections::{HashMap, HashSet},
    ops::{Add, AddAssign, Neg, Sub},
};

static TEST_INPUT_PART_1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
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
    let grid = Grid::new(input);
    let mut nodes: HashMap<u8, Vec<Vector2d>> = HashMap::new();
    for (position, n) in grid.iter() {
        if n != b'.' {
            nodes.entry(n).or_default().push(position);
        }
    }

    let mut anti_nodes = HashSet::new();
    let mut add_anti_node = |anti_node: Vector2d| {
        if anti_node.0 < 0
            || anti_node.1 < 0
            || anti_node.0 >= grid.width as i32
            || anti_node.1 >= grid.height as i32
        {
            return;
        }

        anti_nodes.insert(anti_node);
    };

    for node in nodes.values() {
        for (&a, &b) in node
            .iter()
            .enumerate()
            .flat_map(|(idx, v)| node.iter().skip(idx + 1).map(move |b| (v, b)))
        {
            let difference = b - a;
            add_anti_node(b + difference);
            add_anti_node(a - difference);
        }
    }

    anti_nodes.len() as i64
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);
    let mut nodes: HashMap<u8, Vec<Vector2d>> = HashMap::new();
    for (position, n) in grid.iter() {
        if n != b'.' {
            nodes.entry(n).or_default().push(position);
        }
    }

    let mut anti_nodes = HashSet::new();
    let mut add_anti_node = |anti_node: Vector2d| -> bool {
        if anti_node.0 < 0
            || anti_node.1 < 0
            || anti_node.0 >= grid.width as i32
            || anti_node.1 >= grid.height as i32
        {
            return false;
        }

        anti_nodes.insert(anti_node);
        true
    };

    for node in nodes.values() {
        for (&a, &b) in node
            .iter()
            .enumerate()
            .flat_map(|(idx, v)| node.iter().skip(idx + 1).map(move |b| (v, b)))
        {
            let difference = b - a;
            let mut s = a;
            while add_anti_node(s) {
                s += -difference;
            }
            let mut s = b;
            while add_anti_node(s) {
                s += difference;
            }
        }
    }

    anti_nodes.len() as i64
}

static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 14);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 34);
}
