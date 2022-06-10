use popoki::{Guess, Guesser};

pub struct Naive;

impl Naive {
    pub const fn new() -> Self {
        Naive
    }
}

impl Default for Naive {
    fn default() -> Self {
        Self::new()
    }
}

impl Guesser for Naive {
    fn guess(&mut self, _history: &[Guess]) -> String {
        todo!();
    }
}
