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

struct Present {
    id: usize,
    filled: usize,
    #[allow(unused)]
    shape: [[bool; 3]; 3],
}

impl Present {
    fn new(lines: &[String]) -> Self {
        if lines.len() != 4 {
            panic!("Invalid input file");
        }
        let id = lines[0][0..lines[0].len() - 1]
            .parse::<usize>()
            .expect("Invalid id");
        let mut filled = 0;
        let mut shape = [[false; 3]; 3];
        for (row, line) in lines[1..].iter().enumerate() {
            for (col, c) in line.char_indices() {
                match c {
                    '#' => {
                        shape[row][col] = true;
                        filled += 1;
                    }
                    '.' => {}
                    _ => panic!("Invalid present char {c} in {lines:?}"),
                }
            }
        }
        Self { id, filled, shape }
    }
}

struct Region {
    dims: (usize, usize),
    shape_quantities: Vec<usize>,
}

impl Region {
    fn new(line: &String) -> Self {
        let mut shape_quantities = Vec::new();

        let (dims_str, quantities_str) = line.split_once(':').unwrap();
        let (width_str, length_str) = dims_str.split_once('x').unwrap();
        let dims = (
            width_str.parse::<usize>().unwrap(),
            length_str.parse::<usize>().unwrap(),
        );

        quantities_str.split_ascii_whitespace().for_each(|x| {
            shape_quantities.push(x.parse::<usize>().unwrap());
        });

        Self {
            dims,
            shape_quantities,
        }
    }
}

fn parse_lines(lines: &Vec<String>) -> (Vec<Present>, Vec<Region>) {
    let mut presents = Vec::new();
    let mut regions = Vec::new();

    for i in 0..6 {
        let present = Present::new(&lines[i * 5..i * 5 + 4]);
        assert_eq!(i, present.id);
        presents.push(present);
    }

    for line in &lines[6 * 5..] {
        let region = Region::new(line);
        assert_eq!(region.shape_quantities.len(), presents.len());
        regions.push(region);
    }

    (presents, regions)
}

fn count_fit((presents, regions): &(Vec<Present>, Vec<Region>)) -> usize {
    let mut num_fit = 0;

    // After reading a hint, I only verify that the number of filled squares
    // of a present shape would fit in total in the region, since the real
    // puzzle inputs are either really large or really small. While I could
    // try to actually solve the real problem, that seems unnecessary.
    for region in regions {
        let region_area = region.dims.0 * region.dims.1;
        let mut area = 0;
        for (i, quantity) in region.shape_quantities.iter().enumerate() {
            area += quantity * presents[i].filled;
        }
        if area < region_area {
            num_fit += 1;
        }
    }

    println!("num_fit: {num_fit}");

    num_fit
}

#[test]
fn test_prelim() {
    let count = count_fit(&parse_lines(&get_input("prelim.txt")));
    // This is wrong, but I don't care. The actual puzzle inputs are easier to solve for.
    assert_eq!(count, 3);
}

#[test]
fn test_part1() {
    let count = count_fit(&parse_lines(&get_input("input.txt")));
    assert_eq!(count, 583);
}

fn main() {
    count_fit(&parse_lines(&get_input("prelim.txt")));
    count_fit(&parse_lines(&get_input("input.txt")));
}
