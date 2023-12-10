use gcd;
use regex::Regex;
use std::{collections::HashMap, fs};

fn parse_input(contents: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let mut lines = contents.lines();

    let instrs = lines.next().unwrap();
    lines.next();
    let re = Regex::new(r"(\w\w\w) \= \((\w\w\w), (\w\w\w)\)").unwrap();
    let mut map = HashMap::new();
    for node_str in lines {
        let caps = re.captures(node_str).unwrap();
        let from = caps.get(1).unwrap().as_str();
        let left = caps.get(2).unwrap().as_str();
        let right = caps.get(3).unwrap().as_str();
        map.insert(from, (left, right));
    }
    (instrs, map)
}

fn part1() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let (instrs, map) = parse_input(&contents);

    let mut cur = "AAA";
    let mut steps = 0;
    loop {
        for instr in instrs.chars() {
            if instr == 'L' {
                cur = map[&cur].0;
            } else {
                cur = map[&cur].1;
            }
            steps += 1;
            if cur == "ZZZ" {
                break;
            }
        }
        if cur == "ZZZ" {
            break;
        }
    }
    println!("Part1: {steps}")
}

fn part2() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let (instrs, map) = parse_input(&contents);

    let mut current_nodes = Vec::new();
    for (src, _) in &map {
        if src.ends_with("Z") {
            current_nodes.push(*src);
        }
    }

    let mut cycle_lens = Vec::new();
    for c in current_nodes {
        // println!("Now finding cyckles for node: {c}");
        let mut cur = c;
        let mut steps = 0u64;
        let mut found = false;
        let mut cycle_found = false;
        while !cycle_found {
            for instr in instrs.chars() {
                if instr == 'L' {
                    cur = map[&cur].0;
                } else {
                    cur = map[&cur].1;
                }
                steps += 1;

                if cur.ends_with("Z") {
                    // println!(" Found end after {steps} steps");
                    if found {
                        cycle_found = true;
                        break;
                    }
                    found = true;
                    steps = 0;
                }
            }
        }
        cycle_lens.push(steps);
    }

    let gcd = cycle_lens
        .clone()
        .into_iter()
        .reduce(|acc, x| gcd::binary_u64(acc, x))
        .unwrap();
    let cycle_unique = cycle_lens.iter().map(|x| (*x) / gcd).collect::<Vec<_>>();
    let p1: u64 = cycle_unique.iter().product();
    let p2 = p1 * gcd;
    println!("Part2: {p2}");
}

fn main() {
    part1();
    part2();
}
