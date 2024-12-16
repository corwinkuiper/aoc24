use std::collections::HashMap;

static TEST_INPUT_PART_1: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";

static TEST_INPUT_PART_2: &str = TEST_INPUT_PART_1;

fn part_1(input: &str) -> i64 {
    let lists = input
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse::<i64>().unwrap()))
        .map(|mut x| [x.next().unwrap(), x.next().unwrap()]);

    let mut a: Vec<_> = lists.clone().map(|x| x[0]).collect();
    let mut b: Vec<_> = lists.map(|x| x[1]).collect();
    a.sort();
    b.sort();

    a.iter()
        .copied()
        .zip(b.iter().copied())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

fn part_2(input: &str) -> i64 {
    let mut appearances: HashMap<i64, i64> = HashMap::new();
    let lists = input
        .lines()
        .map(|x| x.split_whitespace().map(|x| x.parse::<i64>().unwrap()))
        .map(|mut x| [x.next().unwrap(), x.next().unwrap()]);

    let a: Vec<_> = lists.clone().map(|x| x[0]).collect();
    let b: Vec<_> = lists.map(|x| x[1]).collect();

    for &n in b.iter() {
        *appearances.entry(n).or_default() += 1;
    }

    a.iter()
        .copied()
        .map(|x| appearances.get(&x).copied().unwrap_or_default() * x)
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 11);
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 31);
}
