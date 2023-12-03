use std::{fs, collections::HashSet};

fn is_digit(c: &char) -> bool {
    c.is_digit(10)
}

fn read_at(map: &Vec<Vec<char>>, x: isize, y: isize) -> char {
    let h = map.len().try_into().unwrap();
    let w = map[0].len().try_into().unwrap();
    if x >= 0 && y >= 0 && x < w && y < h {
        return map[y as usize][x as usize];
    }
    '.'
}

fn solve_part1() {
    let filestr = fs::read_to_string("input.txt").unwrap();
    let lines = filestr.lines();
    let map = lines
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let h = map.len() as isize;
    let w = map[0].len() as isize;
    let mut sum = 0;

    for y in 0..h {
        let mut start: Option<isize> = None;
        let mut x: isize = 0;
        let mut symbol_found = false;

        while x < (w + 1) {
            let c = read_at(&map, x, y);
            if is_digit(&c) {
                if start == None {
                    start = Some(x);
                }
                for dy in -1..2 {
                    for dx in -1..2 {
                        let cc = read_at(&map, x + dx, y + dy);
                        if !is_digit(&cc) && cc != '.' {
                            symbol_found = true;
                        }
                    }
                }
            } else {
                if symbol_found {
                    let startu = start.unwrap() as usize;
                    let xu = x as usize;
                    let s = &map[y as usize][startu..xu];
                    let ss: String = s.iter().collect();
                    let n = ss.parse::<i32>().unwrap();
                    sum += n;
                }
                start = None;
                symbol_found = false;
            }
            x += 1;
        }
    }

    println!("part1: {sum}");
}

fn find_number_start_index(map: &Vec<Vec<char>>, xx: isize, yy: isize) -> isize {
    let mut xi = xx;
    loop {
        if ! is_digit(&read_at(map, xi - 1, yy)) {
            return xi;
        }
        xi -= 1;
    }
}

fn solve_part2() {
    let filestr = fs::read_to_string("input.txt").unwrap();
    let lines = filestr.lines();
    let map = lines
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;

    let ii = &map.iter().enumerate().map(
        |(j, l)| {
            return l.iter().enumerate().filter_map(|(i, c)| {
                if *c == '*' {
                    return Some( (i, j) );
                } else {
                    return None;
                }
            }).collect::<Vec<(usize, usize)>>()
        }).flatten().collect::<Vec<(usize, usize)>>();


    for (x, y) in ii {
        let mut numbers: HashSet<(isize, isize)> = HashSet::new();

        for dy in -1..2 {
            for  dx in -1..2 {
                let xx = isize::try_from(*x).unwrap()+ dx;
                let yy = isize::try_from(*y).unwrap()+ dy;
                if is_digit(&read_at(&map, xx, yy)) {
                    let start_index = find_number_start_index(&map, xx, yy);
                    numbers.insert((start_index, yy));
                }
            }
        }

        if numbers.len() == 2 {
            let mut product = 1;
            for (start_x, y) in numbers {
                let j = y as usize;
                let i = start_x as usize;
                let mut end_x = start_x;
                while is_digit(&read_at(&map, end_x, y)) {
                    end_x+=1;
                }

                let i2 = end_x as usize;
                let s = &map[j as usize][i..i2];
                let ss: String = s.iter().collect();
                let val = ss.parse::<i32>().unwrap();
                product *= val;
            }
            sum += product;
        }

    }

    println!("part2: {sum}");
}


fn main() {
    solve_part1();
   solve_part2();
}
