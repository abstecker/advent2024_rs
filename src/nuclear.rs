use std::fmt::Error;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct LevelReports(Vec<LevelReport>);

impl LevelReports {
    pub fn new() -> Self {
        LevelReports(Vec::new())
    }

    pub fn push(&mut self, lr: &LevelReport) {
        self.0.push(lr.clone());
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn vec(&self) -> &Vec<LevelReport> {
        &self.0
    }

    pub fn get_safe(&self) -> LevelReports {
        let mut safe = LevelReports::new();

        for lr in &self.0 {
            if lr.is_safe() {
                safe.push(&lr);
            }
        }

        safe
    }

    pub fn get_unsafe(&self) -> LevelReports {
        let mut not_safe = LevelReports::new();

        for lr in &self.0 {
            if !lr.is_safe() {
                not_safe.push(&lr);
            }
        }

        not_safe
    }
}

impl Default for LevelReports {
    fn default() -> Self {
        let filename = "data/day02.txt";
        let mut lr = LevelReports::new();
        for line in read_to_string(filename).unwrap().lines() {
            match LevelReport::from_str(line) {
                Ok(report) => {
                    lr.push(&report);
                }
                Err(report) => {
                    println!("Unable to read in {report}");
                }
            }
        }

        lr
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
// pub struct Deck([Card; 52]);
pub struct LevelReport(Vec<usize>);

impl LevelReport {
    pub fn vec(&self) -> &Vec<usize> {
        &self.0
    }

    pub fn is_safe(&self) -> bool {
        self.is_gradual() && self.is_unidirectional()
    }

    pub fn is_gradual(&self) -> bool {
        for i in 0..self.vec().len() {
            let boundary = self.vec().len() - 1;
            if i < boundary {
                if !LevelReport::is_within_3(self.0[i], self.0[i + 1]) {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_within_3(x: usize, y: usize) -> bool {
        let mut higher = x;
        let mut lower = y;

        if x < y {
            higher = y;
            lower = x;
        }

        (higher - lower) < 4
    }

    pub fn is_lowering(&self) -> bool {
        for i in 0..self.vec().len() {
            let boundary = self.vec().len() - 1;
            if i < boundary {
                if self.0[i] <= self.0[i + 1] {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_rising(&self) -> bool {
        for i in 0..self.vec().len() {
            let boundary = self.vec().len() - 1;
            if i < boundary {
                if self.0[i] >= self.0[i + 1] {
                    return false;
                }
            }
        }

        true
    }

    pub fn is_unidirectional(&self) -> bool {
        self.is_rising() || self.is_lowering()
    }
}

impl FromStr for LevelReport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<&str>>();

        match v.len() {
            _ => {
                let mut ints: Vec<usize> = Vec::new();

                for i in v {
                    ints.push(i.parse::<usize>().unwrap());
                }

                Ok(LevelReport(ints))
            }
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod nuclear__test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("7 6 4 2 1")]
    #[case("1 2 3 4 5")]
    fn is_safe(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(lr.is_safe())
    }

    #[rstest]
    #[case("8 7 3 2 1")]
    #[case("1 3 2 4 9")]
    #[case("7 4 8 2 1")]
    #[case("1 2 3 8 3")]
    #[case("43 44 47 49 49")]
    fn is_safe__false(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(!lr.is_safe())
    }

    #[rstest]
    #[case("7 6 4 2 1")]
    #[case("1 3 2 4 5")]
    #[case("43 44 47 49 49")]
    fn is_gradual(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(lr.is_gradual())
    }

    #[rstest]
    #[case("8 7 3 2 1")]
    #[case("1 3 2 4 9")]
    fn is_gradual__false(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(!lr.is_gradual())
    }

    #[rstest]
    #[case("7 6 4 2 1")]
    #[case("1 2 3 8 9")]
    fn is_unidirectional(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(lr.is_unidirectional())
    }

    #[rstest]
    #[case("7 4 8 2 1")]
    #[case("1 2 3 8 3")]
    #[case("43 44 47 49 49")]
    fn is_unidirectional__false(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(!lr.is_unidirectional())
    }

    #[test]
    fn isolate() {
        let lr = LevelReport::from_str("95 93 91 90 90").unwrap();
        assert!(!lr.is_rising());
        assert!(!lr.is_lowering());
    }

    #[test]
    fn is_within_3() {
        assert!(LevelReport::is_within_3(1, 2));
        assert!(LevelReport::is_within_3(1, 1));
        assert!(LevelReport::is_within_3(2, 1));
        assert!(LevelReport::is_within_3(4, 6));
        assert!(LevelReport::is_within_3(4, 7));
    }

    #[test]
    fn from_str() {
        let input = "7 6 4 2 1";

        let expected = LevelReport(vec![7, 6, 4, 2, 1]);

        assert_eq!(expected, LevelReport::from_str(input).unwrap());
    }

    #[test]
    #[ignore]
    fn from_str__invalid() {
        assert!(LevelReport::from_str("7 6 4 2").is_err());
        assert!(LevelReport::from_str("7 6 4 2 1 3").is_err());
    }
}
