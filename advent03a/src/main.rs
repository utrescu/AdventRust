#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub id: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

// https://doc.rust-lang.org/alloc/str/trait.FromStr.html

impl FromStr for Rectangle {
    type Err = ();

    fn from_str(linia: &str) -> Result<Self, <Self as FromStr>::Err> {
        // Avaluar regex https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }
        let captures = RE.captures(linia).unwrap();

        Ok(Rectangle {
            id: captures[1].parse().unwrap(),
            x: captures[2].parse().unwrap(),
            y: captures[3].parse().unwrap(),
            width: captures[4].parse().unwrap(),
            height: captures[5].parse().unwrap(),
        })
    }
}

fn main() {
    // Ara m'agrada i tot aquest sstema de llegir el fitxer
    let inputdata = include_str!("input");

    let rectangles: Vec<Rectangle> = inputdata
        .lines()
        .map(Rectangle::from_str)
        .map(Result::unwrap)
        .collect();

    let mut ocupacio = HashMap::new();

    for rectangle in rectangles.iter() {
        for fila in rectangle.y..rectangle.y + rectangle.height {
            for columna in rectangle.x..rectangle.x + rectangle.width {
                *ocupacio.entry((columna, fila)).or_insert(0) += 1;
            }
        }
    }

    let out1 = ocupacio.values().filter(|v| **v > 1).count();
    println!("I: {}", out1);
}
