#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::implicit_return)]

mod algorithms;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = popoki::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = algorithms::Naive::new();
        w.play(answer, guesser);
    }
}
