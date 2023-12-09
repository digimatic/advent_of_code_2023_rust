use std;
use std::cmp::min;
use std::collections::HashSet;
use std::fs;

fn read_and_parse(input_file: &str) -> (Vec<i64>, Vec<Vec<(i64, i64, i64)>>) {
    let file_content = fs::read_to_string(input_file).unwrap();
    let lines = file_content.lines().collect::<Vec<_>>();

    let seeds = &lines[0][7..]
        .split(" ")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("seeds: {:?}", seeds);

    let mut i = 2;
    let mut maps = Vec::new();
    loop {
        i += 1;
        if i >= lines.len() {
            break;
        }
        let mut current_map = Vec::new();
        while i < lines.len() && lines[i].len() != 0 {
            let xs = lines[i]
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            current_map.push((xs[0], xs[1], xs[2]));
            i += 1;
        }
        maps.push(current_map);
        i += 1;
    }
    (seeds.clone(), maps)
}

#[derive(PartialEq, Clone, Debug)]
enum IntervalRelation {
    Inside,
    Before,
    After,
    CrossStart,
    CrossEnd,
    Encloses,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Interval {
    start: i64,
    count: i64,
}

impl Interval {
    // End is one past the last element in the Interval.
    fn end(&self) -> i64 {
        return self.start + self.count;
    }

    fn compare(&self, other: &Interval) -> IntervalRelation {
        if other.end() <= self.start {
            return IntervalRelation::Before;
        } else if other.start >= self.end() {
            return IntervalRelation::After;
        } else if other.start < self.start && other.end() > self.end() {
            return IntervalRelation::Encloses;
        } else if other.start >= self.start && other.end() <= self.end() {
            return IntervalRelation::Inside;
        } else if other.start < self.start {
            return IntervalRelation::CrossStart;
        } else {
            return IntervalRelation::CrossEnd;
        }
    }

