use regex::Regex;
use std::sync::OnceLock;

static TEST_INPUT_PART_1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

struct Problem {
    a: [i64; 2],
    b: [i64; 2],
    t: [i64; 2],
}

impl Problem {
    fn solve(&self) -> Option<(i64, i64)> {
        let a = self.a;
        let b = self.b;
        let t = self.t;

        let top = t[0] * a[1] - t[1] * a[0];
        let bottom = b[0] * a[1] - a[0] * b[1];

        if top % bottom != 0 {
            return None;
        }

        let rb = top / bottom;
        let ra_partial = t[1] - rb * b[1];

        if ra_partial % a[1] != 0 {
            return None;
        }

        let ra = ra_partial / a[1];

        Some((ra, rb))
    }
}

static REGEX: &str = r#"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)"#;

fn parse(input: &str) -> impl Iterator<Item = Problem> + use<'_> {
    static RE: OnceLock<Regex> = OnceLock::new();
    let re = RE.get_or_init(|| Regex::new(REGEX).unwrap());

    re.captures_iter(input).map(|x| {
        let get = |idx: usize| x.get(idx).unwrap().as_str().parse::<i64>().unwrap();
        Problem {
            a: [get(1), get(2)],
            b: [get(3), get(4)],
            t: [get(5), get(6)],
        }
    })
}

fn part_1(input: &str) -> i64 {
    parse(input)
        .flat_map(|p| p.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

fn part_2(input: &str) -> i64 {
    parse(input)
        .map(|x| Problem {
            t: x.t.map(|x| x + 10000000000000),
            ..x
        })
        .flat_map(|p| p.solve())
        .map(|(a, b)| a * 3 + b)
        .sum()
}

static MY_INPUT: &str = include_str!("input.txt");

fn main() {
    println!("Part 1: {}", part_1(MY_INPUT));
    println!("Part 1: {}", part_2(MY_INPUT));
}

#[test]
fn check_part_1() {
    assert_eq!(part_1(TEST_INPUT_PART_1), 480);
}
