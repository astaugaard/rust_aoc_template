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
    // list of tests (functions that take a bool (weather to be verbose))
    // and return an Option<String>
    // None if there was no error
    // Some(err) if the test failed
    //
    // a helper function golden is supplied so that you can test your parts against real inputs
    //
    // utils::golden(golden name, day, expected output part 1, expected output part 2)
    //
    // golden name is the name of the input file in the goldens directory
    // the day is this structure so it is always &DAY
    // the expected outputs are of the forms Some(expected) or None (when you are not testing that part with this golden)
    //
    // example for aoc day5 in 2021:
    //
    // use crate::utils;
    //
    // test: vec![utils::golden("day5", &DAY, Some("5"), Some("12"))],
    tests: vec![],
    ignore_failed_tests: true, // toggle to false to make the program exit if a test fails useful for debugging


    // do not touch
    parser: Box::new(parser),
    part_a: Box::new(part_a),
    part_b: Box::new(part_b),
});
