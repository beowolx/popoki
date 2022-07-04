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

use clap::{Parser, ValueEnum};
use popoki::Guesser;

mod algorithms;

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The type of algorithm implementation that the user wants to use
    #[clap(short, long, value_enum, default_value_t = Implementation::Naive)]
    implementation: Implementation,

    /// The number of games that we want run our algorithm implementation
    //TODO: Perhaps define a limit of games?
    #[clap(
        help = "The number of games that we want run our algorithm implementation",
        short,
        long,
        default_value_t = 5
    )]
    max: usize,
}

#[derive(Parser, ValueEnum, Clone)]
enum Implementation {
    Naive,
    Allocs,
}

fn main() {
    let args = Args::parse();
    match args.implementation {
        Implementation::Naive => play(algorithms::Naive::new, args.max),
        Implementation::Allocs => play(algorithms::Allocs::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: usize)
where
    G: Guesser,
{
    let w = popoki::Wordle::new();
    for answer in GAMES.split_whitespace().take(max) {
        let guesser = (mk)();

        if let Some(score) = w.play(answer, guesser) {
            println!("Guessed '{answer}' in {score}");
        } else {
            eprintln!("Failed to guess");
        }
    }
}
