use std::cmp;
use std::collections::HashMap;
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

// Compressed map of x and y values
struct CompMap {
    x_map: HashMap<usize, usize>,
    y_map: HashMap<usize, usize>,
    x_vals: Vec<usize>,
    y_vals: Vec<usize>,
}

impl CompMap {
    fn new(tiles: &Vec<(usize, usize)>) -> Self {
        let mut dim_x = 0;
        let mut dim_y = 0;

        let mut x_set = HashSet::new();
        let mut y_set = HashSet::new();

        for &(x, y) in tiles {
            x_set.insert(x);
            y_set.insert(y);
            dim_x = cmp::max(x, dim_x);
            dim_y = cmp::max(y, dim_y);
        }

        let mut x_vals = x_set.iter().copied().collect::<Vec<_>>();
        x_vals.push(0);
        x_vals.push(usize::MAX);
        x_vals.sort();
        let mut x_map = HashMap::new();
        for (i, x) in x_vals.iter().copied().enumerate() {
            x_map.insert(x, i);
        }

        let mut y_vals = y_set.iter().copied().collect::<Vec<_>>();
        y_vals.push(0);
        y_vals.push(usize::MAX);
        y_vals.sort();
        let mut y_map = HashMap::new();
        for (i, y) in y_vals.iter().copied().enumerate() {
            y_map.insert(y, i);
        }

        assert_eq!(x_vals.len(), x_map.len());
        assert_eq!(y_vals.len(), y_map.len());

        CompMap {
            x_map,
            y_map,
            x_vals,
            y_vals,
        }
    }

    fn compress(&self, (x, y): (usize, usize)) -> (usize, usize) {
        (*self.x_map.get(&x).unwrap(), *self.y_map.get(&y).unwrap())
    }
}

struct MovieTheater {
    dim_x: usize,
    dim_y: usize,
    tiles: Vec<(usize, usize)>,
    ctiles: Box<[char]>,
    cmap: CompMap,
}

impl MovieTheater {
    fn parse_lines(lines: &Vec<String>) -> Vec<(usize, usize)> {
        let mut tiles = Vec::new();

        for line in lines {
            let toks = line.split(',').collect::<Vec<_>>();
            assert_eq!(toks.len(), 2);
            let x = toks[0].parse::<usize>().unwrap();
            let y = toks[1].parse::<usize>().unwrap();
            tiles.push((x, y));
        }

        tiles
    }

    pub fn new(lines: &Vec<String>) -> Self {
        let tiles = Self::parse_lines(lines);
        let cmap = CompMap::new(&tiles);
        let dim_x = cmap.x_vals.len();
        let dim_y = cmap.y_vals.len();

        let mut ctiles: Box<[char]> = vec!['.'; dim_x * dim_y].into_boxed_slice();
        let (x0, y0) = cmap.compress(tiles[0]);
        ctiles[y0 * dim_x + x0] = '#';
        let mut x_prev = x0;
        let mut y_prev = y0;
        for t in &tiles {
            let (x, y) = cmap.compress(*t);
            ctiles[y * dim_x + x] = '#';
            if x == x_prev {
                let y_min = cmp::min(y, y_prev);
                let y_max = cmp::max(y, y_prev);
                for y in y_min + 1..y_max {
                    ctiles[y * dim_x + x] = 'X';
                }
            } else if y == y_prev {
                let x_min = cmp::min(x, x_prev);
                let x_max = cmp::max(x, x_prev);
                for x in x_min + 1..x_max {
                    ctiles[y * dim_x + x] = 'X';
                }
            } else {
                panic!("Invalid non-rectangular coordinate ({x}, {y})");
            }
            x_prev = x;
            y_prev = y;
        }
        if x0 == x_prev {
            let y_min = cmp::min(y0, y_prev);
            let y_max = cmp::max(y0, y_prev);
            for y in y_min + 1..y_max {
                ctiles[y * dim_x + x0] = 'X';
            }
        } else if y0 == y_prev {
            let x_min = cmp::min(x0, x_prev);
            let x_max = cmp::max(x0, x_prev);
            for x in x_min + 1..x_max {
                ctiles[y0 * dim_x + x] = 'X';
            }
        } else {
            panic!("Invalid non-rectangular final coordinate ({x0}, {y0})");
        }

        // Now we have to flood fill the interior.
        //
        // I'm using a dumb trick here to find the top left corner.
        // This is definitive, because we know we have only horizontal and
        // vertical edges, and this is the first corner. It must connect
        // both straight down from here, and directly to the right here.
        // Thus we can start our flood fill directly from this point that
        // is directly to the lower right of the top left corner.
        let mut work_queue = Vec::new();
        'outer: for y in 0..dim_y {
            for x in 0..dim_x {
                let idx = y * dim_x + x;
                if ctiles[idx] == '#' {
                    let start_flood = (x + 1, y + 1);
                    work_queue.push(start_flood);
                    break 'outer;
                }
            }
        }

