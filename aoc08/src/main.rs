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

struct CircuitSet {
    circuits: Vec<HashSet<usize>>,
}

impl CircuitSet {
    fn create(n: usize) -> Self {
        let mut circuits = Vec::new();
        for i in 0..n {
            let mut s: HashSet<usize> = HashSet::new();
            s.insert(i);
            circuits.push(s);
        }

        CircuitSet { circuits }
    }

    fn connect_boxes(&mut self, a: usize, b: usize) {
        let mut aset = (false, 0);
        let mut bset = (false, 0);
        for (ci, c) in self.circuits.iter().enumerate() {
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
                    let mut new_set = self.circuits[aset.1].clone();
                    new_set.extend(&self.circuits[bset.1]);
                    self.circuits[aset.1] = new_set;
                    self.circuits.remove(bset.1);
                    //println!("circuits[{}]: {:?}", self.circuits.len(), self.circuits);
                }
                break;
            }
        }
    }

    fn sort(&mut self) {
        self.circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    }
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

struct BoxGrid {
    boxes: Vec<JunctionBox>,
    distances: Vec<(f32, usize, usize)>,
}

impl BoxGrid {
    fn create(lines: &Vec<String>) -> Self {
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

        BoxGrid { boxes, distances }
    }

    fn circuit_trio_product(&self, connections: usize) -> usize {
        assert!(connections > 0);

        let mut circuitset = CircuitSet::create(self.boxes.len());

        for n in 0..connections {
            let (_, a, b) = self.distances[n];
            circuitset.connect_boxes(a, b);
        }

        circuitset.sort();

        // Product of the trio of largest circuits
        assert!(circuitset.circuits.len() >= 3);
        let mut product = 1;
        for i in 0..3 {
            product *= circuitset.circuits[i].len();
        }

        println!("product: {product}");
        product
    }

    fn circuit_last_x_product(&self) -> i64 {
        let mut circuitset = CircuitSet::create(self.boxes.len());

        for (_, a, b) in &self.distances {
            circuitset.connect_boxes(*a, *b);
            if circuitset.circuits.len() == 1 {
                let product = &self.boxes[*a].x * &self.boxes[*b].x;
                println!("product (last 2 x): {product}");
                return product;
            }
        }
        unreachable!("Failed to converge all junction boxes");
    }
}

#[test]
fn test_prelim() {
    let boxgrid = BoxGrid::create(&get_input("prelim.txt"));
    let product = boxgrid.circuit_trio_product(10);
    assert_eq!(product, 40);
}

#[test]
fn test_part1() {
    let boxgrid = BoxGrid::create(&get_input("input.txt"));
    let product = boxgrid.circuit_trio_product(1000);
    assert_eq!(product, 81536);
}

#[test]
fn test_prelim2() {
    let boxgrid = BoxGrid::create(&get_input("prelim.txt"));
    let product = boxgrid.circuit_last_x_product();
    assert_eq!(product, 25272);
}

#[test]
fn test_part2() {
    let boxgrid = BoxGrid::create(&get_input("input.txt"));
    let product = boxgrid.circuit_last_x_product();
    assert_eq!(product, 7017750530);
}

fn main() {
    let boxgrid = BoxGrid::create(&get_input("prelim.txt"));
    boxgrid.circuit_trio_product(10);
    boxgrid.circuit_last_x_product();
    let boxgrid = BoxGrid::create(&get_input("input.txt"));
    boxgrid.circuit_trio_product(10);
    boxgrid.circuit_last_x_product();
}
