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

struct Worksheet {
    problems: Vec<Vec<usize>>,
    ops: Vec<char>,
}

impl Worksheet {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut problems = Vec::new();
        let mut ops = Vec::new();

        for line in &lines[0..lines.len() - 1] {
            let mut operand_line = Vec::new();
            line.split_ascii_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .for_each(|&n| {
                    operand_line.push(n.parse::<usize>().unwrap());
                });
            problems.push(operand_line);
        }

        lines[lines.len() - 1]
            .split_ascii_whitespace()
            .collect::<Vec<_>>()
            .iter()
            .for_each(|&s| match s {
                "*" | "+" => ops.push(s.chars().nth(0).unwrap()),
                c => panic!("Unhandled operation {c}"),
            });

        //println!("problems {problems:?}");
        //println!("ops {ops:?}");
        Worksheet { problems, ops }
    }

    pub fn calculate(&self) -> usize {
        let mut total_sum = 0;

        for (i, op) in self.ops.iter().enumerate() {
            match op {
                '*' => {
                    let mut product = 1;
                    for operand in &self.problems {
                        product *= operand[i];
                    }
                    total_sum += product;
                }
                '+' => {
                    let mut sum = 0;
                    for operand in &self.problems {
                        sum += operand[i];
                    }
                    total_sum += sum;
                }
                c => {
                    panic!("Unhandled operation {c}");
                }
            }
        }

        println!("total_sum: {total_sum}");
        total_sum
    }
}

#[test]
fn test_prelim() {
    let sum = Worksheet::new(&get_input("prelim.txt")).calculate();
    assert_eq!(sum, 4277556);
}

#[test]
fn test_part1() {
    let sum = Worksheet::new(&get_input("input.txt")).calculate();
    assert_eq!(sum, 6378679666679);
}

fn main() {
    let worksheet = Worksheet::new(&get_input("prelim.txt"));
    worksheet.calculate();
    let worksheet = Worksheet::new(&get_input("input.txt"));
    worksheet.calculate();
}
