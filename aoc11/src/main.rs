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

#[derive(Debug)]
struct Servers {
    #[allow(unused)]
    names: Vec<String>,
    name_map: HashMap<String, usize>,
    connections: Vec<HashSet<usize>>,
}

impl Servers {
    pub fn new(lines: &Vec<String>) -> Self {
        let mut names = Vec::new();
        let mut name_map = HashMap::new();
        let mut connections = Vec::new();

        for line in lines {
            let toks = line.split_ascii_whitespace().collect::<Vec<_>>();
            let from_tok = &toks[0][..toks[0].len() - 1];
            if name_map.get(from_tok).is_none() {
                name_map.insert(from_tok.to_string(), names.len());
                names.push(from_tok.to_string());
                connections.push(HashSet::new());
            }
            let from_idx = *name_map.get(from_tok).unwrap();

            for &to_tok in &toks[1..] {
                if name_map.get(to_tok).is_none() {
                    name_map.insert(to_tok.to_string(), names.len());
                    names.push(to_tok.to_string());
                    connections.push(HashSet::new());
                }
                let to_idx = *name_map.get(to_tok).unwrap();
                connections[from_idx].insert(to_idx);
            }
        }

        Servers {
            names,
            name_map,
            connections,
        }
    }

    // Memoization is so easy in Rust (and other modern languages), so this
    // is just a helper function for caching the unique paths.
    pub fn find_paths_cache(
        &self,
        paths_to_out: &mut HashMap<usize, usize>,
        from_idx: usize,
        to_idx: usize,
    ) -> usize {
        let mut paths = 0;

        if paths_to_out.contains_key(&from_idx) {
            return *paths_to_out.get(&from_idx).unwrap();
        }

        for next in &self.connections[from_idx] {
            let p = self.find_paths_cache(paths_to_out, *next, to_idx);
            paths += p;
        }
        paths_to_out.insert(from_idx, paths);

        paths
    }

    fn find_paths(&self, from_idx: usize, to_idx: usize) -> usize {
        let mut paths_to_out = HashMap::new();
        paths_to_out.insert(to_idx, 1);
        self.find_paths_cache(&mut paths_to_out, from_idx, to_idx)
    }

    pub fn find_paths_you_to_out(&self) -> usize {
        let you_idx = *self.name_map.get("you").unwrap();
        let out_idx = *self.name_map.get("out").unwrap();
        let paths = self.find_paths(you_idx, out_idx);
        println!("paths: {paths}");
        paths
    }

    pub fn find_paths_svr_to_out(&self) -> usize {
        let svr_idx = *self.name_map.get("svr").unwrap();
        let out_idx = *self.name_map.get("out").unwrap();
        let dac_idx = *self.name_map.get("dac").unwrap();
        let fft_idx = *self.name_map.get("fft").unwrap();

        // We can be clever and just find paths from svr -> dac, dac -> fft,
        // and then fft -> out. Then we can do the same thing going to fft
        // first, then dac, and that will let us calculate the number of total
        // combinations of paths from svr -> out. It's great that my solution
        // for part 1 was trivially modified to handle these cases.
        let svr_dac = self.find_paths(svr_idx, dac_idx);
        let dac_fft = self.find_paths(dac_idx, fft_idx);
        let fft_out = self.find_paths(fft_idx, out_idx);

        let svr_fft = self.find_paths(svr_idx, fft_idx);
        let fft_dac = self.find_paths(fft_idx, dac_idx);
        let dac_out = self.find_paths(dac_idx, out_idx);

        let paths = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;

        println!("paths: {paths}");

        paths
    }
}

#[test]
fn test_prelim() {
    let paths = Servers::new(&get_input("prelim.txt")).find_paths_you_to_out();
    assert_eq!(paths, 5);
}

#[test]
fn test_part1() {
    let paths = Servers::new(&get_input("input.txt")).find_paths_you_to_out();
    assert_eq!(paths, 500);
}

#[test]
fn test_prelim2() {
    let paths = Servers::new(&get_input("prelim2.txt")).find_paths_svr_to_out();
    assert_eq!(paths, 2);
}

#[test]
fn test_part2() {
    let paths = Servers::new(&get_input("input.txt")).find_paths_svr_to_out();
    assert_eq!(paths, 287039700129600);
}

fn main() {
    let servers = Servers::new(&get_input("prelim.txt"));
    //println!("{servers:?}");
    servers.find_paths_you_to_out();

    let servers = Servers::new(&get_input("prelim2.txt"));
    servers.find_paths_svr_to_out();

    let servers = Servers::new(&get_input("input.txt"));
    servers.find_paths_you_to_out();
    servers.find_paths_svr_to_out();
}
