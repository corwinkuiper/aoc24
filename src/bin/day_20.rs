use std::collections::HashMap;

use aoc24::{Grid, Vector2d};
use petgraph::{algo::dijkstra, Graph};

fn part_1(input: &str, cheat_time: i64) -> i64 {
    let grid = Grid::new(input);

    let mut lookup = HashMap::new();
    let mut graph = Graph::new();
    let start = grid.iter().find(|(_, x)| *x == b'S').unwrap().0;

    for (coord, l) in grid.iter() {
        if l == b'#' {
            continue;
        }
        lookup.insert(coord, graph.add_node(coord));
    }

    for (&coord, &idx) in lookup.iter() {
        for n in coord.neighbours() {
            if let Some(&n) = lookup.get(&n) {
                graph.add_edge(idx, n, ());
            }
        }
    }

    let ct = cheat_time as i32;
    let cheat_pattern: Vec<_> = (-ct - 1..=ct + 1)
        .flat_map(|y| (-ct - 1..=ct + 1).map(move |x| (x, y)))
        .map(|(x, y)| Vector2d(x, y))
        .filter(|&x| x.manhattan() <= ct)
        .collect();

    let result = dijkstra(&graph, lookup[&start], None, |_| 1i64);
    let result = &result;
    lookup
        .iter()
        .flat_map(|(&coord, &idx)| {
            let this_cost = result[&idx];

            cheat_pattern
                .iter()
                .copied()
                .map(move |cheat| (coord + cheat, cheat.manhattan()))
                .filter_map(|(x, time)| lookup.get(&x).map(|x| (x, time)))
                .map(move |(x, time)| result[x] - this_cost - time as i64)
        })
        .filter(|&x| x > 0)
        .filter(|&x| x >= 100)
        .count() as i64
}

static MY_INPUT: &str = aoc24::load_input!();

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT, 2));
    println!("Part 2: {}", part_1(MY_INPUT, 20));
}
