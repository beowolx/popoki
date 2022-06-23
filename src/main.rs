#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(
    clippy::implicit_return,
    clippy::separated_literal_suffix,
    clippy::blanket_clippy_restriction_lints,
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::as_conversions,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::option_if_let_else
)]

mod algorithms;

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let w = popoki::Wordle::new();
    for answer in GAMES.split_whitespace() {
        let guesser = algorithms::Naive::new();

        if let Some(score) = w.play(answer, guesser) {
            println!("Guessed '{answer}' in {score}");
        } else {
            eprintln!("Failed to guess");
        }
    }
}
