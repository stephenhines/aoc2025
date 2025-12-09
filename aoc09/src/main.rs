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

struct MovieTheater {
    tiles: Vec<(usize, usize)>,
}

impl MovieTheater {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut tiles = Vec::new();

        for line in lines {
            let toks = line.split(',').collect::<Vec<_>>();
            assert_eq!(toks.len(), 2);
            let x = toks[0].parse::<usize>().unwrap();
            let y = toks[1].parse::<usize>().unwrap();
            tiles.push((x, y));
        }

        MovieTheater { tiles }
    }

    fn get_area(&self, i: usize, j: usize) -> usize {
        let ci = self.tiles[i];
        let cj = self.tiles[j];

        let dist_x = if ci.0 > cj.0 {
            ci.0 - cj.0
        } else {
            cj.0 - ci.0
        } + 1;

        let dist_y = if ci.1 > cj.1 {
            ci.1 - cj.1
        } else {
            cj.1 - ci.1
        } + 1;

        dist_x * dist_y
    }

    pub fn largest_rect_area(&self) -> usize {
        let mut max_area = 0;

        for i in 0..self.tiles.len() {
            for j in i..self.tiles.len() {
                let area = self.get_area(i, j);
                if area > max_area {
                    max_area = area;
                }
            }
        }

        println!("max_area: {max_area}");

        max_area
    }
}

#[test]
fn test_prelim() {
    let area = MovieTheater::new(&get_input("prelim.txt")).largest_rect_area();
    assert_eq!(area, 50);
}

#[test]
fn test_part1() {
    let area = MovieTheater::new(&get_input("input.txt")).largest_rect_area();
    assert_eq!(area, 4740155680);
}

fn main() {
    let theater = MovieTheater::new(&get_input("prelim.txt"));
    theater.largest_rect_area();
    let theater = MovieTheater::new(&get_input("input.txt"));
    theater.largest_rect_area();
}
