// TODO: procedural (?) macro to generate the entire trait
// proposed usage: AOCSolution!{ year = x, day = y, DataModel = SomeType, Solver = solver}
pub trait AOCSolution {
    // TODO: attribute macro for `YEAR` and date?
    const YEAR: &'static str;
    const DAY: i8;

    type Error;
    type DataModel;

    fn parse_input(input: &str) -> Result<Self::DataModel, Self::Error>;
    fn solve(input: Self::DataModel) -> Result<(), Self::Error>;
}

// TODO:
// question: how do you pass around static/non-binding trait implementations?
pub fn run_solution(_solution: impl AOCSolution) -> Result<(), Box<dyn std::error::Error>> {
    // a generic solution runner would download the input at this step

    Ok(())
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
