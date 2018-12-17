extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let re = Regex::new(r"(\d+), (\d+)").unwrap();

    let inputdata = include_str!("input");

    let mut punts: HashMap<(i32, i32), i32> = HashMap::new();
    let mut llistax: Vec<i32> = Vec::new();
    let mut llistay: Vec<i32> = Vec::new();

    for linia in inputdata.lines() {
        let lloc = re.captures(linia).unwrap();
        let x: i32 = lloc[1].parse().unwrap();
        let y: i32 = lloc[2].parse().unwrap();
        punts.insert((x, y), 0);
        llistax.push(x);
        llistay.push(y);
    }

    let minx: i32 = *llistax.iter().min().unwrap();
    let maxx: i32 = *llistax.iter().max().unwrap();
    let miny: i32 = *llistay.iter().min().unwrap();
    let maxy: i32 = *llistay.iter().max().unwrap();

    for actualy in miny - 1..maxy + 1 {
        for actualx in minx - 1..maxx + 1 {
            let mut min_dist = maxx - minx + maxy - miny;
            let mut mes_proper = None;
            for &(x, y) in punts.keys() {
                let dist = (x - actualx).abs() + (y - actualy).abs();
                if dist < min_dist {
                    min_dist = dist;
                    mes_proper = Some((x, y));
                } else if dist == min_dist {
                    mes_proper = None;
                }
            }

            if mes_proper.is_some() {
                let closest = mes_proper.unwrap();
                // Si és dels costats out!
                if actualx == maxx || actualx == minx || actualy == miny || actualy == maxy {
                    *punts.entry(closest).or_insert(1_000_000) += -1_000_000;
                } else {
                    *punts.entry(closest).or_insert(0) += 1;
                }
            }
        }
    }
    let max = punts.iter().max_by_key(|&(_, b)| b).unwrap();
    println!("Màxim: {:?}", max);
}
