#![feature(hash_drain_filter)]

use std::collections::HashMap;

use color_eyre::{eyre::bail, Report};
use itertools::{FoldWhile, Itertools};

type Model = String;

#[derive(Default, Debug)]
struct SolutionState {
    start: usize,
    sequence_index: usize,
    seen: HashMap<char, usize>,
}

fn parse_input(input: &str) -> Model {
    input.to_owned()
}

fn find_characters_to_process_for_x_distinct(input: Model, chars: usize) -> Result<usize, Report> {
    let sol_state =
        input
            .chars()
            .enumerate()
            .fold_while(SolutionState::default(), |state, (index, c)| {
                if let Some(seen_at) = state.seen.get(&c) {
                    let seen_at = seen_at.to_owned();
                    FoldWhile::Continue(SolutionState {
                        start: seen_at + 1,
                        seen: state
                            .seen
                            .iter()
                            // Get rid of the duplicate char and everything seen before it
                            .filter_map(|(k, v)| if *v < seen_at { None } else { Some((*k, *v)) })
                            // include the current character/index
                            .chain([(c, index)].into_iter())
                            .collect(),

                        sequence_index: index - seen_at,
                    })
                } else {
                    let new_state = SolutionState {
                        seen: {
                            let mut new = state.seen;
                            new.insert(c, index);
                            new
                        },
                        sequence_index: state.sequence_index + 1,
                        ..state
                    };
                    if new_state.sequence_index == chars {
                        FoldWhile::Done(new_state)
                    } else {
                        FoldWhile::Continue(new_state)
                    }
                }
            });

    match sol_state {
        FoldWhile::Continue(s) => bail!(
            "did not find start of sequence of {} distinct characters. last state: {:?}",
            chars,
            s
        ),
        FoldWhile::Done(s) => Ok(s.start + chars),
    }
}

fn part1(input: Model) -> Result<usize, Report> {
    find_characters_to_process_for_x_distinct(input, 4)
        .map_err(|e| e.wrap_err("did not find start-of-packet"))
}

fn part_2(input: Model) -> Result<usize, Report> {
    find_characters_to_process_for_x_distinct(input, 14)
        .map_err(|e| e.wrap_err("did not find start-of-message"))
}

fn solve(input: Model) -> Result<(), Report> {
    // PART 1
    match part1(input.clone()) {
        Ok(i) => println!("index of first start-of-packet: {}", i),
        Err(e) => return Err(e.wrap_err("failed to find start-of-packet in input")),
    }

    // PART 2
    match part_2(input.clone()) {
        Ok(i) => println!("characters to process to find start-of-message: {}", i),
        Err(e) => return Err(e.wrap_err("failed to find start-of-message in input")),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "bacacsd";
        assert_eq!(part1(input.into()).expect("input has a sequence"), 3);

        let input: String = "aabcd".into();
        assert_eq!(part1(input).expect("input has sequence"), 1);
        //                  0123456789
        let input: String = "aaaaaaabcd".into();
        assert_eq!(part1(input).expect("input has a sequence"), 6);

        let input = "bvsvcss";
        let e = part1(input.into()).expect_err("input don't have seq");
        println!("expected error is here: {:?}", e);
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let parsed_input = parse_input(include_str!("../input.txt"));
    solve(parsed_input)?;
    Ok(())
}
