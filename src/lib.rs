pub mod algorithms;

/// It takes a guesser and an answer, and returns an `Option` with the number of guesses it took to
/// guess the answer
pub fn play<G: Guesser>(answer: &'static str, mut guesser: G) -> Option<usize> {
    let mut history = Vec::new();

    // Wordle only allows six guesses.
    // Popoki allows more to avoid chopping off the score distribution for stats
    // purposes.
    for i in 1..32 {
        let guess = guesser.guess(&history);
        if guess == answer {
            return Some(i);
        }
        let correctness = Correctness::compute(answer, &guess);
        history.push(Guess {
            word: guess,
            mask: correctness,
        });
    }
    None
}

pub enum Correctness {
    // Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Wrong,
}

impl Correctness {
    /// Given an answer and a guess, return an array of 5 elements, each of which is
    /// a `Result` indicating whether the guess is correct, incorrect, or not present
    pub fn compute(answer: &str, guess: &str) -> [Self; 5] {
        todo!()
    }
}

pub struct Guess {
    pub word: String,
    pub mask: [Correctness; 5],
}

pub trait Guesser {
    /// A trait method that takes a mutable reference to self and a reference to a
    /// slice of Guesses and returns a String.
    fn guess(&mut self, history: &[Guess]) -> String;
}
