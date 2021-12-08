mod day1;
use day1::Day1;
mod day2;
use day2::Day2;
mod day3;
use day3::Day3;
mod day4;
use day4::Day4;
mod day5;
use day5::Day5;
mod day6;
use day6::Day6;
mod day7;
use day7::Day7;
mod day8;
use day8::Day8;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    // Which day we want to run
    #[structopt(short, long)]
    day: u32,
}

fn main() {
    let args = Cli::from_args();
    match &args.day {
        1 => Day1::run(),
        2 => Day2::run(),
        3 => Day3::run(),
        4 => Day4::run(),
        5 => Day5::run(),
        6 => Day6::run(),
        7 => Day7::run(),
        8 => Day8::run(),
        _ => panic!("Day not implemented"),
    }
}
