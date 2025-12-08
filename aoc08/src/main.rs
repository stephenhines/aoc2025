use core::f32;
use std::collections::HashSet;
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

struct JunctionBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JunctionBox {
    fn dist(&self, other: &JunctionBox) -> f32 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32)
            .sqrt()
    }
}

fn read_junctions(lines: &Vec<String>) -> Vec<JunctionBox> {
    let mut boxes = Vec::new();

    for line in lines {
        let coords = line.split(',').collect::<Vec<_>>();
        assert_eq!(coords.len(), 3);
        let (x, y, z) = (
            coords[0].parse::<i64>().unwrap(),
            coords[1].parse::<i64>().unwrap(),
            coords[2].parse::<i64>().unwrap(),
        );
        boxes.push(JunctionBox { x, y, z });
    }

    boxes
}

fn compute_circuits(boxes: &Vec<JunctionBox>, connections: usize, num_circuits: usize) -> usize {
    let mut product = 1;
    assert!(connections > 0);
    assert!(num_circuits > 0);

    let entries = boxes.len();
    let mut distances = Vec::new();
    for i in 0..entries {
        for j in i + 1..entries {
            let b = &boxes[i];
            let b2 = &boxes[j];
            distances.push((b.dist(b2), i, j));
        }
    }
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut circuits = Vec::new();
    for i in 0..entries {
        let mut s: HashSet<usize> = HashSet::new();
        s.insert(i);
        circuits.push(s);
    }

    // Use n to walk throught the sorted (least to greatest) distances
    let mut n = 0;
    while n < connections {
        let (_, a, b) = distances[n];
        let mut aset = (false, 0);
        let mut bset = (false, 0);
        for (ci, c) in circuits.iter().enumerate() {
            if c.contains(&a) {
                aset = (true, ci);
            }
            if c.contains(&b) {
                bset = (true, ci);
            }
            if aset.0 && bset.0 {
                if aset.1 == bset.1 {
                    // They're already connected
                } else {
                    // Not the most efficient use of HashSet union/extend, but it works
                    let mut new_set = circuits[aset.1].clone();
                    new_set.extend(&circuits[bset.1]);
                    circuits[aset.1] = new_set;
                    circuits.remove(bset.1);
                    //println!("circuits[{}]: {circuits:?}", circuits.len());
                }
                break;
            }
        }
        n += 1;
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    //println!("sorted circuits[{}]: {circuits:?}", circuits.len());

    for i in 0..num_circuits {
        product *= circuits[i].len();
    }

    println!("product: {product}");
    product
}

#[test]
fn test_prelim() {
    let boxes = read_junctions(&get_input("prelim.txt"));
    let product = compute_circuits(&boxes, 10, 3);
    assert_eq!(product, 40);
}

#[test]
fn test_part1() {
    let boxes = read_junctions(&get_input("input.txt"));
    let product = compute_circuits(&boxes, 1000, 3);
    assert_eq!(product, 81536);
}

fn main() {
    let boxes = read_junctions(&get_input("prelim.txt"));
    compute_circuits(&boxes, 10, 3);
    let boxes = read_junctions(&get_input("input.txt"));
    compute_circuits(&boxes, 1000, 3);
}
