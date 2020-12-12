use std::{env::var, error::Error};

use advent_utils::Solver;

fn run<S: Solver>() -> Result<(), Box<dyn Error>> {
    let input_file = format!(
        "{}/day-{:02}/input.txt",
        var("BASE_PATH").unwrap_or_else(|_| ".".to_owned()),
        S::day_number()
    );
    let solver = S::try_from(input_file.into())?;

    for part in S::implemented_parts() {
        println!("day {:02}: {}", S::day_number(), solver.solve(part));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    run::<day_01::Solution>()?;
    run::<day_02::Solution>()?;
    run::<day_03::Solution>()?;
    run::<day_04::Solution>()?;
    run::<day_05::Solution>()?;
    run::<day_06::Solution>()?;
    run::<day_07::Solution>()?;
    run::<day_08::Solution>()?;
    run::<day_09::Solution>()?;
    run::<day_10::Solution>()?;
    run::<day_11::Solution>()?;
    run::<day_12::Solution>()?;

    Ok(())
}
