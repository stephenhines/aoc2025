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

enum Rotate {
    Left,
    Right,
}

struct Rotation {
    rotate: Rotate,
    distance: i32,
}

fn parse_lines(lines: &Vec<String>) -> Vec<Rotation> {
    let mut v: Vec<Rotation> = Vec::new();
    for line in lines {
        let (dir, dist) = line.split_at(1);
        let rotate = if dir == "L" {
            Rotate::Left
        } else {
            Rotate::Right
        };
        let distance = dist.parse::<i32>().unwrap();
        v.push(Rotation { rotate, distance });
    }
    v
}

fn compute_part1(rotations: &Vec<Rotation>) -> usize {
    let mut num_zero_steps = 0;
    let mut dial = 50;

    for rot in rotations {
        let dist = rot.distance;
        match rot.rotate {
            Rotate::Left => {
                dial -= dist;
                if dial < 0 {
                    dial %= 100;
                }
            }
            Rotate::Right => {
                dial += dist;
                if dial > 99 {
                    dial %= 100;
                }
            }
        }
        if dial == 0 {
            num_zero_steps += 1;
        }
    }

    println!("passwd: {num_zero_steps}");

    num_zero_steps
}

fn compute_part2(rotations: &Vec<Rotation>) -> usize {
    let mut num_zero_steps = 0;
    let mut dial = 50;

    for rot in rotations {
        let dist = rot.distance;
        match rot.rotate {
            Rotate::Left => {
                // Handle the case for starting at 0 correctly
                if dial == 0 {
                    dial = 100;
                }
                dial -= dist;

                while dial < 0 {
                    dial += 100;
                    num_zero_steps += 1;
                }
                if dial == 0 {
                    num_zero_steps += 1;
                }
            }
            Rotate::Right => {
                dial += dist;
                while dial > 99 {
                    dial -= 100;
                    num_zero_steps += 1;
                }
            }
        }
        //println! {"dial: {dial} {num_zero_steps}"};
    }

    println!("passwd: {num_zero_steps}");

    num_zero_steps
}

#[test]
fn test_prelim() {
    let passwd = compute_part1(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(passwd, 3);
}

#[test]
fn test_part1() {
    let passwd = compute_part1(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(passwd, 1066);
}

#[test]
fn test_prelim2() {
    let passwd = compute_part2(&mut parse_lines(&get_input("prelim.txt")));
    assert_eq!(passwd, 6);
}

#[test]
fn test_part2() {
    let passwd = compute_part2(&mut parse_lines(&get_input("input.txt")));
    assert_eq!(passwd, 6223);
}

fn main() {
    compute_part1(&mut parse_lines(&get_input("prelim.txt")));
    compute_part1(&mut parse_lines(&get_input("input.txt")));
    compute_part2(&mut parse_lines(&get_input("prelim.txt")));
    compute_part2(&mut parse_lines(&get_input("input.txt")));
}
