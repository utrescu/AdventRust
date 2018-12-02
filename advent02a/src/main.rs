use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input";
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read file {:?}", filename),
    };

    let buffer = BufReader::new(file);

    let mut count2 = 0;
    let mut count3 = 0;

    for line in buffer.lines() {
        let word = match line {
            Ok(line) => line,
            Err(_) => panic!("Error reading line"),
        };
        let (find2, find3) = comprova_repeticions2(word);
        count2 = count2 + find2;
        count3 = count3 + find3;
    }
    println!("{} x {} = {}", count2, count3, count2 * count3);
}

// Primer intent ... funciona però és un xurro
//
// fn comprova_repeticions(word: String) -> (i32, i32) {
//     let slice: &str = &word[..];

//     let mut chars: Vec<char> = slice.chars().collect();
//     chars.sort_by(|a, b| b.cmp(a));

//     let mut old = ' ';
//     let mut count = 0;
//     let mut two = 0;
//     let mut three = 0;

//     for car in chars {
//         if car == old {
//             count = count + 1;
//         } else {
//             match count {
//                 2 => two = 1,
//                 3 => three = 1,
//                 _ => (),
//             }
//             count = 1;
//         }
//         old = car;
//     }

//     match count {
//         2 => {
//             two = 1;
//             println!("two!")
//         }
//         3 => {
//             println!("three!");
//             three = 1
//         }
//         _ => (),
//     }

//     println!("{} -> {} {}", word, two, three);
//     return (two, three);
// }

pub fn comprova_repeticions2(word: String) -> (i32, i32) {
    let mut two = 0;
    let mut three = 0;

    let slice: &str = &word[..];
    let chars: Vec<char> = slice.chars().collect();

    let mut acumulats = HashMap::with_capacity(26);
    for char in chars {
        *acumulats.entry(char).or_insert(0) += 1;
    }

    if acumulats.values().any(|&count| count == 2) {
        two += 1;
    }
    if acumulats.values().any(|&count| count == 3) {
        three += 1;
    }
    return (two, three);
}
