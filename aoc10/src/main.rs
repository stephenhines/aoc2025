use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::error::Error;

use good_lp::{constraint, default_solver, Solution, SolverModel, variables};

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
struct InitLine {
    num_lights: usize,
    light: usize,
    wiring: Vec<usize>,
    joltage: Vec<usize>,
}

impl InitLine {
    pub fn new(line: &String) -> Self {
        let mut light = 0;
        let mut wiring = Vec::new();
        let mut joltage = Vec::new();

        let toks = line.split_whitespace().collect::<Vec<_>>();
        if toks.len() < 3 {
            panic!("Invalid line: {line}");
        }

        //  0123
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let light_toks = toks[0];
        assert_eq!(light_toks.chars().nth(0).unwrap(), '[');
        assert_eq!(light_toks.chars().nth(light_toks.len() - 1).unwrap(), ']');
        light_toks.chars().collect::<Vec<_>>()[1..light_toks.len() - 1]
            .iter()
            .enumerate()
            .for_each(|(i, c)| match c {
                '.' => {}
                '#' => {
                    light += 1 << i;
                }
                c => {
                    panic!("Invalid light token: {c} in {light_toks}");
                }
            });
        let num_lights = light_toks.len() - 2;

        let last_tok = toks[toks.len() - 1];
        assert_eq!(last_tok.chars().nth(0).unwrap(), '{');
        assert_eq!(last_tok.chars().nth(last_tok.len() - 1).unwrap(), '}');
        let joltage_toks = &last_tok[1..last_tok.len() - 1];
        joltage_toks.split(',').for_each(|j| {
            joltage.push(j.parse::<usize>().unwrap());
        });

        let wiring_toks = &toks[1..toks.len() - 1];
        for &w in wiring_toks {
            assert_eq!(w.chars().nth(0).unwrap(), '(');
            assert_eq!(w.chars().nth(w.len() - 1).unwrap(), ')');
            let wiring_strs = &w[1..w.len() - 1];
            let mut val = 0;
            wiring_strs.split(',').for_each(|w| {
                val += 1 << w.parse::<usize>().unwrap();
            });
            wiring.push(val);
        }

        Self {
            num_lights,
            light,
            wiring,
            joltage,
        }
    }

    pub fn check_wires_bitvector(&self, wire_bv: usize) -> Option<usize> {
        let mut m_wire_bv = wire_bv;
        let mut try_light = 0;
        let mut i = 0;
        let mut presses = 0;
        while m_wire_bv != 0 {
            if m_wire_bv & 1 != 0 {
                try_light ^= self.wiring[i];
                presses += 1;
            }
            m_wire_bv >>= 1;
            i += 1;
        }

        if self.light == try_light {
            Some(presses)
        } else {
            None
        }
    }

    pub fn min_presses(&self) -> usize {
        let mut min_presses = usize::MAX;

        for val in 1..1 << self.wiring.len() {
            if let Some(presses) = self.check_wires_bitvector(val) {
                min_presses = cmp::min(min_presses, presses);
            }
        }

        min_presses
    }

    pub fn part2(&self) -> usize {
        let mut min_presses = 0;

        let mut vars = variables!();
        let a = vars.add_variable();
        let b = vars.add_variable();
        let c = vars.add_variable();
        let d = vars.add_variable();
        let e = vars.add_variable();
        let f = vars.add_variable();

        let mut constraints = Vec::new();
        constraints.push(constraint!(e + f == 3));
        constraints.push(constraint!(b + f == 5));
        constraints.push(constraint!(c + d + e == 4));
        constraints.push(constraint!(a + b + d == 7));
        constraints.push(constraint!(a >= 0));
        constraints.push(constraint!(b >= 0));
        constraints.push(constraint!(c >= 0));
        constraints.push(constraint!(d >= 0));
        constraints.push(constraint!(e >= 0));
        constraints.push(constraint!(f >= 0));

        let sol = vars.minimise(a + b + c + d + e + f).using(default_solver).with_all(constraints).solve().unwrap();
        let presses  = sol.eval(a + b + c + d + e + f);
        println!("presses: {presses}");
        min_presses = presses as usize;

        println!("min_presses: {min_presses}");

        //  0123   a   b b   c   d d   e e   f f
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}

        min_presses as usize
    }
}

fn min_presses(init_lines: &Vec<InitLine>) -> usize {
    let mut min_presses = 0;

    for line in init_lines {
        min_presses += line.min_presses();
    }
    println!("min_presses: {min_presses}");

    min_presses
}

fn parse_lines(lines: &Vec<String>) -> Vec<InitLine> {
    let mut init_lines = Vec::new();
    for line in lines {
        init_lines.push(InitLine::new(&line));
    }
    init_lines
}

#[test]
fn test_prelim() {
    let presses = min_presses(&parse_lines(&get_input("prelim.txt")));
    assert_eq!(presses, 7);
}

#[test]
fn test_part1() {
    let presses = min_presses(&parse_lines(&get_input("input.txt")));
    assert_eq!(presses, 415);
}

fn main() {
    let init_lines = parse_lines(&get_input("prelim.txt"));
    min_presses(&init_lines);
    init_lines[0].part2();
    let init_lines = parse_lines(&get_input("input.txt"));
    min_presses(&init_lines);
}
