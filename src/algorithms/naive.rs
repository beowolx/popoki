use popoki::{Correctness, Guess, Guesser, Word, DICTIONARY};
use std::{borrow::Cow, collections::HashMap};

pub struct Naive {
    remaining: HashMap<&'static Word, usize>,
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
                    let word_bytes = word
                        .as_bytes()
                        .try_into()
                        .expect("every dictionary word is 5 characters");
                    (word_bytes, count_parsed)
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
    word: &'static Word,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> Word {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }
        if history.is_empty() {
            return *b"tares";
        }

        let remaining_count: usize = self.remaining.iter().map(|(_, &c)| c).sum();

        let mut best: Option<Candidate> = None;
        for word in self.remaining.keys() {
            let mut sum = 0.0_f64;
            for pattern in Correctness::patterns() {
                let mut in_pattern_total: usize = 0;
                for (candidate, count) in &self.remaining {
                    let g = Guess {
                        word: Cow::Owned((*word).to_owned()),
                        mask: pattern,
                    };
                    g.matches(candidate)
                        .then(|| in_pattern_total = in_pattern_total.saturating_add(*count));
                }
                if in_pattern_total == 0 {
                    continue;
                }
                // TODO: apply sigmoid
                let p_of_this_pattern = in_pattern_total as f64 / remaining_count as f64;
                sum += p_of_this_pattern * p_of_this_pattern.log2();
            }
            let goodness = -sum;
            if let Some(c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate { word, goodness });
                }
            } else {
                best = Some(Candidate { word, goodness });
            }
        }

        // Return the best guess OR a default value (it shouldn't never happen though)
        best.map_or(*b"cigar", |c| *c.word)
    }
}