        while !work_queue.is_empty() {
            let (x, y) = work_queue.pop().unwrap();
            let idx = y * dim_x + x;
            if ctiles[idx] == '.' {
                ctiles[idx] = 'X';
                // Add Up, Down, Left, Right to work queue
                work_queue.push((x, y - 1));
                work_queue.push((x, y + 1));
                work_queue.push((x - 1, y));
                work_queue.push((x + 1, y));
            }
        }

        MovieTheater {
            dim_x,
            dim_y,
            tiles,
            ctiles,
            cmap,
        }
    }

    fn get_area(&self, i: usize, j: usize) -> usize {
        let ci = self.tiles[i];
        let cj = self.tiles[j];

        let dist_x = ci.0.abs_diff(cj.0) + 1;
        let dist_y = ci.1.abs_diff(cj.1) + 1;

        dist_x * dist_y
    }

    pub fn largest_rect_area(&self) -> usize {
        let mut max_area = 0;

        for i in 0..self.tiles.len() {
            for j in i..self.tiles.len() {
                let area = self.get_area(i, j);
                max_area = cmp::max(max_area, area);
            }
        }

        println!("max_area: {max_area}");

        max_area
    }

    pub fn red_green_area(&self) -> usize {
        let mut areas = Vec::new();
        for i in 0..self.tiles.len() {
            for j in i..self.tiles.len() {
                let area = self.get_area(i, j);
                areas.push((area, i, j));
            }
        }

        areas.sort_by(|a, b| b.cmp(a));
        for (area, i, j) in areas {
            // Verify that it works by walking all the inner box area
            let (xi, yi) = self.cmap.compress(self.tiles[i]);
            let (xj, yj) = self.cmap.compress(self.tiles[j]);
            let x_min = cmp::min(xi, xj);
            let x_max = cmp::max(xi, xj);
            let y_min = cmp::min(yi, yj);
            let y_max = cmp::max(yi, yj);
            let mut valid = true;

            'outer: for y in y_min..y_max + 1 {
                for x in x_min..x_max + 1 {
                    let idx = y * self.dim_x + x;
                    match self.ctiles[idx] {
                        'X' | '#' => {}
                        '.' => {
                            valid = false;
                            break 'outer;
                        }
                        c => {
                            panic!("Invalid symbol {c}");
                        }
                    }
                }
            }
            if valid {
                println!("max_rg_area: {area}");
                return area;
            }
        }
        panic!("Couldn't find a valid rectangle");
    }

    #[allow(unused)]
    fn print(&self) {
        println!("dim_x: {}", self.dim_x);
        println!("dim_y: {}", self.dim_y);

        print!("   ");
        for x in 0..self.dim_x {
            print!("{}", x % 10);
        }
        println!("");
        for y in 0..self.dim_y {
            print!(" {} ", y % 10);
            for x in 0..self.dim_x {
                print!("{}", self.ctiles[y * self.dim_x + x]);
            }
            println!("");
        }
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

#[test]
fn test_prelim2() {
    let area = MovieTheater::new(&get_input("prelim.txt")).red_green_area();
    assert_eq!(area, 24);
}

#[test]
fn test_part2() {
    let area = MovieTheater::new(&get_input("input.txt")).red_green_area();
    assert_eq!(area, 1543501936);
}

fn main() {
    let theater = MovieTheater::new(&get_input("prelim.txt"));
    theater.largest_rect_area();
    theater.red_green_area();
    let theater = MovieTheater::new(&get_input("input.txt"));
    theater.largest_rect_area();
    theater.red_green_area();
}
