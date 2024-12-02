use colored::Colorize;
use core::fmt;
use once_cell::sync::Lazy;
use reqwest::header::USER_AGENT;
use std::{
    fs,
    io::Read,
    path::Path,
    time::Instant,
};

pub struct Day<A> {
    pub parser: Box<dyn Fn(String, bool) -> Result<A, String> + Sync + Send>,
    pub part_a: Box<dyn Fn(&A) -> Option<String> + Sync + Send>,
    pub part_b: Box<dyn Fn(&A) -> Option<String> + Sync + Send>,
}

pub struct FetchConfig {
    pub year: u64,
    pub agent: String,
    pub oauthkey: String,
}

pub static FETCH_CONFIG: Lazy<Option<FetchConfig>> = Lazy::new(|| get_fetch_config());

fn get_fetch_config() -> Option<FetchConfig> {
    let content = fs::read_to_string("AOC_FETCH_CONFIG").ok()?;
    let mut content = content.lines();
    let agent = content.next()?;
    let oauthkey = content.next()?;
    let year = content.next()?;

    let year = year.parse::<u64>().ok()?;

    Some(FetchConfig {
        year,
        agent: agent.to_string(),
        oauthkey: oauthkey.to_string(),
    })
}

fn get_day_input(day: u32) -> Result<String, String> {
    let file = format!("inputs/day{day}");
    let file = Path::new(&file);

    if file.exists() {
        let mut file = fs::File::open(file).map_err(|err| err.to_string())?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|err| err.to_string())?;
        return Ok(buf);
    }

    match Lazy::force(&FETCH_CONFIG) {
        Some(conf) => {
            let client = reqwest::blocking::Client::new();
            if !Path::new("inputs").exists() {
                println!(
                    "please make inputs directory or cd to the path that has the inputs directory"
                );
                return Err("no inputs directory".to_string());
            }
            println!("fetching day input");
            let res = client
                .get(format!(
                    "https://adventofcode.com/{}/day/{day}/input",
                    conf.year
                ))
                .header("Cookie", format!("session={};", conf.oauthkey))
                .header(USER_AGENT, &conf.agent)
                .send()
                .map_err(|err| err.to_string())?
                .text()
                .map_err(|err| err.to_string())?;

            fs::write(file, res.as_bytes()).map_err(|err| err.to_string())?;

            Ok(res)
        }
        None => Err(
            "fetch config not set, set fetch config or add file to inputs directory".to_string(),
        ),
    }
}

fn run_day<A>(day: &Day<A>, verbose: bool, number: u32) -> Result<(), String>
where
    A: fmt::Debug,
{
    println!(
        "{}",
        format!("======= Day {:2} ========", number).bright_cyan()
    );

    // let file = fs::read_to_string(format!("inputs/day{}", number)).map_err(|e| format!("{e}"))?;
    let file = get_day_input(number)?;

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

    println!("{}", "====== part B ======".bright_magenta());

    let now = Instant::now();

    match (*day.part_b)(&parsed) {
        Some(ret) => {
            println!("{}", ret);
            println!("part b time: {:.2?}", now.elapsed());
        }
        None => println!("not yet implemented"),
    }

    Ok(())
}

pub fn create_day<A>(day: &'static Day<A>) -> Box<dyn Fn(bool, u32) -> Result<(), String>>
where
    A: fmt::Debug,
{
    Box::new(|verbose, num| run_day(day, verbose, num))
}
