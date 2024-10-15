use colored::Colorize;
use std::{thread::available_parallelism, time::Duration};

use clap::{Parser, Subcommand};
mod day;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use day::create_day;
use once_cell::sync::Lazy;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "VERBOSE")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, value_name = "SKIP_TESTS")]
    skip_tests: bool,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Day {
        #[arg(long, short)]
        day: u32,
    },
    All,
}

fn main() {
    let args = Cli::parse();

    let days: [Box<dyn Fn(bool, bool, u32) -> Result<(), String>>; 25] = [
        create_day(Lazy::force(&day1::DAY)),
        create_day(Lazy::force(&day2::DAY)),
        create_day(Lazy::force(&day3::DAY)),
        create_day(Lazy::force(&day4::DAY)),
        create_day(Lazy::force(&day5::DAY)),
        create_day(Lazy::force(&day6::DAY)),
        create_day(Lazy::force(&day7::DAY)),
        create_day(Lazy::force(&day8::DAY)),
        create_day(Lazy::force(&day9::DAY)),
        create_day(Lazy::force(&day10::DAY)),
        create_day(Lazy::force(&day11::DAY)),
        create_day(Lazy::force(&day12::DAY)),
        create_day(Lazy::force(&day13::DAY)),
        create_day(Lazy::force(&day14::DAY)),
        create_day(Lazy::force(&day15::DAY)),
        create_day(Lazy::force(&day16::DAY)),
        create_day(Lazy::force(&day17::DAY)),
        create_day(Lazy::force(&day18::DAY)),
        create_day(Lazy::force(&day19::DAY)),
        create_day(Lazy::force(&day20::DAY)),
        create_day(Lazy::force(&day21::DAY)),
        create_day(Lazy::force(&day22::DAY)),
        create_day(Lazy::force(&day23::DAY)),
        create_day(Lazy::force(&day24::DAY)),
        create_day(Lazy::force(&day25::DAY)),
    ];

    // let default_parallelism_approx = available_parallelism().unwrap().get();

    // println!("threads: {}", default_parallelism_approx);

    match args.command {
        Commands::Day { day } => {
            match (*days[(day - 1) as usize])(args.verbose, args.skip_tests, day) {
                Ok(()) => {}
                Err(err) => {
                    println!("{}", format!("error: {err}").red())
                }
            };
        }
        Commands::All => {
            for (day, dayfun) in days.iter().enumerate() {
                match (*dayfun)(args.verbose, args.skip_tests, (day + 1) as u32) {
                    Ok(()) => {}
                    Err(err) => {
                        println!("{}", format!("error: {err}").red())
                    }
                };
            }
        }
    }
}

