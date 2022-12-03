use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
struct Elf {
    index: i32,
    calories: i32,
}

/// Calories carried by elf with the most calories
fn part_1(elves: &mut Vec<Elf>) {
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    println!(
        "Elf {} has most calories: {}",
        elves[0].index, elves[0].calories
    );
}

/// Calories carries by 3 elves with the most calories
fn part_2(elves: &mut Vec<Elf>) {
    elves.sort_by(|a, b| b.calories.cmp(&a.calories));
    println!(
        "total of top 3: {}",
        elves[0].calories + elves[1].calories + elves[2].calories
    );
}

fn main() -> std::io::Result<()> {
    let file = {
        let name = std::env::args().nth(1).expect("no file name passed");
        BufReader::new(File::open(name)?)
    };

    let (_current_elf, mut elves) =
        file.lines()
            .flatten()
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

    part_1(&mut elves);
    part_2(&mut elves);

    Ok(())
}
