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

struct Grid {
    rows: usize,
    cols: usize,
    elems: Box<[char]>,
}

impl Grid {
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
        let rows = lines.len() + 2;
        let cols = lines[0].len() + 2;
        let mut elems: Box<[char]> = vec![' '; rows * cols].into_boxed_slice();

        // Use a trick to create a perimeter around the grid, so we can index without bounds checks
        let mut row = 0;
        for line in lines {
            line.char_indices().for_each(|(col, val)| {
                elems[Self::helper_get_idx(col + 1, row + 1, cols)] = val;
            });
            row += 1;
        }

        //println!("{elems:?}");

        Grid { rows, cols, elems }
    }

    fn is_available(&self, col: usize, row: usize) -> bool {
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

    fn get_rolls(&self) -> usize {
        let mut available_rolls = 0;
        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                if self.is_available(col, row) {
                    available_rolls += 1;
                }
            }
        }

        println!("available_rolls: {available_rolls}");

        available_rolls
    }
}

#[test]
fn test_prelim() {
    let grid = Grid::create(&get_input("prelim.txt"));
    let rolls = grid.get_rolls();
    assert_eq!(rolls, 13);
}

#[test]
fn test_part1() {
    let grid = Grid::create(&get_input("input.txt"));
    let rolls = grid.get_rolls();
    assert_eq!(rolls, 1451);
}

fn main() {
    let grid = Grid::create(&get_input("prelim.txt"));
    grid.get_rolls();
    let grid = Grid::create(&get_input("input.txt"));
    grid.get_rolls();
}
