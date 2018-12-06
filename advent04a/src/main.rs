extern crate regex;

use std::collections::HashMap;
use std::fs;

use regex::Regex;

fn read_file() -> Vec<String> {
    let c = fs::read_to_string("input").expect("Read error");
    c.lines().map(|x| x.to_string()).collect()
}

fn main() {
    let re_date = Regex::new(r"\[\d+-\d+-\d+\s\d\d:(\d\d)\]").unwrap();
    let re = Regex::new(r"#(\d+)").unwrap();

    let mut linies = read_file();
    linies.sort();

    let mut sleep_register = HashMap::new();
    let mut guards_sleep_count = HashMap::new();

    let mut idguardia = -1;
    let mut startsleeping = -1;

    for linia in linies.iter() {
        // És una línina inicial?
        if linia.contains("#") {
            let elguardia = re.captures(linia).unwrap();
            idguardia = elguardia[1].parse().unwrap();
        } else if linia.contains("asleep") {
            let readorm = re_date.captures(linia).unwrap();
            startsleeping = readorm[1].parse().unwrap();
        } else if linia.contains("wakes") {
            let readorm = re_date.captures(linia).unwrap();
            let stopsleeping: i32 = readorm[1].parse().unwrap();

            for minute in startsleeping..stopsleeping - 1 {
                *guards_sleep_count.entry(idguardia).or_insert(0) += 1;
                *sleep_register.entry((idguardia, minute)).or_insert(0) += 1;
            }
        }
    }

    // Quin guardia és el que dorm més?
    let (dormilon, _) = guards_sleep_count.iter().max_by_key(|&(_, m)| m).unwrap();
    println!("Dormilón: {}", dormilon);

    // En quin minut dorm més?  dormilon

    let ((dormilonguard, minute), _) = sleep_register
        .iter()
        .filter(|&((g, _), _)| g == dormilon)
        .map(|(k, v)| (k, v))
        .max_by_key(|&(_k, v)| v)
        .unwrap();

    println!(
        "La primera solució és {} x {} = {}",
        dormilonguard,
        minute,
        dormilon * minute
    );

    let ((guard, minute), _) = sleep_register.iter().max_by_key(|&(_k, v)| v).unwrap();

    println!(
        " La segona solució és {:?} x {:?} = {}",
        guard,
        minute,
        guard * minute
    );
}
