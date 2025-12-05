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

#[derive(Debug)]
struct IngredientRange {
    start: usize,
    stop: usize,
}

impl IngredientRange {
    pub fn new(start: usize, stop: usize) -> Self {
        IngredientRange { start, stop }
    }

    pub fn parse_list(lines: &Vec<String>) -> (Vec<Self>, Vec<usize>) {
        let mut ranges = Vec::new();
        let mut ingredients = Vec::new();

        let mut iter = lines.split(|l| l.is_empty());
        let range_lines = iter.next().unwrap();
        let ingredient_lines = iter.next().unwrap();

        for line in range_lines {
            let (start, stop) = line.split_once('-').unwrap();
            let range = IngredientRange::new(
                start.parse::<usize>().unwrap(),
                stop.parse::<usize>().unwrap(),
            );
            ranges.push(range);
        }

        ingredient_lines.iter().for_each(|line| {
            ingredients.push(line.parse::<usize>().unwrap());
        });

        (ranges, ingredients)
    }
}

fn compute_fresh((ranges, ingredients): &(Vec<IngredientRange>, Vec<usize>)) -> usize {
    let mut fresh = 0;

    //println!("{ranges:?}");
    //println!("{ingredients:?}");
    for ingredient in ingredients {
        for range in ranges {
            if ingredient >= &range.start && ingredient <= &range.stop {
                fresh += 1;
                break;
            }
        }
    }

    println!("fresh: {fresh}");

    fresh
}

fn compute_fresh_ranges((ranges, ingredients): &(Vec<IngredientRange>, Vec<usize>)) -> usize {
    0
}

#[test]
fn test_prelim() {
    let fresh = compute_fresh(&IngredientRange::parse_list(&get_input("prelim.txt")));
    assert_eq!(fresh, 3);
}

#[test]
fn test_part1() {
    let fresh = compute_fresh(&IngredientRange::parse_list(&get_input("input.txt")));
    assert_eq!(fresh, 862);
}

fn main() {
    compute_fresh(&IngredientRange::parse_list(&get_input("prelim.txt")));
    compute_fresh(&IngredientRange::parse_list(&get_input("input.txt")));
    compute_fresh_ranges(&IngredientRange::parse_list(&get_input("prelim.txt")));
}
