use std::collections::{HashMap, HashSet};

use aoc24::Vector2d;
use petgraph::{algo::dijkstra, Graph};

fn part_1(input: &str) -> i64 {
    let corrupted: HashSet<_> = input
        .lines()
        .map(|x| {
            let (a, b) = x.split_once(',').unwrap();
            Vector2d(a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .take(1024)
        .collect();

    let mut graph = Graph::new_undirected();
    let mut lookup = HashMap::new();

    for coord in (0..=70).flat_map(|y| (0..=70).map(move |x| Vector2d(x, y))) {
        if corrupted.contains(&coord) {
            continue;
        }
        lookup.insert(coord, graph.add_node(coord));
    }

    for (&coord, &idx) in lookup.iter() {
        for n in coord.neighbours() {
            if let Some(&n) = lookup.get(&n) {
                graph.add_edge(idx, n, 1);
            }
        }
    }

    let r = dijkstra(
        &graph,
        lookup[&Vector2d(0, 0)],
        Some(lookup[&Vector2d(70, 70)]),
        |_| 1i64,
    );

    let end = lookup[&Vector2d(70, 70)];
    r[&end]
}

fn part_2(input: &str) -> Vector2d {
    let mut corrupted_coordinates = HashSet::new();

    for corrupted_coordinate in input.lines().map(|x| {
        let (a, b) = x.split_once(',').unwrap();
        Vector2d(a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
    }) {
        corrupted_coordinates.insert(corrupted_coordinate);
        let mut graph = Graph::new_undirected();
        let mut lookup = HashMap::new();

        for coord in (0..=70).flat_map(|y| (0..=70).map(move |x| Vector2d(x, y))) {
            if corrupted_coordinates.contains(&coord) {
                continue;
            }
            lookup.insert(coord, graph.add_node(coord));
        }

        for (&coord, &idx) in lookup.iter() {
            for n in coord.neighbours() {
                if let Some(&n) = lookup.get(&n) {
                    graph.add_edge(idx, n, 1);
                }
            }
        }

        let r = dijkstra(
            &graph,
            lookup[&Vector2d(0, 0)],
            Some(lookup[&Vector2d(70, 70)]),
            |_| 1i64,
        );

        let end = lookup[&Vector2d(70, 70)];
        if !r.contains_key(&end) {
            return corrupted_coordinate;
        }
    }

    panic!()
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    let p2 = part_2(MY_INPUT);
    println!("Part 2: {},{}", p2.0, p2.1);
}
