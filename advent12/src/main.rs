use std::collections::HashMap;
use std::io;

const SIESREPETEIXENPLEGA: u32 = 10;

fn evolution(initial: &str, data: &Vec<&str>, generacions: usize) -> Result<isize, ()> {
    let mut transformacions: HashMap<&str, char> = HashMap::new();

    data[2..].iter().for_each(|s| {
        transformacions.insert(&s[0..5], if s.ends_with('#') { '#' } else { '.' });
    });

    // Poso els marges del davant i del darrera
    let mut pots = format!("...{}...", initial);

    let mut suma_abans = 0;

    let mut diffs: HashMap<isize, u32> = HashMap::new();
    for gen in 1..=generacions {
        let mut nova_generacio = String::from("");
        for i in 2..pots.len() - 2 {
            let slice = &pots[i - 2..=i + 2];
            match transformacions.get(slice) {
                Some('#') => {
                    nova_generacio.push('#');
                }
                _ => nova_generacio.push('.'),
            }
        }
        // Ja tenim la nova generació.
        pots = format!("...{}...", nova_generacio);

        // Però l'estat inicial era 0 i els nous
        // per l'esquerra són negatius ... (fem servir index) i '3' + generació de referència
        let suma = pots
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum::<isize>();

        // Per la segona solució m'han "xivat" que en algun moment les diferències són sempre iguals o sigui que
        // es podrà plegar

        let score = pots
            .chars()
            .enumerate()
            .filter(|(_, c)| c == &'#')
            .map(|(i, _)| i as isize - (3 + gen as isize))
            .sum::<isize>();
        let e = diffs
            .entry(score as isize - suma_abans as isize)
            .or_insert(0);
        if *e > SIESREPETEIXENPLEGA {
            return Ok((generacions - gen) as isize * (score - suma_abans) + score);
        } else {
            *e += 1;
        }

        suma_abans = suma;
    }
    Ok(suma_abans)
}

fn main() -> io::Result<()> {
    let data = include_str!("input").lines().skip(2).collect::<Vec<_>>();
    println!("Part 1: {:?}", evolution("##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#", &data, 20));

    println!("Part 2: {:?}", evolution("##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#", &data, 50_000_000_000));

    Ok(())
}
