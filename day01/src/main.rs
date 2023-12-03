
use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let linestr = fs::read_to_string("input.txt").unwrap();
    let lines = linestr.lines();

    let re = Regex::new(r"\d").unwrap();

    let mut sum = 0;
    for line in lines {
        let matches: Vec<regex::Match> = re.find_iter(line).collect();
        let m1 = matches[0].as_str().parse::<i32>().unwrap();
        let m2 = matches[matches.len()-1].as_str().parse::<i32>().unwrap();
        sum += m1*10 + m2;

    }
    println!("{sum}")
}

fn part2() {
    let linestr = fs::read_to_string("input.txt").unwrap();
    let lines = linestr.lines();

    let numbers = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("0", 0),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);
    let re = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();

    let mut numbersr: HashMap<String, i32> = HashMap::new();
    for (s, v) in numbers.iter() {
        let r = s.chars().rev().collect::<String>();
        numbersr.insert(r, v.clone());
    }

    let rer = Regex::new(r"\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let mut sum = 0;
    for line in lines {

        // find first number
        let m = re.find(line).unwrap().as_str();
        let v1 = numbers.get(m).unwrap();

        // last number
        let liner_str = line.chars().rev().collect::<String>();
        let liner = liner_str.as_str();
        let m = rer.find(liner).unwrap().as_str();
        let v2 = numbersr.get(m).unwrap();

        sum += v1*10 + v2;
    }
    println!("{sum}")
}
