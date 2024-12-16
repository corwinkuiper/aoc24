static TEST_INPUT_PART_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
static TEST_INPUT_PART_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

fn parse_mul(input: &str) -> Option<i64> {
    let input = input.strip_prefix("mul(")?;
    let (first_number, rest) = input.split_once(',')?;
    let (second_number, _rest) = rest.split_once(')')?;

    Some(first_number.parse::<i64>().ok()? * second_number.parse::<i64>().ok()?)
}

fn part_1(input: &str) -> i64 {
    input
        .char_indices()
        .map(|(idx, _)| &input[idx..])
        .flat_map(parse_mul)
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut enabled = true;
    let mut sum = 0;
    for substr in input.char_indices().map(|(idx, _)| &input[idx..]) {
        if substr.starts_with("do()") {
            enabled = true;
        }

        if substr.starts_with("don't()") {
            enabled = false;
        }

        if enabled {
            if let Some(num) = parse_mul(substr) {
                sum += num;
            }
        }
    }

    sum
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_parse() {
    assert!("4 ".parse::<i64>().is_err());
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 161);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 48);
}
