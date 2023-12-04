use regex::Regex;
use std::{collections::HashMap, fs};

fn read_and_parse() -> Vec<(Vec<i32>, Vec<i32>)> {
    let read_to_string = &fs::read_to_string("input.txt").unwrap();
    let lines = read_to_string.lines();

    let re = Regex::new(r"\:|\|").unwrap();
    let pile = lines
        .map(|line| {
            let matches = re.split(line).collect::<Vec<&str>>();

            let ws = matches[1];
            let hs = matches[2];

            let wls = ws.split_ascii_whitespace();
            let hls = hs.split_ascii_whitespace();

            let wl = wls.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            let hl = hls.map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

            (wl, hl)
        })
        .collect::<Vec<_>>();
    pile
}

fn part1() {
    let pile = read_and_parse();

    let mut score_sum = 0;
    for (wins, haves) in &pile {
        let mut score = 0;
        let mut card_point = 1;
        for have in haves {
            if wins.iter().find(|x| **x == *have) != None {
                score = card_point;
                card_point *= 2;
            }
        }
        score_sum += score;
    }
    println!("{score_sum}");
}

fn part2() {
    let pile = read_and_parse();

    let mut cards: HashMap<i32, i32> = HashMap::new();

    let mut card_id = 1;
    for (hl, wl) in &pile {
        let mut points = 0;

        let v = cards.entry(card_id).or_insert(0);
        *v += 1;

        for have in hl {
            if wl.iter().find(|x| **x == *have) != None {
                points += 1;
            }
        }

        let n = *(cards.get(&card_id).unwrap());
        for ci in 0..points {
            let v = card_id + ci + 1;
            let e = cards.entry(v).or_insert(0);
            *e += n;
        }

        card_id += 1;
    }

    let mut sum = 0;
    for (_, n) in cards {
        sum += n;
    }

    println!("{sum}");
}

fn main() {
    part1();
    part2();

}
