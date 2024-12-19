use std::collections::{HashMap, HashSet};

// patterns, designs
fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(',').map(|x| x.trim()).collect();
    let designs = designs.lines().map(|x| x.trim()).collect();
    (patterns, designs)
}

fn is_possible<'a>(design: &'a str, patterns: &[&str], cache: &mut HashMap<&'a str, bool>) -> bool {
    if let Some(&cached) = cache.get(design) {
        return cached;
    }
    if design.is_empty() {
        return true;
    }
    let result = patterns
        .iter()
        .filter_map(|&x| design.strip_prefix(x))
        .any(|x| is_possible(x, patterns, cache));
    cache.insert(design, result);
    result
}

fn part_1(input: &str) -> i64 {
    let (patterns, designs) = parse(input);
    let mut cache = HashMap::new();
    designs
        .iter()
        .filter(|&&x| is_possible(x, &patterns, &mut cache))
        .count() as i64
}

fn count_possible<'a>(
    design: &'a str,
    patterns: &[&str],
    cache: &mut HashMap<&'a str, i64>,
) -> i64 {
    if let Some(&cached) = cache.get(design) {
        return cached;
    }
    if design.is_empty() {
        return 1;
    }
    let result = patterns
        .iter()
        .filter_map(|&x| design.strip_prefix(x))
        .map(|x| count_possible(x, patterns, cache))
        .sum();
    cache.insert(design, result);
    result
}

fn part_2(input: &str) -> i64 {
    let (patterns, designs) = parse(input);
    let mut cache = HashMap::new();
    designs
        .iter()
        .map(|&x| count_possible(x, &patterns, &mut cache))
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 1: {}", part_2(MY_INPUT));
}
