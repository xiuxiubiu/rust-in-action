use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("readme.md").unwrap();
    let br = BufReader::new(f);

    for line in br.lines() {
        let line = line.unwrap();

        println!("{} ({} bytes long)", line, line.len());
    }
}
