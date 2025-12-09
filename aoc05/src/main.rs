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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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

    pub fn merge_ranges(ranges: &mut Vec<IngredientRange>) -> usize {
        ranges.sort();

        // This isn't the nicest way to merge these ranges, but it is still pretty fast
        let mut i = 0;
        while i < ranges.len() {
            let j = i + 1;
            if j < ranges.len() {
                let ri = &ranges[i];
                let rj = &ranges[j];
                if rj.start <= ri.stop {
                    if rj.stop > ri.stop {
                        ranges[i].stop = ranges[j].stop;
                    }
                    ranges.remove(j);
                    continue;
                }
             }
            i += 1;
        }
        //println!("ranges {ranges:?}");

        let mut sum = 0;
        for range in ranges {
            sum += 1 + range.stop - range.start;
        }

        println!("sum {sum}");
        sum
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

#[test]
fn test_prelim2() {
    let sum = IngredientRange::merge_ranges(&mut IngredientRange::parse_list(&get_input("prelim.txt")).0);
    assert_eq!(sum, 14);
}

#[test]
fn test_part2() {
    let sum = IngredientRange::merge_ranges(&mut IngredientRange::parse_list(&get_input("input.txt")).0);
    assert_eq!(sum, 357907198933892);
}

fn main() {
    compute_fresh(&IngredientRange::parse_list(&get_input("prelim.txt")));
    compute_fresh(&IngredientRange::parse_list(&get_input("input.txt")));
    IngredientRange::merge_ranges(&mut IngredientRange::parse_list(&get_input("prelim.txt")).0);
    IngredientRange::merge_ranges(&mut IngredientRange::parse_list(&get_input("input.txt")).0);
}
