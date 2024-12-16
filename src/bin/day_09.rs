use std::collections::VecDeque;

use itertools::Itertools;

static TEST_INPUT_PART_1: &str = "2333133121414131402";
static TEST_INPUT_PART_2: &str = TEST_INPUT_PART_1;

const SENTINEL: i64 = i64::MAX;

fn expand(input: &str) -> VecDeque<i64> {
    let input = input.trim();
    let mut expanded = Vec::new();
    let mut a = true;
    let mut v = 0;

    for &x in input.as_bytes() {
        let number = x - b'0';
        if a {
            expanded.extend((0..number).map(|_| v));
            v += 1;
        } else {
            expanded.extend((0..number).map(|_| SENTINEL));
        }
        a = !a;
    }

    expanded.into()
}

fn back(a: &mut VecDeque<i64>, len: usize) -> i64 {
    if a.len() - 1 == len {
        return SENTINEL;
    }
    let mut b = a.pop_back().unwrap();
    while b == SENTINEL && a.len() - 1 != len {
        b = a.pop_back().unwrap();
    }
    b
}

fn contract(input: &mut VecDeque<i64>) {
    let mut idx = 0;
    while idx < input.len() {
        if input[idx] == SENTINEL {
            input[idx] = back(input, idx);
        }

        idx += 1;
    }
}

fn part_1(input: &str) -> i64 {
    let mut v = expand(input);
    contract(&mut v);

    v.iter()
        .copied()
        .enumerate()
        .filter(|(_, x)| *x != SENTINEL)
        .map(|(idx, x)| idx as i64 * x)
        .sum()
}

#[derive(Debug, Clone)]
struct File {
    id: i64,
    size: usize,
    idx: usize,
}

struct Gap {
    idx: usize,
    size: usize,
}

struct Disk {
    files: Vec<File>,
    gaps: Vec<Gap>,
}

impl Disk {
    fn new(input: &str) -> Self {
        let input = input.trim();
        let mut files = Vec::new();
        let mut gaps = Vec::new();
        let mut a = true;
        let mut v = 0;

        let mut idx = 0;

        gaps.push(Gap { idx: 0, size: 0 });

        for &x in input.as_bytes() {
            let number = (x - b'0') as usize;
            if a {
                files.push(File {
                    id: v,
                    size: number,
                    idx,
                });
                v += 1;
            } else {
                gaps.push(Gap { idx, size: number })
            }
            idx += number;
            a = !a;
        }

        gaps.push(Gap { idx, size: 0 });

        Self { files, gaps }
    }
}

fn part_2(input: &str) -> i64 {
    let mut disk = Disk::new(input);

    let mut unprocessed = disk.files.clone();
    let mut destination = unprocessed.clone();

    while let Some(mut end) = unprocessed.pop() {
        if let Some((slot, _)) = destination
            .iter()
            .tuple_windows()
            .find(|(a, b)| b.idx - (a.idx + a.size) >= end.size)
        {
            if slot.idx >= end.idx {
                continue;
            }
            end.idx = slot.idx + slot.size;
            destination.retain(|x| x.id != end.id);
            destination.push(end);
            destination.sort_by_key(|x| x.idx);
        }
    }

    destination
        .iter()
        .map(|x| (x.idx..(x.idx + x.size)).sum::<usize>() as i64 * x.id)
        .sum()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 1928);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_2), 2858);
}
