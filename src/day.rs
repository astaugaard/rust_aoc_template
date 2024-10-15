use colored::Colorize;
use core::fmt;
use std::{fs, time::Instant};

pub struct Day<A> {
    pub parser: Box<dyn Fn(String, bool) -> Result<A, String> + Sync + Send>,
    pub part_a: Box<dyn Fn(&A) -> Option<String> + Sync + Send>,
    pub part_b: Box<dyn Fn(&A) -> Option<String> + Sync + Send>,
    pub tests: Vec<Box<dyn Fn(bool) -> Option<String> + Sync + Send>>,
    pub ignore_failed_tests: bool,
}

fn run_tests(
    tests: &Vec<Box<dyn Fn(bool) -> Option<String> + Sync + Send>>,
    verbose: bool,
) -> Option<String> {
    for i in tests {
        if let Some(a) = (*i)(verbose) {
            return Some(a);
        };
    }
    None
}
fn run_day<A>(day: &Day<A>, verbose: bool, skip_tests: bool, number: u32) -> Result<(), String>
where
    A: fmt::Debug,
{
    println!(
        "{}",
        format!("======= Day {:2} ========", number).bright_cyan()
    );

    let file = fs::read_to_string(format!("inputs/day{}", number)).map_err(|e| format!("{e}"))?;

    if !skip_tests {
        match run_tests(&day.tests, verbose) {
            Some(err) => {
                println!("{}", format!("test failed: {}", err).red());
            }
            None => {}
        }
    }

    let now = Instant::now();

    let parsed = match (*day.parser)(file, verbose) {
        Ok(parsed) => parsed,
        Err(err) => return Err(format!("failed to parse input: {}", err)),
    };

    println!("parsing time: {:.2?}", now.elapsed());

    println!("{}", "====== part A ======".bright_magenta());

    let now = Instant::now();

    match (*day.part_a)(&parsed) {
        Some(ret) => {
            println!("{}", ret);
            println!("part a time: {:.2?}", now.elapsed());
        }
        None => println!("not yet implemented"),
    }

    let now = Instant::now();

    match (*day.part_b)(&parsed) {
        Some(ret) => {
            println!("{}", ret);
            println!("part a time: {:.2?}", now.elapsed());
        }
        None => println!("not yet implemented"),
    }

    Ok(())
}

pub fn create_day<A>(day: &'static Day<A>) -> Box<dyn Fn(bool, bool, u32) -> Result<(), String>>
where
    A: fmt::Debug,
{
    Box::new(|verbose, skip_tests, num| run_day(day, verbose, skip_tests, num))
}
