use clap::{arg, command, Parser};
use regex::Regex;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about = "searches for patterns", long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, help = "The pattern to search for")]
    pattern: String,
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(&args.pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, 
and dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}
