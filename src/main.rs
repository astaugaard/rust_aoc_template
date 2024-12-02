use colored::Colorize;
use std::{
    fs::File,
    io::Write,
};

use chrono::{self, FixedOffset, TimeZone, Utc};
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

use day::{create_day, FETCH_CONFIG};
use once_cell::sync::Lazy;

#[derive(Parser)]
#[command(version,about,long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "VERBOSE")]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    Day {
        #[arg(long, short)]
        day: u32,
    },
    All,
    SetFetchConfig {
        #[arg(long, short)]
        agent: String,

        #[arg(long, short)]
        oauthkey: String,

        #[arg(long, short)]
        year: u64,
    },
}

fn main() {
    let args = Cli::parse();

    let days: [Box<dyn Fn(bool, u32) -> Result<(), String>>; 25] = [
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
            match (*days[(day - 1) as usize])(args.verbose, day) {
                Ok(()) => {}
                Err(err) => {
                    println!("{}", format!("error: {err}").red())
                }
            };
        }
        Commands::All => {
            let current_time = Utc::now();
            // let fetch_config = get_fetch_config();
            for (day, dayfun) in days.iter().enumerate() {
                let day = day+1;
                let runday = match Lazy::force(&FETCH_CONFIG) {
                    Some(conf) => {
                        let release_time = FixedOffset::east_opt(5 * 60 * 60)
                            .unwrap()
                            .with_ymd_and_hms(
                                conf.year as i32,
                                12,
                                day as u32,
                                0, // midnight in the utc time zone
                                0,
                                2,
                            );

                        current_time.naive_utc() > release_time.unwrap().naive_utc()
                    }
                    None => true,
                };

                if runday {
                    match (*dayfun)(args.verbose, day as u32) {
                        Ok(()) => {}
                        Err(err) => {
                            println!("{}", format!("error: {err}").red())
                        }
                    }
                }
            }
        }
        Commands::SetFetchConfig {
            agent,
            oauthkey,
            year,
        } => {
            let mut file = File::create("AOC_FETCH_CONFIG").unwrap();
            file.write_all(format!("{agent}\n{oauthkey}\n{year}").as_bytes())
                .unwrap();
        }
    }
}


// let client = reqwest::Client::new();
//                     let res = match client
//                         .get(&self.aoc_url)
//                         .header("Cookie", format!("session={};", &self.aoc_token))
//                         .send()
//                         .await
//                     {
//                         Ok(x) => x.text().await,
//                         Err(x) => Err(x),
//                     };
