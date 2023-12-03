use regex::Regex;
use std::fs;
use std::cmp::max;

fn solve_part1() {
    let linestr = fs::read_to_string("input.txt").unwrap();
    let lines = linestr.lines();

    let mut sum = 0;
    for line in lines {
        // games
        let re1 = Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re1.captures(line).unwrap();
        let game_id: i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let cube_sets_str = caps.get(2).unwrap().as_str();
        let mut possible = true;
        let disclosures: Vec<&str> = Regex::new(r"; ").unwrap().split(cube_sets_str).collect();
        for disclosure in disclosures {
            let cube_strs: Vec<&str> = Regex::new(r", ").unwrap().split(disclosure).collect();
            for cube in cube_strs {
                let e: Vec<&str> = cube.split(' ').collect();
                let n = e[0].parse::<i32>().unwrap();
                let color = e[1];
                if (color == "red" && n > 12)
                    || (color == "green" && n > 13)
                    || (color == "blue" && n > 14)
                {
                    possible = false;
                    break;
                }
            }
            if !possible {
                break;
            }
        }
        if possible {
            sum += game_id;
        }
        //let re = Regex::new(r"Game (\d+): (?:((?:(\d+) (red|green|blue)),?);?)*");
    }
    println!("part01: {sum}");
}

fn solve_part2() {
    let linestr = fs::read_to_string("input.txt").unwrap();
    let lines = linestr.lines();

    let mut sum = 0;
    for line in lines {
        // games
        let re1 = Regex::new(r"Game (\d+): (.*)").unwrap();
        let caps = re1.captures(line).unwrap();
        let cube_sets_str = caps.get(2).unwrap().as_str();
        let disclosures: Vec<&str> = Regex::new(r"; ").unwrap().split(cube_sets_str).collect();
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for disclosure in disclosures {
            let cube_strs: Vec<&str> = Regex::new(r", ").unwrap().split(disclosure).collect();
            for cube in cube_strs {
                let e: Vec<&str> = cube.split(' ').collect();
                let n = e[0].parse::<i32>().unwrap();
                let color = e[1];
                if color == "red" {
                    max_red = max(n, max_red);
                }
                if color == "green" {
                    max_green = max(n, max_green);
                }
                if color == "blue" {
                    max_blue = max(n, max_blue);
                }
            }
        }
        sum += max_red * max_green * max_blue;
    }
    println!("part02: {sum}");
}

fn main() {
    solve_part1();
    solve_part2();
}
