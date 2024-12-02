use once_cell::sync::Lazy;

use crate::day;

type Input = ();

fn parser(_input: String, _verbose: bool) -> Result<Input, String> {
    Ok(())
}

fn part_a(_input: &Input) -> Option<String> {
    None
}

fn part_b(_input: &Input) -> Option<String> {
    None
}

pub static DAY: Lazy<day::Day<Input>> = Lazy::new(|| day::Day {
    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
});
