#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
struct Punt {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Punt {
    fn mou(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }

    fn recula(&mut self) {
        self.x -= self.dx;
        self.y -= self.dy;
    }
}

impl FromStr for Punt {
    type Err = ();

    fn from_str(linia: &str) -> Result<Self, <Self as FromStr>::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<\s*(-?\d*),\s*(-?\d*)> velocity=<\s*(-?\d*),\s*(-?\d*)>")
                    .unwrap();
        }
        let captures = RE.captures(linia).unwrap();

        Ok(Punt {
            x: captures[1].parse().unwrap(),
            y: captures[2].parse().unwrap(),
            dx: captures[3].parse().unwrap(),
            dy: captures[4].parse().unwrap(),
        })
    }
}

fn main() {
    let inputdata = include_str!("input");

    let mut punts: Vec<Punt> = inputdata
        .lines()
        .map(Punt::from_str)
        .map(Result::unwrap)
        .collect();

    let mut oldarea = std::i64::MAX;
    for seconds in 1.. {
        punts.iter_mut().for_each(|punt| punt.mou());

        let maxx = punts.iter().map(|p| p.x).max().unwrap();
        let minx = punts.iter().map(|p| p.x).min().unwrap();
        let maxy = punts.iter().map(|p| p.y).max().unwrap();
        let miny = punts.iter().map(|p| p.y).min().unwrap();
        let area = (maxx - minx) * (maxy - miny);
        // Quan el mapa es faci gran és que les lletres es separen
        if area > oldarea {
            println!("Ha tardat {} segons {} -> {}", seconds - 1, area, oldarea);
            punts.iter_mut().for_each(|punt| punt.recula());
            break;
        }
        oldarea = area;
        println!("Area {}", area);
    }

    // Ara he de pintar la graella ... minx i miny per començar

    let min_x = punts.iter().map(|n| n.x).min().unwrap();
    let min_y = punts.iter().map(|n| n.y).min().unwrap();
    // Suposem que en fem prou amb 100x10 (no sé com posar-hi valors no constants)
    let mut pantalla: [[char; 100]; 10] = [['.'; 100]; 10];
    punts
        .iter()
        .for_each(|n| pantalla[(n.y - min_y) as usize][(n.x - min_x) as usize] = '#');
    // imprimir la graella
    pantalla.iter().for_each(|line| {
        line.iter().for_each(|c| print!("{}", c));
        println!();
    });
}
