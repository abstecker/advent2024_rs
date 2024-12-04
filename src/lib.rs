use std::collections::HashMap;
use std::fs::read_to_string;
use std::process::exit;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Advent {
    v1: Vec<isize>,
    v2: Vec<isize>,
}

impl Advent {

    pub fn get_distance(&self) -> isize {
        let mut distance: isize = 0;

        for i in 0..self.v1.len() {
            let u1 = self.v1[i];
            let u2 = self.v2[i];

            let diff: isize = (u1 - u2).abs();
            println!("{u1} - {u2} = {diff}");

            distance = distance + diff
        }
        distance
    }

    pub fn frequency(u1: isize) -> usize {
        let count: usize = 0;

        // for

        count
    }

    pub fn simularity_map(&self) -> HashMap<isize, usize> {
        let mut mappie: HashMap<isize, usize> = HashMap::new();

        for i in &self.v2 {
            mappie.insert(*i, 0);
        }

        for j in &self.v1 {
            let old_count_option = mappie.get(&j);
            match old_count_option {
                None => {
                    println!("NOPE: {j}");
                    // exit(1);
                }
                Some(o) => {
                    let new_count = 0 + 1;
                    mappie.insert(*j, new_count);
                }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_distance() {
        assert_eq!(3246517, Advent::default().get_distance());
    }
}