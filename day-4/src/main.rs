use color_eyre::{eyre::ensure, Report};

struct SimpleRange(i32, i32);

impl SimpleRange {
    fn contained_in(&self, x: &Self) -> bool {
        x.0 <= self.0 && x.1 >= self.1
    }

    fn overlap_with(&self, x: &Self) -> bool {
        // a range X (a,b) overlaps with another Y (c, d), if:
        // - X starts before Y starts and ends after Y starts (a <= c <= b), or
        // - X starts after Y starts but before it ends (c <= a <= d)
        (self.0 <= x.0 && self.1 >= x.0) || (self.0 >= x.0 && self.0 <= x.1)
    }
}

struct RangePair(SimpleRange, SimpleRange);

fn parse_input(input: &str) -> Result<Vec<RangePair>, Report> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            ensure!(
                parts.len() == 2,
                "each line must have exactly 2 range reprs"
            );
            // ! ugly `mut` to move the SimpleRange pair without cloning :/
            let mut ranges = parts
                .iter()
                .map(|part| {
                    let extremes = part
                        .split("-")
                        .map(|p| p.parse())
                        .collect::<Result<Vec<i32>, _>>()?;
                    ensure!(
                        extremes.len() == 2,
                        "expected range in format `<lower>-<upper>, got: {}",
                        part
                    );
                    Ok(SimpleRange(extremes[0], extremes[1]))
                })
                .collect::<Result<Vec<SimpleRange>, _>>()?;
            Ok(RangePair(ranges.swap_remove(0), ranges.swap_remove(0)))
        })
        .collect()
}

fn solve(range_pairs: Vec<RangePair>) -> Result<(), Report> {
    let total_conflicts = range_pairs.iter().fold(0, |acc, pair| {
        if pair.0.contained_in(&pair.1) || pair.1.contained_in(&pair.0) {
            acc + 1
        } else {
            acc
        }
    });
    println!("total absolute conflicts = {}", total_conflicts);

    let total_overlaps = range_pairs.iter().fold(0, |acc, pair| {
        if pair.0.overlap_with(&pair.1) {
            acc + 1
        } else {
            acc
        }
    });
    println!("total conflicts = {}", total_overlaps);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../input.txt");
        let parsed_input = parse_input(input).expect("input should be parsed successfully");
        assert_eq!(parsed_input.len(), 1000);
    }
}

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let parsed_input = parse_input(include_str!("../input.txt"))?;
    solve(parsed_input)?;
    Ok(())
}
