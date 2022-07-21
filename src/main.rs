#![warn(clippy::all, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::implicit_return,
    clippy::separated_literal_suffix,
    clippy::blanket_clippy_restriction_lints,
    clippy::float_arithmetic,
    clippy::cast_precision_loss,
    clippy::as_conversions,
    clippy::print_stderr,
    clippy::print_stdout,
    clippy::option_if_let_else,
    clippy::expect_used,
    clippy::integer_arithmetic,
    clippy::separated_literal_suffix,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

use clap::{Parser, ValueEnum};
use popoki::{Guesser, Solver};

const GAMES: &str = include_str!("../answers.txt");

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Set how candidates are ranked at each step of the solver.
    ///
    /// By default, candidates will be ranked based on expected score.
    #[clap(short, long, value_enum, default_value_t = Rank::ExpectedScore)]
    rank_by: Rank,

    /// By default, correctness computation are cached. This flag disables that.
    #[clap(long)]
    no_cache: bool,

    /// By default, only the most likely 1/3 of candidates are considered at each step. This flag
    /// disables that pruning behavior.
    #[clap(long)]
    no_cutoff: bool,

    /// By default, all games are played in "hard mode" where known-incorrect guesses are
    /// disallowed. This flag allows arbitrary guesses at every step, which allows for more optimal
    /// guessing, but also increases the search space.
    #[clap(long)]
    easy: bool,

    /// The number of games to run.
    ///
    /// If not passed, all Wordle games are run.
    #[clap(short, long, default_value_t = 5)]
    games: usize,
}

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Rank {
    /// Just pick the first candidate.
    First,

    /// E[score] = p(word) * (score + 1) + (1 - p(word)) * (score + E[guesses](entropy - E[information]))
    ExpectedScore,

    /// p(word) * E[information]
    WeightedInformation,

    /// p(word) + E[information]
    InfoPlusProbability,

    /// E[information]
    ExpectedInformation,
}

fn main() {
    let args = Args::parse();
    let mut solver = Solver::builder();

    if args.no_cache {
        solver.cache = false;
    }
    if args.no_cutoff {
        solver.cutoff = false;
    }
    if args.easy {
        solver.hard_mode = false;
    }
    solver.rank_by = match args.rank_by {
        Rank::First => popoki::Rank::First,
        Rank::ExpectedScore => popoki::Rank::ExpectedScore,
        Rank::WeightedInformation => popoki::Rank::WeightedInformation,
        Rank::InfoPlusProbability => popoki::Rank::InfoPlusProbability,
        Rank::ExpectedInformation => popoki::Rank::ExpectedInformation,
    };

    play(move || solver.build(), args.games);
}

fn play<G>(mut mk: impl FnMut() -> G, max: usize)
where
    G: Guesser,
{
    let w = popoki::Wordle::new();
    let mut score = 0;
    let mut games = 0_i32;
    let mut histogram = Vec::new();

    for answer in GAMES.split_whitespace().take(max) {
        let guesser = (mk)();
        if let Some(s) = w.play(answer, guesser) {
            games += 1_i32;
            score += s;
            if s >= histogram.len() {
                histogram.extend(std::iter::repeat(0).take(s - histogram.len() + 1));
            }
            *histogram
                .get_mut(s)
                .expect("Failed while indexing histogram") += 1;
        } else {
            eprintln!("Failed to guess '{answer}'");
        }
    }

    let sum: usize = histogram.iter().sum();
    for (score_h, count) in histogram.into_iter().enumerate().skip(1) {
        let frac = count as f64 / sum as f64;
        let w1 = (30.0 * frac).round() as usize;
        let w2 = (30.0 * (1.0 - frac)).round() as usize;
        eprintln!(
            "{:>2}: {}{} ({})",
            score_h,
            "#".repeat(w1),
            " ".repeat(w2),
            count
        );
    }
    eprintln!("average score: {:.4}", score as f64 / f64::from(games));
}

#[cfg(test)]
mod tests {
    #[test]
    fn default_solver() {
        let w = popoki::Wordle::new();
        let results: Vec<_> = crate::GAMES
            .split_whitespace()
            .take(20)
            .filter_map(|answer| w.play(answer, popoki::Solver::default()))
            .collect();

        assert_eq!(
            results,
            [3, 4, 4, 4, 4, 5, 4, 4, 3, 3, 4, 3, 4, 3, 5, 2, 3, 3, 3, 4]
        );
    }
}
