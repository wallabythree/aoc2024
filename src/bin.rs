use std::env;

fn print_usage() {
    eprintln!("Usage: aoc year day");
}

struct Arguments {
    year: u16,
    day: u8,
}

impl Arguments {
    fn parse(args: env::Args) -> Arguments {
        let args: Vec<_> = args.skip(1).collect();

        if args.len() != 2 {
            eprintln!("Incorrect number of arguments");
            print_usage();
            std::process::exit(1);
        }

        let year = args[0].parse::<u16>().expect("Could not parse year");
        let day = args[1].parse::<u8>().expect("Could not parse day");

        Arguments { year, day }
    }
}

fn main() {
    let session_key = env::var("AOC_SESSION")
        .expect("Session key error");

    let args = Arguments::parse(env::args());

    let client = rudolf_rs::Client::new(session_key);
    let input = client.get(args.year, args.day).unwrap();

    let solution = solutions::get_solution(
        args.year as usize,
        args.day as usize
    );

    println!("{}", solution.part1(&input));
    println!("{}", solution.part2(&input));
}

