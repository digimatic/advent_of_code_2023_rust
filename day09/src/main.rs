use std::fs;

fn part1and2() {
    let input_content = fs::read_to_string("input.txt").unwrap();
    let histories = input_content.lines().map(|x| {
        x.split(" ")
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut sum = 0;
    let mut sum2 = 0;
    for history in histories {
        println!();
        let mut sequences = Vec::new();
        sequences.push(history);

        loop {
            let sequence = sequences.last().unwrap();
            let t = sequence.iter().fold((Vec::new(), 0), |(mut v, last), x| {
                let y = x - last;
                v.push(y);
                (v, *x)
            });
            let mut t2 = t.0;
            t2.remove(0);

            let all_zero = t2.iter().all(|x| *x == 0);
            sequences.push(t2);
            if all_zero {
                break;
            }
        }

        sum += sequences.iter().fold(0, |acc, xs| acc + xs.last().unwrap());

        sum2 += sequences
            .iter()
            .rev()
            .fold(0, |acc, xs| xs.first().unwrap() - acc);
    }
    println!("part1: {sum}");
    println!("part2: {sum2}");
}

fn main() {
    part1and2();
}
