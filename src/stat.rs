use serde::Serialize;

#[derive(Serialize)]
#[derive(Clone, Debug)]
pub struct Stat {
    questions: i32,
    words: i32,
}

impl Stat {
    // Create new Stat with questions and words.
    pub fn new(q: i32, w: i32) -> Self {
        Self { questions: q, words: w }
    }
    // Combine two Stat instances by summing it's questions and words.
    pub fn sum(&mut self, stat: &Stat) {
        self.questions += stat.questions;
        self.words += stat.words;
    }
    // Get the ratio of the current Stat instance.
    pub fn get_ratio(&self) -> f64 {
        Stat::calculate_ratio(&self.questions, &self.words)
    }
    // Calculate words/questions relation.
    pub fn calculate_ratio(questions: &i32, words: &i32,) -> f64 {
        *words as f64 / *questions as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_new() -> Result<(), String> {
        let stat = Stat::new(1, 1);

        assert!(stat.questions == 1 && stat.words == 1);
        Ok(())
    }
    #[test]
    fn test_stat_sum() -> Result<(), String> {
        let mut stat = Stat::new(1, 1);
        let stat2 = Stat::new(1, 1);
        stat.sum(&stat2);

        assert!(stat.questions == 2 && stat.words == 2);
        Ok(())
    }
    #[test]
    fn test_stat_get_ratio() -> Result<(), String> {
        let stat = Stat::new(2, 5);

        assert_eq!(Stat::get_ratio(&stat), 2.5);
        Ok(())
    }
}