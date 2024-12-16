static TEST_INPUT_PART_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

static TEST_INPUT_PART_2: &str = TEST_INPUT_PART_1;

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|x| x.split_once(": ").unwrap())
        .map(|x| {
            (
                x.0.parse::<i64>().unwrap(),
                x.1.split_ascii_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn part_1(input: &str) -> i64 {
    let input = parse(input);

    fn can_make_number(number: i64, list: &[i64], total: i64) -> bool {
        if let Some(this) = list.first().copied() {
            [total + this, total * this]
                .into_iter()
                .any(|x| can_make_number(number, &list[1..], x))
        } else {
            total == number
        }
    }

    input
        .iter()
        .filter(|(x, y)| can_make_number(*x, &y[1..], y[0]))
        .map(|(x, _)| *x)
        .sum()
}

fn concat(a: i64, b: i64) -> i64 {
    let mut n = 0;
    let mut c = b;
    while c != 0 {
        n += 1;
        c /= 10;
    }

    a * 10_i64.pow(n) + b
}

#[test]
fn check_concat() {
    assert_eq!(concat(123, 456), 123456);
}

fn part_2(input: &str) -> i64 {
    let input = parse(input);

    fn can_make_number(number: i64, list: &[i64], total: i64) -> bool {
        if let Some(this) = list.first().copied() {
            [total + this, total * this, concat(total, this)]
                .into_iter()
                .any(|x| can_make_number(number, &list[1..], x))
        } else {
            total == number
        }
    }

    input
        .iter()
        .filter(|(x, y)| can_make_number(*x, &y[1..], y[0]))
        .map(|(x, _)| *x)
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 3749);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 11387);
}
