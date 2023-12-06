use std::fs;

fn part1() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let lines = file_contents.lines().collect::<Vec<_>>();
    let times_str = lines[0].replace("Time:", "");
    let dist_str = lines[1].replace("Distance:", "");
    let tss = times_str.split(" ").flat_map(|x| x.parse::<i64>());
    let dss = dist_str.split(" ").flat_map(|x| x.parse::<i64>());
    let tds = tss.zip(dss).collect::<Vec<_>>();

    let mut sum = 1i64;
    for (total_time, total_distance) in tds {
        let mut wins = 0;

        for button_time in 0..=total_time {
            let move_time = total_time - button_time;
            let speed = button_time;
            let distance = speed * move_time;
            if distance > total_distance {
                wins += 1;
            }
        }
        sum *= wins;
    }
    println!("{sum}");
}

fn part2() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let lines = file_contents.lines().collect::<Vec<_>>();
    let total_time = lines[0]
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let total_distance = lines[1]
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let mut wins = 0i64;

    for button_time in 0..=total_time {
        let move_time = total_time - button_time;
        let speed = button_time;
        let distance = speed * move_time;
        if distance > total_distance {
            wins += 1;
        }
    }
    println!("{wins}");
}

fn main() {
    part1();
    part2();
}
