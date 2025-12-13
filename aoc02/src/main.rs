use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_input(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    lines
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
    start_str: String,
    end_str: String,
}

impl Range {
    pub fn check_range(&self) -> usize {
        let mut sum = 0;
        let s_midpoint = self.start_str.len() / 2;
        let (sl, _) = self.start_str.split_at(s_midpoint);

        let mut start_left = sl.parse::<usize>().unwrap_or_default();

        // We can't start with zero, so just try moving things along first
        if start_left == 0 {
            start_left = 1;
        }

        loop {
            let sl = format!("{start_left}");
            let try_str = [sl.clone(), sl.clone()].concat();
            let try_val = try_str.parse::<usize>().unwrap();
            if try_val >= self.start && try_val <= self.end {
                //println!("found: {try_val}");
                sum += try_val;
            }
            if try_val >= self.end {
                break;
            }
            start_left += 1;
        }

        sum
    }

    fn check_range_digits(&self, repeating_digits: usize) -> HashSet<usize> {
        let starting_value = 10usize.pow(repeating_digits as u32 - 1);

        let start_len = self.start_str.len();
        let end_len = self.end_str.len();
        let mut num_start_copies = start_len / repeating_digits;
        let num_end_copies = end_len / repeating_digits;

        // Make sure we don't try to use just a single copy of something
        if num_start_copies == 1 {
            num_start_copies = 2;
        }

        let mut invalid_ids = HashSet::new();

        // We need to handle the case where the number of digits changes
        // between the start and end values, so we just iterate over
        // how many copies would be needed for the start value through
        // the number of copies needed for the end value.
        for copies in num_start_copies..=num_end_copies {
            let mut value = starting_value;

            // Since we're using a loop over "copies", we can just stop once
            // we have rolled over into a different number of digits in the
            // string to copy. The higher range will be handled in a different
            // instance of this function/loop.
            while value < 10 * starting_value {
                let mut copy_value = 0;
                for i in 0..copies {
                    copy_value += value * 10u32.pow(i as u32 * repeating_digits as u32) as usize;
                }

                if copy_value > self.end {
                    break;
                }

                if copy_value >= self.start {
                    //println!("Found: {copy_value}");
                    invalid_ids.insert(copy_value);
                }

                value += 1;
            }
        }

        invalid_ids
    }

    pub fn check_range_part2(&self) -> usize {
        let mut invalid_ids = HashSet::new();
        let mut sum = 0;
        for i in 1..=self.end_str.len() / 2 {
            invalid_ids.extend(self.check_range_digits(i));
        }
        invalid_ids.iter().for_each(|id| { sum += id; });

        sum
    }
}

fn parse_lines(lines: &Vec<String>) -> Vec<Range> {
    let mut v: Vec<Range> = Vec::new();
    for line in lines {
        let ranges = line.split(',').collect::<Vec<_>>();
        for r in ranges {
            let rs = r.split('-').collect::<Vec<_>>();
            if rs.len() != 2 {
                panic!("Invalid range: {line}");
            }
            v.push(Range {
                start: rs[0].parse().unwrap(),
                end: rs[1].parse().unwrap(),
                start_str: rs[0].to_string(),
                end_str: rs[1].to_string(),
            });
        }
    }
    v
}

fn compute_part1(ranges: &Vec<Range>) -> usize {
    let mut sum = 0;

    for r in ranges {
        sum += r.check_range();
    }

    println!("invalid ids: {sum}");

    sum
}

fn compute_part2(ranges: &Vec<Range>) -> usize {
    let mut sum = 0;

    for r in ranges {
        sum += r.check_range_part2();
    }

    println!("invalid ids part 2: {sum}");

    sum
}

#[test]
fn test_prelim() {
    let invalid_ids = compute_part1(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(invalid_ids, 1227775554);
}

#[test]
fn test_part1() {
    let invalid_ids = compute_part1(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(invalid_ids, 34826702005);
}

#[test]
fn test_prelim2() {
    let invalid_ids = compute_part2(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(invalid_ids, 4174379265);
}

#[test]
fn test_part2() {
    let invalid_ids = compute_part2(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(invalid_ids, 43287141963);
}

fn main() {
    compute_part1(&mut parse_lines(&get_input("prelim.txt")));
    compute_part1(&mut parse_lines(&get_input("input.txt")));
    compute_part2(&mut parse_lines(&get_input("prelim.txt")));
    compute_part2(&mut parse_lines(&get_input("input.txt")));
}
