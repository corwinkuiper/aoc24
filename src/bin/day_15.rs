use std::collections::HashSet;

use aoc24::{Grid, Vector2d};

fn parse(input: &str) -> (Grid, Vec<Vector2d>) {
    let (grid, directions) = input.split_once("\n\n").unwrap();

    (
        Grid::new(grid),
        directions
            .lines()
            .flat_map(|x| {
                x.chars().map(|x| match x {
                    '^' => Vector2d(0, -1),
                    '>' => Vector2d(1, 0),
                    '<' => Vector2d(-1, 0),
                    'v' => Vector2d(0, 1),
                    other => panic!("BAD DIRECTION {}", other),
                })
            })
            .collect(),
    )
}

fn push(things: &mut [Vector2d], to_push: usize, direction: Vector2d, grid: &Grid) -> bool {
    let pos = things[to_push];
    let target = pos + direction;
    let can_push = if let Some(target) = things.iter().position(|&x| x == target) {
        push(things, target, direction, grid)
    } else {
        grid.get(target) != Some(b'#')
    };

    if can_push {
        things[to_push] = target;
    }

    can_push
}

fn part_1(input: &str) -> i64 {
    let (grid, directions) = parse(input);

    let robot = grid.iter().find(|(_, c)| *c == b'@').unwrap().0;

    let mut things = vec![robot];
    things.extend(grid.iter().filter(|(_, c)| *c == b'O').map(|(x, _)| x));

    for direction in directions {
        push(&mut things, 0, direction, &grid);
    }

    things[1..]
        .iter()
        .copied()
        .map(|x| x.0 as i64 + x.1 as i64 * 100)
        .sum()
}

fn is_hitting_box(box_pos: Vector2d, pos: Vector2d) -> bool {
    [(0, 0), (1, 0)]
        .into_iter()
        .map(|(x, y)| Vector2d(x, y) + box_pos)
        .any(|x| x == pos)
}

fn box_push_casts(box_pos: Vector2d, direction: Vector2d) -> [Vector2d; 2] {
    match (direction.0, direction.1) {
        (1, 0) => [(2, 0), (2, 0)],
        (-1, 0) => [(-1, 0), (-1, 0)],
        (0, -1) => [(0, -1), (1, -1)],
        (0, 1) => [(0, 1), (1, 1)],
        _ => panic!("Bad direction"),
    }
    .map(|(x, y)| box_pos + Vector2d(x, y))
}

fn push_big_boxes(
    things: &mut [Vector2d],
    to_push: usize,
    direction: Vector2d,
    has_wall: &impl Fn(Vector2d) -> bool,
    explore: bool,
) -> bool {
    let pos = things[to_push];
    let target = pos + direction;

    let can_push = if to_push != 0 {
        let casts = box_push_casts(pos, direction);
        if casts.into_iter().any(has_wall) {
            return false;
        }

        let things_to_push: Vec<_> = casts
            .into_iter()
            .flat_map(|x| {
                things
                    .iter()
                    .position(|&y| y != pos && is_hitting_box(y, x))
            })
            .collect();

        let can_push = things_to_push
            .iter()
            .all(|&idx| push_big_boxes(things, idx, direction, has_wall, true));

        if can_push && !explore {
            loop {
                let Some(thing) = casts
                    .into_iter()
                    .flat_map(|x| {
                        things
                            .iter()
                            .position(|&y| y != pos && is_hitting_box(y, x))
                    })
                    .next()
                else {
                    break;
                };
                dbg!(direction, to_push, things[to_push], thing, things[thing]);
                assert!(push_big_boxes(things, thing, direction, has_wall, explore));
            }
            things[to_push] = target;
        }

        can_push
    } else {
        if has_wall(target) {
            return false;
        }
        let can_push = if let Some(target) = things
            .iter()
            .position(|&x| x != pos && is_hitting_box(x, target))
        {
            push_big_boxes(things, target, direction, has_wall, explore)
        } else {
            true
        };

        if can_push && !explore {
            things[to_push] = target;
        }

        can_push
    };

    can_push
}

fn part_2(input: &str) -> i64 {
    let (grid, directions) = parse(input);

    let robot = grid.iter().find(|(_, c)| *c == b'@').unwrap().0;
    let robot = Vector2d(robot.0 * 2, robot.1);

    let mut things = vec![robot];
    things.extend(
        grid.iter()
            .filter(|(_, c)| *c == b'O')
            .map(|(x, _)| Vector2d(x.0 * 2, x.1)),
    );

    for direction in directions {
        push_big_boxes(
            &mut things,
            0,
            direction,
            &|mut pos| {
                pos.0 /= 2;
                grid.get(pos) == Some(b'#')
            },
            false,
        );
    }

    things[1..]
        .iter()
        .copied()
        .map(|x| x.0 as i64 + x.1 as i64 * 100)
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}
