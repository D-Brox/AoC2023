use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand, ValueEnum};
use seq_macro::seq;

mod days;
use days::*;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, ValueEnum)]
enum Part {
    Part1,
    Part2,
}

seq!( N in 1..=25 {
    #[derive(Subcommand)]
    enum Commands {
        #(
            Day~N{
                #[arg(value_enum)]
                part: Part,
                #[arg(short, long, value_name = "FILE")]
                input: PathBuf,
            },
        )*
    }
});

fn main() {
    let cli = Cli::parse();
    seq!( N in 1..=25 {
        match &cli.command {
            #(
                Commands::Day~N{part,input} => {
                    let input:Vec<String> = read_lines(input)
                        .unwrap()
                        .map(|l|l.unwrap())
                        .collect();
                    match part{
                        Part::Part1 => println!("{:?}",day~N::solution1(input)),
                        Part::Part2 => println!("{:?}",day~N::solution2(input)),
                    }
                },
            )*
        }
    });
}
