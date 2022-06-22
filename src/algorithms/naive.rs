use popoki::{Correctness, Guess, Guesser, DICTIONARY};
use std::collections::HashMap;

pub struct Naive {
    remaining: HashMap<&'static str, usize>,
}

#[allow(clippy::expect_used)]
impl Naive {
    pub fn new() -> Self {
        Self {
            remaining: DICTIONARY
                .lines()
                .map(|line| {
                    let (word, count) = line
                        .split_once(' ')
                        .expect("every line is word + space + frequency");
                    let count_parsed: usize = count.parse().expect("every count is a number");
                    (word, count_parsed)
                })
                .collect(),
        }
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
struct Candidate {
    word: &'static str,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }

        let remaining_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        let mut best: Option<Candidate> = None;
        for word in self.remaining.keys() {
            let mut sum = 0.0_f64;
            for pattern in Correctness::patterns() {
                // considering a world where we _did_ guess `word`and got `pattern`as the correctness.
                // now, compute what _then_ is left.
                let in_pattern_total: usize = 0;
                for (candidate, count) in &self.remaining {
                    let g = Guess {
                        word: (*word).to_owned(),
                        mask: pattern,
                    };
                    g.matches(candidate)
                        .then(|| in_pattern_total.saturating_add(*count));
                }
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                // - SUM_i p_i * log_2(p_i)
                sum += p_of_this_pattern * p_of_this_pattern.log2();
            }
            let goodness = 0.0_f64 - sum;
            if let Some(ref c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness });
                }
            } else {
                best = Some(Candidate { word, goodness });
            }
        }
        best.unwrap().word.to_owned()
    }
}
