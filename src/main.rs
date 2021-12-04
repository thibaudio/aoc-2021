mod day1;
use day1::Day1;
mod day2;
use day2::Day2;
mod day3;
use day3::Day3;
mod day4;
use day4::Day4;
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
        _ => panic!("Day not implemented"),
    }
}
