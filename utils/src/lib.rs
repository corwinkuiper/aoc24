use std::ops::{Add, AddAssign, Neg, Sub};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub text: Box<[u8]>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Vector2d(pub i32, pub i32);

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

impl Vector2d {
    pub const DIRECTIONS: [Vector2d; 4] = [
        Vector2d(1, 0),
        Vector2d(0, -1),
        Vector2d(-1, 0),
        Vector2d(0, 1),
    ];

    pub fn neighbours(self) -> impl Iterator<Item = Vector2d> {
        Self::DIRECTIONS.into_iter().map(move |x| x + self)
    }

    pub fn rotate(self) -> Vector2d {
        Vector2d(-self.1, self.0)
    }
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Self {
            width,
            height,
            text: input.as_bytes().into(),
        }
    }

    pub fn get(&self, v: Vector2d) -> Option<u8> {
        if !(0..self.width as i32).contains(&v.0) {
            return None;
        }
        if !(0..self.height as i32).contains(&v.1) {
            return None;
        }

        Some(self.text[(v.0 + v.1 * (self.width as i32 + 1)) as usize])
    }

    pub fn iter(&self) -> impl Iterator<Item = (Vector2d, u8)> + use<'_> {
        (0..self.height as i32)
            .flat_map(move |y| (0..self.width as i32).map(move |x| Vector2d(x, y)))
            .map(|v| (v, self.get(v).unwrap()))
    }
}
