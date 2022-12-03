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

enum Outcome {
    Win,
    Draw,
    Lose
}

impl From<char> for Outcome {
    fn from(c: char) -> Outcome {
        match c {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("unexpcted outcome: {c}")
        }
    }
}

impl From<char> for Hand {
    fn from(c: char) -> Self {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("unexpected hand: {c}")
        }
    }
}

impl From<(Outcome, Hand)> for Hand {
    fn from((outcome, other_hand): (Outcome, Hand)) -> Self {
        match outcome {
            Outcome::Win => other_hand.loses_against(),
            Outcome::Draw => other_hand,
            Outcome::Lose => other_hand.wins_against(),
        }
    }
}

impl Hand {
    fn intrinsic_score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn loses_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        }
    }

    // surely theirs a more elegant (but at the same time efficient) way to
    // represent this transitive relationship?
    fn wins_against(&self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Scissors => Hand::Paper,
            Hand::Paper => Hand::Rock,
        }
    }

    fn score(&self, other_hand: Hand) -> i32 {
        let clash_score = match other_hand {
            ye if &ye == self => 3,
            agg if agg == self.loses_against() => 0,
            _ => 6,
        };

        clash_score + self.intrinsic_score()
    }
}

/// round represents a RPS round
struct Round(Hand, Hand);

fn main() -> std::io::Result<()> {
    let file = {
        let name = std::env::args().nth(1).expect("no file name passed");
        BufReader::new(File::open(name)?)
    };

    // parse `t1 t2` pairs from file
    let rounds: Vec<(char, char)> = file
        .lines()
        .flatten()
        .map(|line| {
            let mut chars = line.chars();
            let (Some(t1), Some(' '), Some(t2), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
                panic!("expected round fmt: <hand><spc><hand>, got {}", line);
            };
            (t2, t1)
        })
        .collect();

    // part 1 tells us `t1 t2` -> `their_hand, our_hand`
    let score_part_1 = total_score_for_second_player(
        rounds
            .iter()
            .map(|(theirs, ours)| Round(Hand::from(*theirs), Hand::from(*ours))),
    );
    println!("total score, assuming second token is our hand: {}", score_part_1);

    // part 2 tells us `t1 t2` -> `their_hand, desired_outcome`
    let score_part_2 = total_score_for_second_player(rounds.iter().map(|(theirs, outcome)| {
        let theirs = Hand::from(*theirs);
        let outcome = Outcome::from(*outcome);
        let ours = Hand::from((outcome, theirs));

        Round(theirs, ours)
    }));
    println!(
        "total score, knowing second token is outcome: {}",
        score_part_2
    );

    Ok(())
}

// Compute total score 
fn total_score_for_second_player<T>(rounds: T) -> i32
where
    T: Iterator<Item = Round>,
{
    rounds
        .map(|round| round.1.score(round.0))
        .reduce(|acc, s| acc + s)
        .expect("there must be at least one round to compute a score")
}
