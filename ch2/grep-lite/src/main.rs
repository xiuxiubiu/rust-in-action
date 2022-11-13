use clap::{arg, command, Parser};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about = "searches for patterns", long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(short, long, help = "The pattern to search for")]
    pattern: String,

    /// File to search
    #[arg(short, long, help = "File To search")]
    input: String,
}

fn process_lines<T>(reader: T, re: Regex)
where
    T: BufRead,
{
    for line in reader.lines() {
        let line = line.unwrap();

        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(&args.pattern).unwrap();

    //     let quote = "Every face, every shop, bedroom window, public-house,
    // and dark square is a picture feverishly turned--in search of what?
    // It is the same with books. What do we seek through millions of pages?";

    if args.input == "-" {
        let stdin = io::stdin();
        let reader = BufReader::new(stdin);
        process_lines(reader, re);
    } else {
        let f = File::open(args.input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }
}
