use std::iter;

use aoc24::Vector2d;

#[derive(Clone, Copy, Default)]
enum CodeButton {
    #[default]
    BA,
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    B9,
}

impl CodeButton {
    fn to_vec(self) -> Vector2d {
        match self {
            CodeButton::BA => (2, 3),
            CodeButton::B0 => (1, 3),
            CodeButton::B1 => (0, 2),
            CodeButton::B2 => (1, 2),
            CodeButton::B3 => (2, 2),
            CodeButton::B4 => (0, 1),
            CodeButton::B5 => (1, 1),
            CodeButton::B6 => (2, 1),
            CodeButton::B7 => (0, 0),
            CodeButton::B8 => (1, 0),
            CodeButton::B9 => (2, 0),
        }
        .into()
    }

    fn from_vec(vec: Vector2d) -> Option<Self> {
        [
            CodeButton::BA,
            CodeButton::B0,
            CodeButton::B1,
            CodeButton::B2,
            CodeButton::B3,
            CodeButton::B4,
            CodeButton::B5,
            CodeButton::B6,
            CodeButton::B7,
            CodeButton::B8,
            CodeButton::B9,
        ]
        .into_iter()
        .find(|x| x.to_vec() == vec)
    }
}

#[derive(Clone, Copy, Default)]
enum ControllerButton {
    Up,
    Down,
    Left,
    Right,
    #[default]
    A,
}

impl ControllerButton {
    fn to_vec(self) -> Vector2d {
        match self {
            ControllerButton::Up => (1, 0),
            ControllerButton::Down => (1, 1),
            ControllerButton::Left => (0, 1),
            ControllerButton::Right => (2, 1),
            ControllerButton::A => (2, 0),
        }
        .into()
    }

    fn from_vec(vec: Vector2d) -> Option<Self> {
        [
            ControllerButton::Up,
            ControllerButton::Down,
            ControllerButton::Left,
            ControllerButton::Right,
            ControllerButton::A,
        ]
        .into_iter()
        .find(|x| x.to_vec() == vec)
    }
}

trait Button: Copy {
    fn to_vec(self) -> Vector2d;
    fn from_vec(vec: Vector2d) -> Option<Self>
    where
        Self: Sized;
}

impl Button for ControllerButton {
    fn to_vec(self) -> Vector2d {
        self.to_vec()
    }

    fn from_vec(vec: Vector2d) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_vec(vec)
    }
}

impl Button for CodeButton {
    fn to_vec(self) -> Vector2d {
        self.to_vec()
    }

    fn from_vec(vec: Vector2d) -> Option<Self>
    where
        Self: Sized,
    {
        Self::from_vec(vec)
    }
}

#[derive(Default, Clone, Copy)]
struct Pusher<B: Button> {
    aiming: B,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_vec(vec: Vector2d) -> Self {
        match (vec.0, vec.1) {
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            _ => panic!("bad direction"),
        }
    }

    fn to_button(self) -> ControllerButton {
        match self {
            Direction::Up => ControllerButton::Up,
            Direction::Down => ControllerButton::Down,
            Direction::Left => ControllerButton::Left,
            Direction::Right => ControllerButton::Right,
        }
    }
}

fn path<B: Button>(start: B, end: B) -> Vec<Vec<Vector2d>> {
    let s = start.to_vec();
    let e = end.to_vec();

    let diff = e - s;

    let diff_x = diff.0.signum();
    let diff_y = diff.1.signum();
    let mut c = s;

    let a = Vec::from_iter(iter::from_fn(|| {
        if c.0 != e.0 {
            c.0 += diff_x;
            Some(c)
        } else if c.1 != e.1 {
            c.1 += diff_y;
            Some(c)
        } else {
            None
        }
    }));

    let mut c = s;
    let b = Vec::from_iter(iter::from_fn(|| {
        if c.1 != e.1 {
            c.1 += diff_y;
            Some(c)
        } else if c.0 != e.0 {
            c.0 += diff_x;
            Some(c)
        } else {
            None
        }
    }));

    let mut v = vec![a, b];
    v.retain(|x| !x.iter().any(|&x| B::from_vec(x).is_none()));

    v
}

impl<B: Button> Pusher<B> {
    fn push(&mut self, button: B) -> Vec<Vec<Vector2d>> {
        let paths = path(self.aiming, button);
        self.aiming = button;
        paths
    }
}

#[derive(Default)]
struct Stack {
    code: Pusher<CodeButton>,
    controllers: Vec<Pusher<ControllerButton>>,
    current_sequence: Vec<ControllerButton>,
}

impl Stack {
    fn push_code_button(&mut self, button: CodeButton) {
        let mut minimums = self.code.push(button);
        for controller in self.controllers.iter_mut() {
            let mut c = *controller;
            minimums.into_iter().map(|x| {
                Vec::from_iter(
                    x.into_iter()
                        .map(|x| c.push(Direction::from_vec(x).to_button())),
                )
            })
        }
    }
}
