use commons::AOCSolution;

#[derive(Default)]
struct Elf {
    index: i32,
    calories: i32,
}

/// Calories carried by elf with the most calories
struct Calories {}

impl AOCSolution for Calories {
    const YEAR: &'static str = "2022";

    const DAY: i8 = 1;

    type Error = std::io::Error;

    type DataModel = Vec<Elf>;

    fn parse_input(input: &str) -> Result<Self::DataModel, Self::Error> {
        let (_, elves) = input
            .lines()
            .fold((Elf::default(), Vec::<Elf>::new()), |acc, line| {
                if line.is_empty() {
                    let next_elf = Elf {
                        index: acc.0.index + 1,
                        calories: 0,
                    };

                    let mut elves = acc.1;
                    elves.push(acc.0);
                    return (next_elf, elves);
                }

                let calories: i32 = line
                    .parse()
                    .expect(format!("line should be integer: {}", line).as_str());

                (
                    Elf {
                        calories: acc.0.calories + calories,
                        ..acc.0
                    },
                    acc.1,
                )
            });
        Ok(elves)
    }

    fn solve(mut elves: Self::DataModel) -> Result<(), Self::Error> {
        elves.sort_by(|a, b| b.calories.cmp(&a.calories));
        println!(
            "Elf {} has most calories: {}",
            elves[0].index, elves[0].calories
        );
        println!(
            "total of top 3: {}",
            elves[0].calories + elves[1].calories + elves[2].calories
        );
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    Ok(())
}
