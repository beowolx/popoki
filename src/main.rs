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
    clippy::option_if_let_else,
    clippy::expect_used
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
    Vecrem,
    Once,
    Weight,
    Prune,
}

fn main() {
    let args = Args::parse();
    match args.implementation {
        Implementation::Naive => play(algorithms::Naive::new, args.max),
        Implementation::Allocs => play(algorithms::Allocs::new, args.max),
        Implementation::Vecrem => play(algorithms::Vecrem::new, args.max),
        Implementation::Once => play(algorithms::OnceInit::new, args.max),
        Implementation::Weight => play(algorithms::Weight::new, args.max),
        Implementation::Prune => play(algorithms::Prune::new, args.max),
    }
}

fn play<G>(mut mk: impl FnMut() -> G, max: usize)
where
    G: Guesser,
{
    let w = popoki::Wordle::new();
    let mut score: usize = 0;
    let mut games = 0_i32;
    for answer in GAMES.split_whitespace().take(max) {
        let answer_b: popoki::Word = answer
            .as_bytes()
            .try_into()
            .expect("all answers are 5 letters");
        let guesser = (mk)();
        if let Some(s) = w.play(answer_b, guesser) {
            games = games.saturating_add(1_i32);
            score = score.saturating_add(s);
            println!("Guessed '{answer}' in {s}");
        } else {
            eprintln!("Failed to guess");
        }
    }
    println!(
        "The average guess score is: {:.2}",
        score as f64 / f64::from(games)
    );
}
