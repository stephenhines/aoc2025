use std::collections::HashMap;
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

struct TachyonGrid {
    rows: usize,
    cols: usize,
    elems: Box<[char]>,
}

impl TachyonGrid {
    fn helper_get_idx(col: usize, row: usize, cols: usize) -> usize {
        row * cols + col
    }

    pub fn get_idx(&self, col: usize, row: usize) -> usize {
        Self::helper_get_idx(col, row, self.cols)
    }

    pub fn get_elem(&self, col: usize, row: usize) -> char {
        self.elems[self.get_idx(col, row)]
    }

    pub fn create(lines: &Vec<String>) -> Self {
        let rows = lines.len();
        let cols = lines[0].len();
        let mut elems: Box<[char]> = vec![' '; rows * cols].into_boxed_slice();

        lines.iter().enumerate().for_each(|(row, line)| {
            line.char_indices().for_each(|(col, val)| {
                elems[Self::helper_get_idx(col, row, cols)] = val;
            });
        });

        TachyonGrid { rows, cols, elems }
    }

    #[allow(unused)]
    pub fn print(&self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{}", self.get_elem(col, row));
            }
            println!();
        }
    }

    pub fn is_start(&self, col: usize, row: usize) -> bool {
        self.get_elem(col, row) == 'S'
    }

    pub fn get_start(&self) -> (usize, usize) {
        for col in 0..self.cols {
            if self.is_start(col, 0) {
                return (col, 0);
            }
        }
        panic!("start not found");
    }

    pub fn is_splitter(&self, col: usize, row: usize) -> bool {
        self.get_elem(col, row) == '^'
    }

    pub fn is_beam(&self, col: usize, row: usize) -> bool {
        self.get_elem(col, row) == '|'
    }

    pub fn add_beam(&mut self, col: usize, row: usize) {
        self.elems[self.get_idx(col, row)] = '|';
    }

    pub fn add_splits(&mut self) -> usize {
        // Add a beam below the start
        let (col, row) = self.get_start();
        self.add_beam(col, row + 1);

        let mut splits = 0;
        for row in 1..self.rows {
            for col in 0..self.cols {
                // Check above for beam
                if self.is_beam(col, row - 1) {
                    match self.get_elem(col, row) {
                        '.' => {
                            self.add_beam(col, row);
                        }
                        '^' => {
                            // Split the beam here
                            self.add_beam(col - 1, row);
                            self.add_beam(col + 1, row);
                            splits += 1;
                        }
                        '|' => {}
                        c => {
                            panic!("Unhandled tachyon space {c}");
                        }
                    }
                }
            }
        }

        println!("splits: {splits}");

        splits
    }

    #[allow(unused)]
    pub fn count_splits(&self) -> usize {
        let mut splits = 0;
        // Start on row 1
        for row in 1..self.rows {
            for col in 0..self.cols {
                if self.is_splitter(col, row) && self.is_beam(col, row - 1) {
                    splits += 1;
                }
            }
        }

        println!("splits: {splits}");
        splits
    }

    // Dynamic programming memoization (recursive helper function)
    fn count_timelines_memo(
        &self,
        memo: &mut HashMap<(usize, usize), usize>,
        col: usize,
        row: usize,
    ) -> usize {
        if memo.contains_key(&(col, row)) {
            return *memo.get(&(col, row)).unwrap();
        }

        // Base case for recursion
        if row == self.rows - 1 {
            memo.insert((col, row), 1);
            return 1;
        }

        let timelines = match self.get_elem(col, row) {
            '.' | 'S' | '|' => self.count_timelines_memo(memo, col, row + 1),
            '^' => {
                self.count_timelines_memo(memo, col - 1, row + 1)
                    + self.count_timelines_memo(memo, col + 1, row + 1)
            }
            c => {
                panic!("Unhandled tachyon space {c}");
            }
        };
        memo.insert((col, row), timelines);
        timelines
    }

    pub fn count_timelines(&self) -> usize {
        let mut memo = HashMap::new();
        let (col, row) = self.get_start();

        let timelines = self.count_timelines_memo(&mut memo, col, row);
        println!("timelines: {timelines}");
        timelines
    }
}

#[test]
fn test_prelim() {
    let mut grid = TachyonGrid::create(&get_input("prelim.txt"));
    let splits = grid.add_splits();
    assert_eq!(splits, 21);
}

#[test]
fn test_part1() {
    let mut grid = TachyonGrid::create(&get_input("input.txt"));
    let splits = grid.add_splits();
    assert_eq!(splits, 1504);
}

#[test]
fn test_prelim2() {
    let grid = TachyonGrid::create(&get_input("prelim.txt"));
    let timelines = grid.count_timelines();
    assert_eq!(timelines, 40);
}

#[test]
fn test_part2() {
    let grid = TachyonGrid::create(&get_input("input.txt"));
    let timelines = grid.count_timelines();
    assert_eq!(timelines, 5137133207830);
}

fn main() {
    let mut grid = TachyonGrid::create(&get_input("prelim.txt"));
    grid.add_splits();
    grid.count_timelines();
    //grid.print();
    let mut grid = TachyonGrid::create(&get_input("input.txt"));
    grid.add_splits();
    grid.count_timelines();
    //grid.print();
}
