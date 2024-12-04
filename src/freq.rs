use std::collections::{HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct XIncrement {
    pub x: isize,
    pub increment: usize,
}

impl XIncrement {
    pub fn new(x: isize) -> Self {
        Self { x, increment: 0 }
    }

    pub fn plus_one(&self) -> Self {
        XIncrement {
            x: self.x,
            increment: self.increment + 1,
        }
    }

    pub fn times(&self) -> isize {
        self.x * self.increment as isize
    }

    pub fn process_map(mappie: &FrequencyMap) -> Vec<XIncrement> {
        let mut v: Vec<XIncrement> = Vec::new();

        for (k, j) in mappie.map() {
            let xi = XIncrement {
                x: *k,
                increment: *j,
            };
            v.push(xi);
        }

        v.sort();

        v
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrequencyMap(HashMap<isize, usize>);

impl FrequencyMap {
    pub fn init(v: &Vec<isize>) -> Self {
        let mut mappie = FrequencyMap::default();

        for i in v {
            mappie.inc(*i);
        }

        mappie
    }

    pub fn get_frequency(&self, x: isize) -> usize {
        let mut count = 0;

        if self.0.contains_key(&x) {
            count = *self.0.get(&x).unwrap();
        }

        count
    }

    pub fn inc(&mut self, x: isize) -> bool {
        let mut found = false;

        if self.0.contains_key(&x) {
            let count = self.0.get_mut(&x).unwrap();
            *count += 1;
            found = true;
        } else {
            self.0.insert(x, 1);
        }

        found
    }

    pub fn map(&self) -> &HashMap<isize, usize> {
        &self.0
    }
}

impl Default for FrequencyMap {
    fn default() -> Self {
        FrequencyMap(HashMap::new())
    }
}
