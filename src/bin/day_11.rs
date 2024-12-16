use std::collections::HashMap;

static TEST_INPUT_PART_1: &str = "125 17";

fn count_digits(mut n: i64) -> i64 {
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}

fn split_digits(mut n: i64) -> Option<(i64, i64)> {
    let number_of_digits = count_digits(n);
    if number_of_digits % 2 != 0 {
        return None;
    }

    let mut t = 0;
    for x in 0..number_of_digits / 2 {
        t += (n % 10) * 10_i64.pow(x as u32);
        n /= 10;
    }

    Some((n, t))
}

fn transform_cached(x: i64, remaining_steps: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(left) = cache.get(&(x, remaining_steps)) {
        return *left;
    }

    if remaining_steps == 0 {
        return 1;
    }

    let result = if x == 0 {
        transform_cached(1, remaining_steps - 1, cache)
    } else if let Some((l, r)) = split_digits(x) {
        transform_cached(l, remaining_steps - 1, cache)
            + transform_cached(r, remaining_steps - 1, cache)
    } else {
        transform_cached(x * 2024, remaining_steps - 1, cache)
    };

    cache.insert((x, remaining_steps), result);

    result
}

fn solve(input: &str, steps: i64) -> i64 {
    let mut cache = HashMap::new();

    input
        .split_ascii_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .map(|x| transform_cached(x, steps, &mut cache))
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", solve(MY_INPUT, 25));
    println!("Part 1: {}", solve(MY_INPUT, 75));
}

#[test]
fn check_part_1() {
    assert_eq!(solve(TEST_INPUT_PART_1, 25), 55312);
}
