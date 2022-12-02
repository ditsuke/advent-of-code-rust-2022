use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn aggressor(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    // surely theirs a more elegant (but at the same time efficient) way to
    // represent this transitive relationship?
    fn victim(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Scissors => Hand::Paper,
            Hand::Paper => Hand::Rock,
        }
    }

    fn score(&self, other_hand: Hand) -> i32 {
        use Hand::*;

        let intrinsic_score = match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        };

        let clash_score = match other_hand {
            ye if &ye == self => 3,
            agg if agg == self.aggressor() => 0,
            _ => 6,
        };

        clash_score + intrinsic_score
    }
}

fn parse_token<S>(token: S) -> Hand
where
    S: AsRef<str>,
{
    match token.as_ref() {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        token => panic!("unexpected token: {}", token),
    }
}

fn parse_strategy<S>(token: S, theirs: Hand) -> Hand
where
    S: AsRef<str>,
{
    match token.as_ref() {
        "X" => theirs.victim(),
        "Y" => theirs,
        "Z" => theirs.aggressor(),
        token => panic!("unexpected token: {}", token),
    }
}

struct Round(Hand, Hand);

fn main() -> std::io::Result<()> {
    let file = {
        let name = std::env::args().nth(1).expect("no file name passed");
        BufReader::new(File::open(name)?)
    };

    // parse `k v` pairs from file
    let rounds: Vec<(String, String)> = file
        .lines()
        .flatten()
        .map(|line| {
            let tokens = line.split(" ").collect::<Vec<_>>();
            assert!(
                tokens.len() == 2,
                "unexpected input format; line = {}",
                line
            );

            (tokens[0].to_owned(), tokens[1].to_owned())
        })
        .collect();

    let score_part_1 = total_score(
        rounds
            .iter()
            .map(|(theirs, ours)| Round(parse_token(theirs), parse_token(ours))),
    );
    println!("total score: {}", score_part_1);

    let score_part_2 = total_score(rounds.iter().map(|(theirs, outcome)| {
        let theirs = parse_token(theirs);
        let ours = parse_strategy(outcome, theirs);
        Round(theirs, ours)
    }));
    println!(
        "total score with outcome-based interpretation = {}",
        score_part_2
    );

    Ok(())
}

fn total_score<T>(rounds: T) -> i32
where
    T: Iterator<Item = Round>,
{
    rounds
        .map(|round| round.1.score(round.0))
        .reduce(|acc, s| acc + s)
        .expect("there must be at least one round to compute a score")
}
