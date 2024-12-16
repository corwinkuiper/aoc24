use core::str;
use std::{
    collections::HashSet,
    iter,
    sync::{LazyLock, OnceLock},
};

use regex::Regex;
use utils::Vector2d;

static TEST_SIZE: Vector2d = Vector2d(11, 7);
static TEST_INPUT_PART_1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

struct Robot {
    position: Vector2d,
    velocity: Vector2d,
}

fn parse(input: &str) -> impl Iterator<Item = Robot> + use<'_> {
    static RE: LazyLock<Regex, fn() -> Regex> =
        LazyLock::new(|| Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap());

    RE.captures_iter(input).map(|x| {
        let get = |idx: usize| x.get(idx).unwrap().as_str().parse::<i32>().unwrap();
        Robot {
            position: Vector2d(get(1), get(2)),
            velocity: Vector2d(get(3), get(4)),
        }
    })
}

fn part_1(input: &str, bounds: Vector2d) -> i64 {
    let mut quadrant_counts = [0; 4];

    for mut robot in parse(input) {
        robot.position += Vector2d(robot.velocity.0 * 100, robot.velocity.1 * 100);
        robot.position.0 = robot.position.0.rem_euclid(bounds.0);
        robot.position.1 = robot.position.1.rem_euclid(bounds.1);

        if robot.position.0 == bounds.0 / 2 || robot.position.1 == bounds.1 / 2 {
            continue;
        }

        let quadrant_idx =
            robot.position.0 / (bounds.0 / 2 + 1) + 2 * (robot.position.1 / (bounds.1 / 2 + 1));
        quadrant_counts[usize::try_from(quadrant_idx).unwrap()] += 1;
    }

    quadrant_counts.into_iter().product()
}

fn find_connected(robots: &HashSet<Vector2d>, check: Vector2d, processed: &mut HashSet<Vector2d>) {
    if processed.contains(&check) || !robots.contains(&check) {
        return;
    }
    processed.insert(check);

    [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ]
    .into_iter()
    .for_each(|(x, y)| {
        find_connected(robots, check + Vector2d(x, y), processed);
    });
}

fn part_2(input: &str, bounds: Vector2d) -> i64 {
    let mut robots: Vec<_> = parse(input).collect();

    for iteration in 1.. {
        let mut frame = vec![b'.'; ((bounds.0 + 1) * bounds.1) as usize];
        for y in 0..(bounds.1 - 1) {
            let idx = (bounds.0) + y * (bounds.0 + 1);
            frame[idx as usize] = b'\n';
        }

        for robot in robots.iter_mut() {
            robot.position += Vector2d(robot.velocity.0, robot.velocity.1);
            robot.position.0 = robot.position.0.rem_euclid(bounds.0);
            robot.position.1 = robot.position.1.rem_euclid(bounds.1);

            let idx = robot.position.0 + robot.position.1 * (bounds.0 + 1);
            frame[idx as usize] = b'#';
        }

        let x = robots.iter().map(|x| x.position.0 as f64);
        let average = x.clone().sum::<f64>() / robots.len() as f64;
        let variance = x.map(|x| (x - average).powi(2)).sum::<f64>() / robots.len() as f64;
        let y = robots.iter().map(|x| x.position.1 as f64);
        let average = y.clone().sum::<f64>() / robots.len() as f64;
        let variance_y = y.map(|x| (x - average).powi(2)).sum::<f64>() / robots.len() as f64;

        if variance < 700. && variance_y < 700. {
            println!("{}", iteration);
            println!("{}", variance);
            println!("{}", str::from_utf8(&frame).unwrap());
            return iteration;
        }
    }

    unreachable!()
}

static MY_INPUT_BOUNDS: Vector2d = Vector2d(101, 103);
static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT, MY_INPUT_BOUNDS));
    println!("Part 2: {}", part_2(MY_INPUT, MY_INPUT_BOUNDS));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1, TEST_SIZE), 12);
}
