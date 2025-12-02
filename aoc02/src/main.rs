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

struct Range {
    start: usize,
    end: usize,
    start_str: String,
    end_str: String,
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

fn check_range(r: &Range) -> usize {
    let mut sum = 0;
    println!("range: {} {}", r.start, r.end);
    let s_midpoint = r.start_str.len() / 2;
    let e_midpoint = r.end_str.len() / 2;
    let (sl, sr) = r.start_str.split_at(s_midpoint);
    let (el, er) = r.end_str.split_at(e_midpoint);

    println!("subs: {sl} {sr}");

    let mut start_left = sl.parse::<usize>().unwrap_or_default();

    // We can't start with zero, so just try moving things along first
    if start_left == 0 {
        start_left = 1;
    }

    loop {
        let sl = format!("{start_left}");
        let try_str = [sl.clone(), sl.clone()].concat();
        let try_val = try_str.parse::<usize>().unwrap();
        if try_val >= r.start && try_val <= r.end {
            println!("found: {try_val}");
            sum += try_val;
        }
        if try_val >= r.end {
            break;
        }
        start_left += 1;
    }

    sum
}

fn compute_part1(ranges: &Vec<Range>) -> usize {
    let mut sum = 0;

    for r in ranges {
        sum += check_range(r);
    }

    println!("invalid ids: {sum}");

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

fn main() {
    compute_part1(&mut parse_lines(&get_input("prelim.txt")));
    compute_part1(&mut parse_lines(&get_input("input.txt")));
}
