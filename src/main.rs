#![warn(clippy::all, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::shadow_reuse,
    clippy::print_stdout,
    clippy::wildcard_enum_match_arm,
    clippy::else_if_without_else,
    clippy::expect_used,
    clippy::integer_division,
    clippy::blanket_clippy_restriction_lints
)]

const GAMES: &str = include_str!("../answers.txt");

fn main() {
    let guesser = popoki::algorithms::Naive::new();
    for anwser in GAMES.split_whitespace() {
        popoki::play(anwser, guesser);
    }
}
