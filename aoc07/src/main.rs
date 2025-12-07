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

    pub fn is_empty(&self, col: usize, row: usize) -> bool {
        self.get_elem(col, row) == '.'
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

    pub fn add_splits(&mut self) {
        let mut col = 0;
        let mut row = 0;
        let mut start = Default::default();
        for col in 0..self.cols {
            if self.is_start(col, row) {
                start = (col, row);
                // Add a beam to below the start
                self.add_beam(col, row + 1);
                break;
            }
        }

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
                        }
                        '|' => {}
                        c => {
                            panic!("Unhandled tachyon space {c}");
                        }
                    }
                }
            }
        }
    }

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

    pub fn is_available(&self, col: usize, row: usize) -> bool {
        if self.get_elem(col, row) != '@' {
            return false;
        }

        let mut num_used = 0;
        for (c, r) in [
            (col - 1, row - 1),
            (col, row - 1),
            (col + 1, row - 1),
            (col - 1, row),
            (col + 1, row),
            (col - 1, row + 1),
            (col, row + 1),
            (col + 1, row + 1),
        ] {
            if self.get_elem(c, r) == '@' {
                num_used += 1;
            }
        }
        num_used < 4
    }
}

#[test]
fn test_prelim() {
    let mut grid = TachyonGrid::create(&get_input("prelim.txt"));
    grid.add_splits();
    let splits = grid.count_splits();
    assert_eq!(splits, 21);
}

#[test]
fn test_part1() {
    let mut grid = TachyonGrid::create(&get_input("input.txt"));
    grid.add_splits();
    let splits = grid.count_splits();
    assert_eq!(splits, 1504);
}

fn main() {
    let mut grid = TachyonGrid::create(&get_input("prelim.txt"));
    grid.add_splits();
    let splits = grid.count_splits();
    let mut grid = TachyonGrid::create(&get_input("input.txt"));
    grid.add_splits();
    let splits = grid.count_splits();
    //grid.print();
}
