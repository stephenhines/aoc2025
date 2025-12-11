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

    pub fn find_paths(&self) -> usize {
        let you_idx = self.name_map.get("you").unwrap();
        let out_idx = self.name_map.get("out").unwrap();
        let mut paths_to_out = HashMap::new();
        paths_to_out.insert(*out_idx, 1);
        let paths = self.find_paths_cache(&mut paths_to_out, *you_idx, *out_idx);

        println!("paths: {paths}");

        paths
    }
}

#[test]
fn test_prelim() {
    let paths = Servers::new(&get_input("prelim.txt")).find_paths();
    assert_eq!(paths, 5);
}

#[test]
fn test_part1() {
    let paths = Servers::new(&get_input("input.txt")).find_paths();
    assert_eq!(paths, 500);
}


fn main() {
    let servers = Servers::new(&get_input("prelim.txt"));
    println!("{servers:?}");
    servers.find_paths();
    let servers = Servers::new(&get_input("input.txt"));
    servers.find_paths();
}
