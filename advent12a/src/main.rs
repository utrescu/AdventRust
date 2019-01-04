use std::collections::HashMap;
use std::io;

fn evolution(initial: &str, data: &Vec<&str>, generacions: usize) -> Result<isize, ()> {
    let mut transformacions: HashMap<&str, char> = HashMap::new();

    data[2..].iter().for_each(|s| {
        transformacions.insert(&s[0..5], if s.ends_with('#') { '#' } else { '.' });
    });

    // Poso els marges del davant i del darrera
    let mut pots = format!("...{}...", initial);

    let mut suma_abans = 0;

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

        // El mateix però pel final ja que tenim indexat començant per negatius...
        // (per això no s'entenia res de l'enunciat)

        suma_abans = suma;
    }
    Ok(suma_abans)
}

fn main() -> io::Result<()> {
    let data = include_str!("input").lines().skip(2).collect::<Vec<_>>();
    println!("Part 1: {:?}", evolution("##.#....#..#......#..######..#.####.....#......##.##.##...#..#....#.#.##..##.##.#.#..#.#....#.#..#.#", &data, 20));

    Ok(())
}
