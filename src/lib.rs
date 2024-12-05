pub mod freq;
pub mod nuclear;

use crate::freq::{FrequencyMap, XIncrement};
use std::fs::read_to_string;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Advent {
    pub v1: Vec<isize>,
    pub v2: Vec<isize>,
}

impl Advent {
    pub fn get_distance(&self) -> isize {
        let mut distance: isize = 0;

        for i in 0..self.v1.len() {
            let u1 = self.v1[i];
            let u2 = self.v2[i];

            let diff: isize = (u1 - u2).abs();
            // println!("{u1} - {u2} = {diff}");

            distance = distance + diff
        }
        distance
    }

    pub fn get_similarity_score(&self) -> isize {
        let mapped = self.map_right_frequency();

        let mut score: isize = 0;

        for m in mapped.iter() {
            score = score + m.times();
        }

        score
    }

    pub fn map_right_frequency(&self) -> Vec<XIncrement> {
        let mappie = FrequencyMap::init(&self.v2);

        let mut xis: Vec<XIncrement> = Vec::new();

        for i in &self.v1 {
            let xi = XIncrement {
                x: *i,
                increment: mappie.get_frequency(*i),
            };
            xis.push(xi);
        }

        xis
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

        Advent { v1, v2 }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;
    use crate::freq::{FrequencyMap, XIncrement};

    #[test]
    fn get_distance() {
        assert_eq!(3246517, Advent::default().get_distance());
    }

    #[test]
    fn get_similarity_score() {
        assert_eq!(29379307, Advent::default().get_similarity_score());
    }

    #[test]
    fn frequency__v1() {
        let advent = Advent::default();

        let mappie = FrequencyMap::init(&advent.v1);
        let pm = XIncrement::process_map(&mappie);

        assert_eq!(1000, mappie.map().clone().into_keys().len());
        assert_eq!(1000, pm.len());
    }

    #[test]
    fn frequency__v2() {
        let advent = Advent::default();

        let mappie = FrequencyMap::init(&advent.v2);
        let pm = XIncrement::process_map(&mappie);

        assert_eq!(552, mappie.map().clone().into_keys().len());
        assert_eq!(552, pm.len());
    }

    #[test]
    fn get_frequency__40092() {
        let advent = Advent::default();

        let mappie = FrequencyMap::init(&advent.v2);

        assert_eq!(0, mappie.get_frequency(76569));
        assert_eq!(20, mappie.get_frequency(40092));
    }

    #[test]
    fn map_right_frequency() {
        let advent = Advent::default();

        let mapped = advent.map_right_frequency();

        assert_eq!(1000, mapped.len());
    }
}
