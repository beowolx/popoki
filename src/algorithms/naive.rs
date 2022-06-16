use popoki::{Guess, Guesser, DICTIONARY};
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
    count: usize,
    goodness: f64,
}

impl Guesser for Naive {
    fn guess(&mut self, history: &[Guess]) -> String {
        if let Some(last) = history.last() {
            self.remaining.retain(|word, _| last.matches(word));
        }
        let mut best: Option<Candidate> = None;
        //TOO: how do we compute goodness?
        let goodness = 0.0_f64;
        for (&word, &count) in &self.remaining {
            if let Some(ref c) = best {
                if goodness > c.goodness {
                    best = Some(Candidate {
                        word,
                        count,
                        goodness,
                    });
                }
            } else {
                best = Some(Candidate {
                    word,
                    count,
                    goodness,
                });
            }
        }
        best.unwrap().word.to_owned()
    }
}
