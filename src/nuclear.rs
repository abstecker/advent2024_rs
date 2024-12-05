use std::fmt::Error;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
// pub struct Deck([Card; 52]);
pub struct LevelReport([usize; 5]);

impl LevelReport {

    pub fn array(&self) -> &[usize; 5] {
        &self.0
    }

    pub fn is_safe(&self) -> bool {
        self.is_gradual() && self.is_unidirectional()
    }

    fn is_gradual(&self) -> bool {
        LevelReport::is_within_3(self.0[0], self.0[1]) &&
            LevelReport::is_within_3(self.0[1], self.0[2]) &&
            LevelReport::is_within_3(self.0[2], self.0[3]) &&
            LevelReport::is_within_3(self.0[3], self.0[4])
    }

    fn is_within_3(x: usize, y: usize) -> bool {
        let mut higher = x;
        let mut lower = y;

        if x < y {
            higher = y;
            lower = x;
        }

        (higher - lower) < 4
    }

    fn is_lowering(&self) -> bool {
        (self.0[0] < self.0[1]) &&
            (self.0[1] < self.0[2]) &&
            (self.0[2] < self.0[3]) &&
            (self.0[3] < self.0[4])
    }

    fn is_rising(&self) -> bool {
        (self.0[0] > self.0[1]) &&
        (self.0[1] > self.0[2]) &&
        (self.0[2] > self.0[3]) &&
        (self.0[3] > self.0[4])
    }

    fn is_unidirectional(&self) -> bool {
        self.is_rising() || self.is_lowering()
    }

    pub fn as_vec(&self) -> Vec<usize> {
        self.0.to_vec()
    }
}

impl FromStr for LevelReport {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<&str>>();

        if v.len() != 5 {
            Err(Error)
        } else {
            let level1 = v[0].parse::<usize>().unwrap();
            let level2 = v[1].parse::<usize>().unwrap();
            let level3 = v[2].parse::<usize>().unwrap();
            let level4 = v[3].parse::<usize>().unwrap();
            let level5 = v[4].parse::<usize>().unwrap();

            Ok(LevelReport([level1, level2, level3, level4, level5]))
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
    #[case("1 3 2 4 5")]
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
    fn is_unidirectional__false(#[case] input: &str) {
        let lr = LevelReport::from_str(input).unwrap();
        assert!(!lr.is_unidirectional())
    }

    #[test]
    fn is_within_3() {
        assert!(LevelReport::is_within_3(1,2));
        assert!(LevelReport::is_within_3(1,1));
        assert!(LevelReport::is_within_3(2,1));
        assert!(LevelReport::is_within_3(4,6));
        assert!(LevelReport::is_within_3(4,7));
    }

    #[test]
    fn from_str() {
        let input = "7 6 4 2 1";

        let expected = LevelReport([7, 6, 4, 2, 1]);

        assert_eq!(expected, LevelReport::from_str(input).unwrap());
    }

    #[test]
    fn from_str__invalid() {
        assert!(LevelReport::from_str("7 6 4 2").is_err());
        assert!(LevelReport::from_str("7 6 4 2 1 3").is_err());
    }
}
