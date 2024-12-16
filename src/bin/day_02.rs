use itertools::Itertools;

static TEST_INPUT_PART_1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn is_safe(x: impl Iterator<Item = i64> + Clone) -> bool {
    let differences = x.tuple_windows().map(|(a, b)| a - b);
    if differences.clone().any(|x| x.abs() > 3) {
        return false;
    }

    let sign_of_first = differences.clone().next().unwrap().signum();

    if differences.clone().any(|x| x.signum() != sign_of_first) {
        return false;
    }

    true
}

fn is_safe_with_skip(input: &[i64]) -> bool {
    if is_safe(input.iter().cloned()) {
        return true;
    }
    for i in 0..input.len() {
        if is_safe(
            input
                .iter()
                .cloned()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, x)| x),
        ) {
            return true;
        }
    }

    false
}

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| is_safe(x.iter().cloned()))
        .count() as i64
}

fn part_2(input: &str) -> i64 {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|x| is_safe_with_skip(x))
        .count() as i64
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 2);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_1), 4);
}
