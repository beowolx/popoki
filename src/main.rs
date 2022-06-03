#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    for answer in GAMES.split_whitespace() {
        let guesser = popoki::algorithms::Naive::new();
        popoki::play(answer, guesser);
    }
}
