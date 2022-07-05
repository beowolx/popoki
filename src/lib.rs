use std::{borrow::Cow, collections::HashSet};
pub const DICTIONARY: &str = include_str!("../dictionary.txt");

pub struct Wordle {
    dictionary: HashSet<&'static str>,
}

impl Wordle {
    pub fn new() -> Self {
        Self {
            dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
                line.split_once(' ')
                    .expect("every line is word + space + frequency")
                    .0
            })),
        }
    }

    pub fn play<G: Guesser>(&self, answer: &'static str, mut guesser: G) -> Option<usize> {
        let mut history = Vec::new();

        // Wordle only allows six guesses.
        // Popoki allows more to avoid chopping off the score distribution for stats
        // purposes.
        for i in 1..=32 {
            let guess = guesser.guess(&history);
            if guess == answer {
                return Some(i);
            }
            assert!(self.dictionary.contains(&*guess));
            let correctness = Correctness::compute(answer, &guess);
            history.push(Guess {
                word: Cow::Owned(guess),
                mask: correctness,
            });
        }
        None
    }
}

impl Default for Wordle {
    fn default() -> Self {
        Wordle::new()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        assert_eq!(answer.len(), 5);
        assert_eq!(guess.len(), 5);
        let mut c = [Correctness::Wrong; 5];

        // Mark things green
        for (i, item) in c.iter_mut().enumerate() {
            if answer.as_bytes().get(i) == guess.as_bytes().get(i) {
                *item = Correctness::Correct;
            }
        }

        let mut used = [false; 5];
        for (i, c) in c.iter().enumerate() {
            if *c == Correctness::Correct {
                used[i] = true;
            }
        }

        // Mark things yellow
        for (i, g) in guess.bytes().enumerate() {
            if c[i] == Correctness::Correct {
                // Already marked as green
                continue;
            }
            if answer.bytes().enumerate().any(|(i, a)| {
                if a == g && !used[i] {
                    used[i] = true;
                    return true;
                }
                false
            }) {
                c[i] = Correctness::Misplaced;
            }
        }
        c
    }

    pub fn patterns() -> impl Iterator<Item = [Self; 5]> {
        itertools::iproduct!(
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong],
            [Self::Correct, Self::Misplaced, Self::Wrong]
        )
        .map(|(a, b, c, d, e)| [a, b, c, d, e])
    }
}

pub struct Guess<'a> {
    pub word: Cow<'a, str>,
    pub mask: [Correctness; 5],
}

impl Guess<'_> {
    pub fn matches(&self, word: &str) -> bool {
        // If guess G gives mask C against answer A, then guess A should give
        // mask C against answer G
        Correctness::compute(word, &self.word) == self.mask
    }
}

pub trait Guesser {
    fn guess(&mut self, history: &[Guess]) -> String;
}

#[cfg(test)]
macro_rules! guesser {
    (|$history:ident| $impl:block) => {{
        struct G;
        impl $crate::Guesser for G {
            fn guess(&mut self, $history: &[Guess]) -> String {
                $impl
            }
        }
        G
    }};
}
#[cfg(test)]
macro_rules! mask {
    (C) => {$crate::Correctness::Correct};
    (M) => {$crate::Correctness::Misplaced};
    (W) => {$crate::Correctness::Wrong};
    ($($c:tt)+) => {[
        $(mask!($c)),+
    ]}
}

#[cfg(test)]
mod tests {
    mod guess_matcher {
        use crate::Guess;
        use std::borrow::Cow;

        macro_rules! check {
            ($prev:literal + [$($mask:tt)+] allows $next:literal) => {
                assert!(Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }.matches($next));
                assert_eq!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            };
            ($prev:literal + [$($mask:tt)+] disallows $next:literal) => {
                assert!(!Guess {
                    word: Cow::Borrowed($prev),
                    mask: mask![$($mask )+]
                }.matches($next));
                assert_ne!($crate::Correctness::compute($next, $prev), mask![$($mask )+]);
            }
        }

        #[test]
        fn matches() {
            // Look how we can simplify one assertion using macros!
            // assert!(Guess {
            //     word: "abcde".to_string(),
            //     mask: mask![C C C C C]
            // }
            // .matches("abcde"));

            check!("abcde" + [C C C C C] allows "abcde");
            check!("abcde" + [W W W W W] allows "fghij");
            check!("abcde" + [M M M M M] allows "eabcd");
            check!("baaaa" + [W C M W W] allows "aaccc");

            check!("abcde" + [W W W W W] disallows "bcdea");
            check!("aaabb" + [C M W W W] disallows "accaa");
            check!("abcdf" + [C C C C C] disallows "abcde");
            check!("baaaa" + [W C M W W] disallows "caacc");

            check!("tares" + [W M M W W] disallows "brink");
        }
    }
    mod game {
        use crate::{Guess, Wordle};
        #[test]
        fn genius() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "right".to_owned() });
            assert_eq!(w.play("right", guesser), Some(1));
        }

        #[test]
        fn magnificent() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 1 {
                    return "right".to_owned();
                }
                "wrong".to_owned()
            });
            assert_eq!(w.play("right", guesser), Some(2));
        }

        #[test]
        fn impressive() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 2 {
                    return "right".to_owned();
                }
                "wrong".to_owned()
            });
            assert_eq!(w.play("right", guesser), Some(3));
        }

        #[test]
        fn splendid() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 3 {
                    return "right".to_owned();
                }
                "wrong".to_owned()
            });
            assert_eq!(w.play("right", guesser), Some(4));
        }

        #[test]
        fn great() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 4 {
                    return "right".to_owned();
                }
                "wrong".to_owned()
            });
            assert_eq!(w.play("right", guesser), Some(5));
        }

        #[test]
        fn phew() {
            let w = Wordle::new();
            let guesser = guesser!(|history| {
                if history.len() == 5 {
                    return "right".to_owned();
                }
                "wrong".to_owned()
            });
            assert_eq!(w.play("right", guesser), Some(6));
        }

        #[test]
        fn oops() {
            let w = Wordle::new();
            let guesser = guesser!(|_history| { "wrong".to_owned() });
            assert_eq!(w.play("right", guesser), None);
        }
    }
    mod compute {
        use crate::Correctness;

        #[test]
        fn all_green() {
            assert_eq!(Correctness::compute("abcde", "abcde"), mask![C C C C C]);
        }

        #[test]
        fn all_gray() {
            assert_eq!(Correctness::compute("abcde", "fghij"), mask![W W W W W]);
        }

        #[test]
        fn all_yellow() {
            assert_eq!(Correctness::compute("abcde", "eabcd"), mask![M M M M M]);
        }

        #[test]
        fn repeat_green() {
            assert_eq!(Correctness::compute("aabbb", "aaccc"), mask![C C W W W]);
        }

        #[test]
        fn repeat_yellow() {
            assert_eq!(Correctness::compute("aabbb", "ccaac"), mask![W W M M W]);
        }

        #[test]
        fn repeat_some_green() {
            assert_eq!(Correctness::compute("aabbb", "caacc"), mask![W C M W W]);
        }

        #[test]
        fn only_one_yellow() {
            assert_eq!(Correctness::compute("azzaz", "aaabb"), mask![C M W W W]);
        }

        #[test]
        fn only_one_green() {
            assert_eq!(Correctness::compute("baccc", "aaddd"), mask![W C W W W]);
        }

        #[test]
        fn only_one_gray() {
            assert_eq!(Correctness::compute("abcde", "aacde"), mask![C W C C C]);
        }
    }
}
