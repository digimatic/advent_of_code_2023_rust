use std::fs;

fn part1() {
    let input_content = fs::read_to_string("inputExample1.txt").unwrap();
    let histories = input_content.lines().map(|x| {
        x.split(" ")
            .map(|y| y.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
    });
    for history in histories {
        let mut sequences = Vec::new();
        sequences.push(history);

        loop {
            let sequence = sequences.last().unwrap();
            let t = sequence.iter().fold( (Vec::new(), 0),
                |(mut v, last), x| {
                    let y = x - last;
                    v.push(y);
                    (v, *x)
                });
            let mut t2 = t.0;
            t2.remove(0);
            println!("{:?}", t2);
            sequences.push(t2);
        }
    }
}

fn main() {
    part1();
}
