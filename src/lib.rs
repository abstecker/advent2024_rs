use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Advent {
    v1: Vec<isize>,
    v2: Vec<isize>,
}

impl Advent {

    fn diff(u1: isize, u2: isize) -> isize {
        (u1 - u2).abs()
    }

    pub fn get_vecs() -> (Vec<isize>, Vec<isize>) {
        let filename = "data/day01.txt";

        let mut v1: Vec<isize> = Vec::new();
        let mut v2: Vec<isize> = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {

            let v = line.split_whitespace().collect::<Vec<&str>>();

            let u1 = v[0].parse::<isize>().unwrap();
            v1.push(u1);

            let u2 = v[1].parse::<isize>().unwrap();
            v2.push(u2);
        }

        v1.sort();
        v2.sort();

        (v1, v2)
    }

    pub fn simularity_map(v1: Vec<isize>, v2: Vec<isize>) -> HashMap<isize, usize> {
        let mut mappie: HashMap<isize, usize> = HashMap::new();

        for i in v1 {
            mappie.insert(i, 0);
        }

        for j in v2 {
            let old_count_option = mappie.get(&j);
            match old_count_option {
                None => {
                    println!("NOPE");
                    exit(1);
                }
                Some(o) => {
                    let new_count = 0 + 1;
                    mappie.insert(j, new_count);
                },
            };
        }

        mappie
    }
}


impl Default for Advent {
    fn default() -> Self {
        let filename = "data/day01.txt";

        let mut v1: Vec<isize> = Vec::new();
        let mut v2: Vec<isize> = Vec::new();

        for line in read_to_string(filename).unwrap().lines() {

            let v = line.split_whitespace().collect::<Vec<&str>>();

            let u1 = v[0].parse::<isize>().unwrap();
            v1.push(u1);

            let u2 = v[1].parse::<isize>().unwrap();
            v2.push(u2);
        }

        v1.sort();
        v2.sort();

        Advent {
            v1,
            v2,
        }
    }
}