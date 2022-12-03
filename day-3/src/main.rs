#![feature(assert_matches)]

use color_eyre::{
    eyre::{ensure, eyre},
    Report,
};
use itertools::process_results;
use std::collections::HashMap;

// NOTE: WIP
// I'll need some time to refactor all solutions to impl AOCSolution
// This will also open doors for a more integrated solution-runner.
mod commons {
    pub trait AOCSolution {
        const YEAR: &'static str;
        const DAY: i8;

        type Error;
        type DataModel;

        fn parse_input(input: &str) -> Result<Self::DataModel, Self::Error>;
        fn solve(input: &str) -> Result<(), Self::Error>;
    }
}

struct RuckSackReorganisation();

struct Box(HashMap<char, i32>, HashMap<char, i32>);

impl RuckSackReorganisation {
    /// value of an item type as described in part 1
    pub fn value(c: &char) -> Result<i16, color_eyre::Report> {
        match *c {
            c @ 'a'..='z' => Ok(c as i16 - 'a' as i16 + 1),
            c @ 'A'..='Z' => Ok(c as i16 - 'A' as i16 + 27),
            c => Err(eyre!("unexpected item type: {}", c)),
        }
    }

    /// calculate total value of repeat types
    pub fn part_1(boxes: &[Box]) -> Result<i32, color_eyre::Report> {
        let misplaced_types = boxes.iter().map(|b| {
            let dupli_type =
                b.1.iter()
                    .find(|t| b.0.contains_key(t.0))
                    .ok_or(eyre!("compartments don't share a common type"))
                    .map(|t| t.0)?;
            Self::value(dupli_type)
        });

        process_results(misplaced_types, |iter| {
            iter.fold(0_i32, |acc, v| acc + v as i32)
        })
    }

    /// calculate sum of priorities of the type repeated between
    pub fn part_2(boxes: &[Box]) -> Result<i32, Report> {
        println!("total boxes: {}", boxes.len());
        boxes
            .iter()
            .step_by(3)
            .zip(boxes.iter().skip(1).step_by(3))
            .zip(boxes.iter().skip(2).step_by(3))
            .try_fold(0, |acc, ((a, b), c)| {
                println!("folding some group");
                // To get around the fact that I don't want to refactor my modelling of the data, we
                // clone the original first halves of the containers so we can "extend" them
                let (mut x, mut y, mut z) = (a.0.clone(), b.0.clone(), c.0.clone());
                (
                    x.extend(a.1.iter()),
                    y.extend(b.1.iter()),
                    z.extend(c.1.iter()),
                );

                let shared_types: Vec<_> = x
                    .iter()
                    .filter(|(c, _)| y.contains_key(*c) && z.contains_key(*c))
                    .collect();

                ensure!(
                    shared_types.len() == 1,
                    "unexpected number of shared types in group: {} [{:?}]",
                    shared_types.len(),
                    shared_types
                );

                Ok(acc + Self::value(shared_types[0].0)? as i32)
            })
    }
}

use commons::*;

impl AOCSolution for RuckSackReorganisation {
    const YEAR: &'static str = "2022";
    const DAY: i8 = 3;

    type Error = color_eyre::Report;
    type DataModel = Vec<Box>;

    fn parse_input(input: &str) -> Result<Self::DataModel, Self::Error> {
        let parsed: Self::DataModel = input
            .lines()
            .map(|line| {
                let line: Vec<char> = line.chars().collect();
                let first_half = line.get(0..line.len() / 2).unwrap();
                let second_half = line.get(line.len() / 2..line.len()).unwrap();

                let compartment_1: HashMap<char, i32> =
                    first_half.iter().fold(HashMap::new(), |mut map, c| {
                        map.entry(*c).and_modify(|o| *o += 1).or_default();
                        map
                    });

                let compartment_2: HashMap<char, i32> =
                    second_half.iter().fold(HashMap::new(), |mut map, c| {
                        map.entry(*c).and_modify(|o| *o += 1).or_default();
                        map
                    });

                Box(compartment_1, compartment_2)
            })
            .collect();

        Ok(parsed)
    }

    fn solve(input: &str) -> Result<(), Self::Error> {
        let boxes: Vec<Box> = Self::parse_input(input)?;

        let part_1 = Self::part_1(boxes.as_ref())?;
        println!("total value of repeat types: {}", part_1);

        let part_2 = Self::part_2(boxes.as_ref())?;
        println!(
            "total value of shared types between groups of 3: {}",
            part_2
        );

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::assert_matches::assert_matches;

    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../input.txt");
        let boxes = RuckSackReorganisation::parse_input(input)
            .expect("solution's input parser should parse without error");

        assert_matches!(RuckSackReorganisation::part_1(boxes.as_ref()), Ok(7691));
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../input.txt");
        let boxes = RuckSackReorganisation::parse_input(input)
            .expect("input should be parsed");
        
        assert_matches!(RuckSackReorganisation::part_2(boxes.as_ref()), Ok(2508));
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let input = include_str!("../input.txt");

    RuckSackReorganisation::solve(input)?;

    Ok(())
}
