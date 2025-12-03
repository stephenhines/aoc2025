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

fn parse_lines(lines: &Vec<String>) -> Vec<Vec<u32>> {
    let mut batteries: Vec<_> = Vec::new();
    for line in lines {
        let mut row: Vec<_> = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap());
        }
        batteries.push(row);
    }
    batteries
}

fn compute_joltage(batteries: &Vec<Vec<u32>>) -> u32 {
    let mut joltage = 0;

    for b in batteries {
        let last_hi = b.len() - 1;
        let mut hi_idx = 0;
        let mut hi = b[hi_idx];

        let mut idx = 0;
        while idx < last_hi {
            if b[idx] > hi {
                hi_idx = idx;
                hi = b[hi_idx];
            }
            idx += 1;
        }

        idx = hi_idx + 1;
        let last_lo = b.len();

        let mut lo_idx = idx;
        let mut lo = b[lo_idx];
        while idx < last_lo {
            if b[idx] > lo {
                lo_idx = idx;
                lo = b[lo_idx];
            }
            idx += 1;
        }

        let jolt = hi * 10 + lo;
        //println!("jolt: {jolt}");

        joltage += jolt;
    }

    println!("joltage: {joltage}");

    joltage
}

#[test]
fn test_prelim() {
    let joltage = compute_joltage(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(joltage, 357);
}

#[test]
fn test_part1() {
    let joltage = compute_joltage(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(joltage, 17031);
}

fn main() {
    compute_joltage(&mut parse_lines(&get_input("prelim.txt")));
    compute_joltage(&mut parse_lines(&get_input("input.txt")));
}
