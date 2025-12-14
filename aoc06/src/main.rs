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

struct CephalopodWorksheet {
    rows: Vec<Vec<char>>,
    line_len: usize,
}

impl CephalopodWorksheet {
    pub fn new(lines: &[String]) -> Self {
        let mut rows = Vec::new();

        let line_len = lines[0].len();

        for line in lines {
            let row_chars = line.chars().collect::<Vec<_>>();
            if row_chars.len() != line_len {
                panic!("Mismatched line lengths!\n{line}");
            }
            rows.push(row_chars);
        }

        Self { rows, line_len }
    }

    fn calc_ceph(&self, cur_index: usize, last_index: usize) -> usize {
        let mut result;

        let mut operands = Vec::new();
        // Get the operands first
        for col in (cur_index..=last_index).rev() {
            let mut found = false;
            let mut operand = 0;
            for row in 0..self.rows.len() - 1 {
                let ch = self.rows[row][col];
                if ch.is_digit(10) {
                    found = true;
                    operand *= 10;
                    operand += ch.to_digit(10).unwrap();
                }
            }
            if found {
                operands.push(operand as usize);
            }
        }

        let op_row = &self.rows[self.rows.len() - 1];
        let op = op_row[cur_index];
        match op {
            '*' => {
                result = 1;
                for operand in operands {
                    result *= operand;
                }
            },
            '+' => {
                result = 0;
                for operand in operands {
                    result += operand;
                }
            },
            c => { panic!("Invalid char: {c}"); },
        }

        result
    }

    pub fn calculate(&self) -> usize {
        let mut total_sum = 0;

        let mut last_index = self.line_len - 1;

        let op_row = &self.rows[self.rows.len() - 1];

        while last_index != 0 {
            let mut cur_index = last_index;
            while op_row[cur_index] == ' ' {
                cur_index -= 1;
            }
            total_sum += self.calc_ceph(cur_index, last_index);

            last_index = cur_index;
            if last_index != 0 {
                // hop over the empty column too
                last_index -= 2;
            }
        }

        println!("total_sum (cephalopod): {total_sum}");
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

#[test]
fn test_prelim2() {
    let sum = CephalopodWorksheet::new(&get_input("prelim.txt")).calculate();
    assert_eq!(sum, 3263827);
}

#[test]
fn test_part2() {
    let sum = CephalopodWorksheet::new(&get_input("input.txt")).calculate();
    assert_eq!(sum, 11494432585168);
}

fn main() {
    let worksheet = Worksheet::new(&get_input("prelim.txt"));
    worksheet.calculate();
    let worksheet = Worksheet::new(&get_input("input.txt"));
    worksheet.calculate();
    let worksheet = CephalopodWorksheet::new(&get_input("prelim.txt"));
    worksheet.calculate();
    let worksheet = CephalopodWorksheet::new(&get_input("input.txt"));
    worksheet.calculate();
}
