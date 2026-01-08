use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use good_lp::{constraint, default_solver, variable, variables, Expression, Solution, SolverModel};

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
    wiring_indices: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl InitLine {
    pub fn new(line: &String) -> Self {
        let mut light = 0;
        let mut wiring = Vec::new();
        let mut wiring_indices = Vec::new();
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
            let mut indices_vec = Vec::new();
            wiring_strs.split(',').for_each(|w| {
                let index = w.parse::<usize>().unwrap();
                val += 1 << index;
                indices_vec.push(index);
            });
            wiring.push(val);
            wiring_indices.push(indices_vec);
        }

        Self {
            num_lights,
            light,
            wiring,
            wiring_indices,
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

    // I needed to write this because I missed out that I needed to "round up"
    // the floating point solutions.
    pub fn check_part2(&self, button_presses: Vec<usize>) {
        let mut joltage = vec![0usize; self.num_lights];
        for (i, presses) in button_presses.iter().enumerate() {
            let buttons = &self.wiring_indices[i];
            for &button in buttons {
                joltage[button] += presses;
            }
        }

        for i in 0..self.num_lights {
            if self.joltage[i] != joltage[i] {
                println!("Failed! {self:?}");
                println!("presses: {button_presses:?}");
                println!("expected joltage: {:?}", self.joltage);
                println!("got joltage:      {:?}", joltage);
            }
            assert_eq!(self.joltage[i], joltage[i]);
        }
    }

    pub fn part2(&self) -> usize {
        let mut prob_vars = variables!();
        let vars = prob_vars.add_vector(variable().integer().min(0), self.wiring.len());

        let mut constraints = Vec::new();
        for (i, joltage) in self.joltage.iter().enumerate() {
            let mut sum = Expression::from(0);
            for (w, wire) in self.wiring.iter().enumerate() {
                if wire & (1 << i) != 0 {
                    sum += vars[w];
                }
            }
            let joltage_expr = Expression::from(*joltage as i32);
            let constraint = constraint!(sum == joltage_expr);
            constraints.push(constraint);
        }

        let objective: Expression = vars.iter().sum();
        //let model = prob_vars.minimise(objective).using(default_solver);
        let model = prob_vars.minimise(objective).using(default_solver);
        let sol = model.with_all(constraints).solve().unwrap();
        let mut min_presses = 0;
        let mut button_presses = Vec::new();
        for v in vars {
            let val = sol.value(v).round() as usize;
            min_presses += val;
            button_presses.push(val);
        }
        //println!("min_presses(j): {min_presses}");

        self.check_part2(button_presses);

        min_presses
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

fn part2(init_lines: &Vec<InitLine>) -> usize {
    let mut min_presses = 0;

    for line in init_lines {
        min_presses += line.part2();
    }
    println!("min_presses (joltage): {min_presses}");

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

#[test]
fn test_prelim2() {
    let presses = part2(&parse_lines(&get_input("prelim.txt")));
    assert_eq!(presses, 33);
}

#[test]
fn test_part2() {
    let presses = part2(&parse_lines(&get_input("input.txt")));
    assert_eq!(presses, 16663);
}

fn main() {
    let init_lines = parse_lines(&get_input("prelim.txt"));
    min_presses(&init_lines);
    part2(&init_lines);

    let init_lines = parse_lines(&get_input("input.txt"));
    min_presses(&init_lines);
    part2(&init_lines);
}
