mod day1;
use day1::Day1;
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
        _ => panic!("Day not implemented"),
    }
}
