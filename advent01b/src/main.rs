use std::fs::File;
use std::io::{BufRead, BufReader};

fn process(start: isize, list: &Vec<isize>, frequencies: &mut Vec<isize>) -> (isize, bool) {
    let mut frequency = start;

    for number in list {
        frequency = frequency + number;

        if frequencies.contains(&frequency) {
            return (frequency, true);
        }
        frequencies.push(frequency);
    }
    return (frequency, false);
}

fn main() {
    let mut valors: Vec<isize> = Vec::new();
    let mut frequencies: Vec<isize> = Vec::new();
    let mut frequency: isize = 0;
    let mut found = false;
    let filename = "input";

    // Read the file in `valors`
    let file = File::open(filename).unwrap();
    for line in BufReader::new(file).lines() {
        match line {
            Err(why) => panic!("{:?}", why),
            Ok(string) => match string.trim().parse::<isize>() {
                Err(_) => panic!("Not a number"),
                Ok(number) => {
                    valors.push(number);
                }
            },
        }
    }

    // Go!
    let mut i = 1;
    while !found {
        let (newfrequency, isresult) = process(frequency, &valors, &mut frequencies);
        found = isresult;
        frequency = newfrequency;
        println!("Round {} : {}, {}", i, frequency, found,);
        i = i + 1;
    }
}
