use std::collections::HashSet;

use aoc24::{Grid, Vector2d};

static TEST_INPUT_PART_1: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

static TEST_INPUT_PART_2: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

fn fill(grid: &Grid, letter: u8, position: Vector2d, pos: &mut HashSet<Vector2d>) {
    if grid.get(position) != Some(letter) {
        return;
    }
    if pos.contains(&position) {
        return;
    }

    pos.insert(position);

    for position in position.neighbours() {
        fill(grid, letter, position, pos);
    }
}

fn perimeter(area: &HashSet<Vector2d>) -> i64 {
    let mut perimeter = 0;
    for position in area.iter() {
        perimeter += position.neighbours().filter(|x| !area.contains(x)).count() as i64;
    }
    perimeter
}

fn explore(
    area: &HashSet<Vector2d>,
    mut position: Vector2d,
    direction: Vector2d,
    look: Vector2d,
    explored: &mut HashSet<(Vector2d, Vector2d)>,
) -> i64 {
    let mut count = 0;
    loop {
        explored.insert((position, look));
        if area.contains(&position) || !area.contains(&(position + look)) {
            break;
        }
        position += direction;
        count += 1;
    }

    count
}

fn sides(area: &HashSet<Vector2d>) -> i64 {
    let mut processed: HashSet<(Vector2d, Vector2d)> = HashSet::new();

    let mut side_lengths = 0;

    for &position in area.iter() {
        for direction in Vector2d::DIRECTIONS {
            let neighbour = position + direction;
            if area.contains(&neighbour) {
                continue;
            }
            let look = -direction;
            if processed.contains(&(neighbour, look)) {
                continue;
            }

            explore(area, neighbour, direction.rotate(), look, &mut processed);
            explore(area, neighbour, -(direction.rotate()), look, &mut processed);

            side_lengths += 1;
        }
    }

    side_lengths
}

fn part_1(input: &str) -> i64 {
    let grid = Grid::new(input);

    let mut sum = 0;

    let mut processed: HashSet<Vector2d> = HashSet::new();
    for (position, kind) in grid.iter() {
        if processed.contains(&position) {
            continue;
        }

        let mut area = HashSet::new();
        fill(&grid, kind, position, &mut area);

        processed.extend(area.iter().copied());

        sum += area.len() as i64 * perimeter(&area);
    }

    sum
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);

    let mut sum = 0;

    let mut processed: HashSet<Vector2d> = HashSet::new();
    for (position, kind) in grid.iter() {
        if processed.contains(&position) {
            continue;
        }

        let mut area = HashSet::new();
        fill(&grid, kind, position, &mut area);

        processed.extend(area.iter().copied());

        sum += area.len() as i64 * sides(&area);
    }

    sum
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 1930);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 236);
}
