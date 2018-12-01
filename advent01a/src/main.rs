use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut frequency = 0;
    let filename = "input";

    let file = File::open(filename).unwrap();
    for line in BufReader::new(file).lines() {
        match line {
            Err(why) => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<isize>() {
                Err(_) => panic!("Not a number"),
                Ok(number) => {
                    frequency = frequency + number;
                }
            },
        }
    }
    print!("Frequency: {}", frequency);
}
