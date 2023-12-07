use std::{collections::HashMap, fs};

fn card_value(card: char, joker: bool) -> i64 {
    if joker && card == 'J' {
        return 1;
    }
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!(),
    }
}

fn hand_value_multi(vals: &Vec<i64>, joker_count: Option<i64>) -> i64 {
    let mut p = 0i64;
    if joker_count.is_none() || joker_count == Some(0) {
        if vals[0] == 5 {
            // five of a kind
            p = 21;
        } else if vals[0] == 4 {
            // four of kind
            p = 20;
        } else if vals[0] == 3 && vals[1] == 2 {
            // full house
            p = 19;
        } else if vals[0] == 3 {
            // three of a kind
            p = 18;
        } else if vals[0] == 2 && vals[1] == 2 {
            // two pairs
            p = 17;
        } else if vals[0] == 2 {
            // one pair
            p = 16;
        } else if vals.len() == 5
        // five unique
        {
            p = 15;
        }
    } else if joker_count == Some(5) {
        p = 21;
    } else if joker_count == Some(1) {
        if vals[0] >= 4
        // x x x x J => five of a kind
        {
            p = 21;
        } else if vals[0] >= 3
        // x x x y J => 4 of a kind
        {
            p = 20;
        } else if vals[0] == 2 && vals[1] == 2
        // xx yy J
        {
            p = 19;
        } else if vals[0] == 2
        // x x y z J => 3 of a kind
        {
            p = 18;
        } else
        // 4 different + J => 1 pair
        {
            p = 16;
        }
    } else if joker_count == Some(2) {
        if vals[0] >= 3
        // x x x J J
        {
            p = 21;
        } else if vals[0] == 2
        // x x y J J => 4 of a kind
        {
            p = 20;
        } else
        // x y z J J => three of a kind
        {
            p = 18;
        }
    } else if joker_count == Some(3) {
        if vals[0] == 2 {
            p = 21;
        } else {
            p = 20;
        }
    } else if joker_count == Some(4) {
        p = 21;
    }

    p
}

fn hand_value(hand: &str, joker: bool) -> i64 {
    let mut value = hand
        .chars()
        .fold(0i64, |acc, card| 100 * acc + card_value(card, joker));

    let mut card_count = HashMap::new();
    for card in hand.chars() {
        card_count
            .entry(card)
            .and_modify(|v| *v += 1)
            .or_insert(1i64);
    }
    let mut j: Option<i64> = None;
    if joker {
        let j_count = card_count.get(&'J');
        if j_count.is_some() {
            j = Some(*(j_count.unwrap()));
        }
        card_count.remove(&'J');
    }

    let mut vals = card_count.values().cloned().collect::<Vec<_>>();
    vals.sort();
    vals.reverse();

    let p = 10000000000i64 * hand_value_multi(&vals, j);
    value += p;

    return value;
}

fn total_winnings(file_content: String, joker: bool) -> i64 {
    let mut hands_value_bet = file_content
        .lines()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            return (tokens[0], tokens[1].parse::<i64>().unwrap());
        })
        .map(|(hand, bet)| (hand_value(hand, joker), bet))
        .collect::<Vec<_>>();

    hands_value_bet.sort_by(|(value, _), (value2, _)| value.cmp(value2));

    let sum = (1..)
        .zip(hands_value_bet)
        .fold(0i64, |acc, (rank, (_, bet))| acc + rank * bet);
    return sum;
}

fn part1() {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let sum = total_winnings(file_content, false);
    println!("Part 1: {sum}");
}

fn part2() {
    let file_content = fs::read_to_string("input.txt").unwrap();
    let sum = total_winnings(file_content, true);
    println!("Part 2: {sum}");
}

fn main() {
    part1();
    part2();
}
