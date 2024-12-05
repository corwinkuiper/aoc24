use std::{cmp::Ordering, collections::HashMap};

static TEST_INPUT_PART_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

static TEST_INPUT_PART_2: &str = TEST_INPUT_PART_1;

fn parse_rules_and_runs(input: &str) -> Option<(HashMap<i64, Vec<i64>>, Vec<Vec<i64>>)> {
    let (rules, runs) = input.split_once("\n\n")?;

    let mut rules_map: HashMap<i64, Vec<i64>> = HashMap::new();

    for (key, value) in rules.lines().map(|x| {
        let (x, y) = x.split_once('|').unwrap();
        (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
    }) {
        rules_map.entry(key).or_default().push(value);
    }
    let runs = runs
        .lines()
        .map(|x| x.split(',').map(|x| x.parse::<i64>().unwrap()).collect())
        .collect();
    Some((rules_map, runs))
}

fn is_run_valid(run: &[i64], rules: &HashMap<i64, Vec<i64>>) -> bool {
    for (idx, num) in run.iter().copied().enumerate() {
        let Some(rule) = rules.get(&num) else {
            continue;
        };
        for first in run.iter().copied().take(idx) {
            if rule.contains(&first) {
                return false;
            }
        }
    }

    true
}

fn part_1(input: &str) -> i64 {
    let (rules, runs) = parse_rules_and_runs(input).unwrap();

    runs.iter()
        .filter(|run| is_run_valid(run, &rules))
        .map(|x| x[x.len() / 2])
        .sum()
}

fn order_run(run: &mut [i64], rules: &HashMap<i64, Vec<i64>>) {
    run.sort_by(|a, b| {
        let a_before_b = rules.get(a).map(|x| x.contains(b)).unwrap_or_default();
        let b_before_a = rules.get(b).map(|x| x.contains(a)).unwrap_or_default();
        if a_before_b {
            return Ordering::Less;
        }
        if b_before_a {
            return Ordering::Greater;
        }

        Ordering::Equal
    });
}

fn part_2(input: &str) -> i64 {
    let (rules, runs) = parse_rules_and_runs(input).unwrap();

    let invalid_runs = runs.iter().filter(|run| !is_run_valid(run, &rules));

    invalid_runs
        .map(|x| {
            let mut run = x.clone();
            order_run(&mut run, &rules);
            run
        })
        .map(|x| x[x.len() / 2])
        .sum()
}

static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 143);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 123);
}
