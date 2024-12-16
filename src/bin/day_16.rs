use std::collections::{HashMap, HashSet};

use aoc24::{Grid, Vector2d};
use petgraph::{graph::NodeIndex, Graph};

static MY_INPUT: &str = aoc24::load_input!();

static TEST_INPUT_PART_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

fn part_1(input: &str) -> i64 {
    let grid = Grid::new(input);

    let start = grid.iter().find(|(_, c)| *c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, c)| *c == b'E').unwrap().0;

    let mut pg = petgraph::Graph::new();
    let mut lookup = HashMap::new();

    for (node, c) in grid.iter() {
        if c == b'#' {
            continue;
        }
        lookup.insert(node, [0, 0, 0, 0].map(|_| pg.add_node(0)));
    }

    for (&node, a) in lookup.iter() {
        for (idx, neighbor) in node.neighbours().enumerate() {
            let Some(corresponding) = lookup.get(&neighbor) else {
                continue;
            };

            let b = corresponding;

            pg.add_edge(a[idx], b[idx], 1);
        }

        for idx in 0..4 {
            pg.add_edge(a[idx], a[(idx + 1) % 4], 1000);
            pg.add_edge(a[idx], a[(idx as isize - 1).rem_euclid(4) as usize], 1000);
        }
    }

    let solved = petgraph::algo::dijkstra(&pg, lookup[&start][0], None, |e| *e.weight() as i64);

    lookup[&end].iter().map(|x| solved[x]).min().unwrap()
}

fn part_2(input: &str) -> i64 {
    let grid = Grid::new(input);

    let start = grid.iter().find(|(_, c)| *c == b'S').unwrap().0;
    let end = grid.iter().find(|(_, c)| *c == b'E').unwrap().0;

    let mut pg = petgraph::Graph::new();
    let mut lookup = HashMap::new();

    for (node, c) in grid.iter() {
        if c == b'#' {
            continue;
        }
        lookup.insert(node, [0, 0, 0, 0].map(|_| pg.add_node(node)));
    }

    for (&node, a) in lookup.iter() {
        for (idx, neighbor) in node.neighbours().enumerate() {
            let Some(corresponding) = lookup.get(&neighbor) else {
                continue;
            };

            let b = corresponding;

            pg.add_edge(a[idx], b[idx], 1);
        }

        for idx in 0..4 {
            pg.add_edge(a[idx], a[(idx + 1) % 4], 1000);
            pg.add_edge(a[idx], a[(idx as isize - 1).rem_euclid(4) as usize], 1000);
        }
    }

    let solved = petgraph::algo::dijkstra(&pg, lookup[&start][0], None, |e| *e.weight() as i64);

    let end_weight = lookup[&end].iter().map(|x| solved[x]).min().unwrap();
    let ends = lookup[&end].iter().filter(|&x| solved[x] == end_weight);
    let mut explored = HashSet::new();

    fn explore(
        graph: &Graph<Vector2d, i32>,
        solved: &HashMap<NodeIndex<u32>, i64>,
        lookup: &HashMap<Vector2d, [NodeIndex; 4]>,
        explored: &mut HashSet<Vector2d>,
        idx: NodeIndex<u32>,
    ) {
        let pos = graph[idx];
        explored.insert(pos);

        let weight = solved[&idx];

        for n in pos.neighbours().chain([pos].into_iter()) {
            let Some(l) = lookup.get(&n) else {
                continue;
            };

            for n in l
                .iter()
                .copied()
                .filter(|&x| graph.neighbors(x).any(|b| b == idx))
            {
                let edge = graph.edges_connecting(n, idx).next().unwrap();
                let w = solved[&n];
                if weight == w + (*edge.weight() as i64) {
                    explore(graph, solved, lookup, explored, n);
                }
            }
        }
    }

    for &end in ends {
        explore(&pg, &solved, &lookup, &mut explored, end);
    }

    explored.len() as i64
}

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 2: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 7036);
}

#[test]
fn check_part_2() {
    assert_eq!(part_2(TEST_INPUT_PART_1), 45);
}