    // split other against self so no of the new splitted
    // intervals crosses any borders of self.
    fn split(&self, other: &Interval) -> Vec<Interval> {
        let mut r: Vec<Interval> = Vec::new();

        match self.compare(other) {
            IntervalRelation::After => r.push(other.clone()),
            IntervalRelation::Before => r.push(other.clone()),
            IntervalRelation::Inside => r.push(other.clone()),
            IntervalRelation::Encloses => {
                r.push(Interval {
                    start: other.start,
                    count: (self.start - other.start),
                });
                r.push(self.clone());
                r.push(Interval {
                    start: self.end(),
                    count: (other.end() - self.end()),
                });
            }
            IntervalRelation::CrossEnd => {
                r.push(Interval {
                    start: other.start,
                    count: (self.end() - other.start),
                });
                r.push(Interval {
                    start: self.end(),
                    count: (other.end() - self.end()),
                });
            }
            IntervalRelation::CrossStart => {
                r.push(Interval {
                    start: other.start,
                    count: (self.start - other.start),
                });
                r.push(Interval {
                    start: self.start,
                    count: (other.end() - self.start),
                });
            }
        }
        return r;
    }
}

#[test]
fn test_interval_compare() {
    let i0 = Interval {
        start: 10,
        count: 90,
    };
    let i1 = Interval { start: 1, count: 5 }; // before
    let i2 = Interval {
        start: 200,
        count: 5,
    }; // after
    let i3 = Interval {
        start: 20,
        count: 5,
    }; // inside
    let i4 = Interval {
        start: 1,
        count: 200,
    }; // encapsuling
    let i5 = Interval {
        start: 1,
        count: 20,
    }; // crossstart
    let i6 = Interval {
        start: 50,
        count: 100,
    }; // crossend
    assert_eq!(i0.compare(&i1), IntervalRelation::Before);
    assert_eq!(i0.compare(&i2), IntervalRelation::After);
    assert_eq!(i0.compare(&i3), IntervalRelation::Inside);
    assert_eq!(i0.compare(&i4), IntervalRelation::Encloses);
    assert_eq!(i0.compare(&i5), IntervalRelation::CrossStart);
    assert_eq!(i0.compare(&i6), IntervalRelation::CrossEnd);
}

#[test]
fn test_interval_split() {
    let i0 = Interval {
        start: 10,
        count: 90,
    };
    let i1 = Interval { start: 1, count: 5 }; // before
    let i2 = Interval {
        start: 200,
        count: 5,
    }; // after
    let i3 = Interval {
        start: 20,
        count: 5,
    }; // inside
    let i4 = Interval {
        start: 1,
        count: 200,
    }; // encapsuling
    let i5 = Interval {
        start: 1,
        count: 20,
    }; // crossstart
    let i6 = Interval {
        start: 50,
        count: 100,
    }; // crossend

    assert_eq!(i0.split(&i1).len(), 1);
    assert_eq!(i0.split(&i2).len(), 1);
    assert_eq!(i0.split(&i3).len(), 1);

    assert_eq!(i0.split(&i4).len(), 3);
    assert_eq!(i0.split(&i5).len(), 2);
    assert_eq!(i0.split(&i6).len(), 2);
}

fn solve_part1(input_file: &str) {
    let (seeds, maps) = read_and_parse(input_file);

    let mut min_dest: Option<i64> = None;
    for seed_ref in &seeds {
        let mut seed = *seed_ref;
        for map in &maps {
            for (dst_start, src_start, n) in map {
                if seed >= *src_start && seed < (*src_start + *n) {
                    seed += *dst_start - *src_start;
                    break;
                }
            }
        }
        min_dest = match min_dest {
            None => Some(seed),
            Some(x) => Some(min(x, seed)),
        }
    }
//    println!("{:?}", min_dest.unwrap());
}

fn solve_part2(input_file: &str) {
    let (seeds, maps) = read_and_parse(input_file);

    let mut min_dest = i64::MAX;

    for seed in seeds.chunks(2) {
        let seed_start = seed[0];
        let seed_count = seed[1];
//        println!("{seed_start} {seed_count}");

        // current numbers to be transformed down the transform maps
        let mut s = HashSet::new();
        s.insert(Interval {
            start: seed_start,
            count: seed_count,
        });

        for mappings in &maps {
            // Split current intervals based on mappings
            loop {
                let mut s2 = Vec::new();
                let n = s.len();
                for current in &s {
                    for (_dst_start, src_start, count) in mappings {
                        let src = Interval {
                            start: *src_start,
                            count: *count,
                        };

                        let mut current_splitted = src.split(current);

                        // test
                        for c in &current_splitted {
                            let r = src.compare(c);
                            assert!(r != IntervalRelation::CrossEnd);
                            assert!(r != IntervalRelation::CrossStart);
                            assert!(r != IntervalRelation::Encloses);
                        }

                        s2.append(&mut current_splitted);
                    }
                }

                s.clear();
                for c in &s2 {
                    s.insert(*c);
                }
                if s.len() == n {
                    break;
                }
            }

            // Now do mapping
            let s2 = s;
            s = HashSet::new();
            for mut interval in s2 {
                if interval.count == 0 {
                    continue;
                }

                let mut mapped = false;
                for (dst_start, src_start, count) in mappings {
                    let src = Interval {
                        start: *src_start,
                        count: *count,
                    };

                    let r = src.compare(&interval);
                    if  r == IntervalRelation::Inside {
                        interval.start += dst_start - src_start;
                        s.insert(interval);
                        mapped = true;
                        break;
                    }
                    // assert!(r != IntervalRelation::CrossEnd);
                    // assert!(r != IntervalRelation::CrossStart);
                    // assert!(r != IntervalRelation::Encloses);
                }
                if !mapped {
                    s.insert(interval);
                }
            }
        }

        for interval in s {
            min_dest = min(interval.start, min_dest);
            if interval.start == 0 {
                println!("0000000");
            }
        }
    }

    println!("Part 2: {min_dest}");
}

fn main() {
    solve_part1("input.txt");
    solve_part2("input.txt");
}
