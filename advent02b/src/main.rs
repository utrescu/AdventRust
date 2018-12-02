use std::fs::File;
use std::io::{BufRead, BufReader};

const NOSOLUTION: &str = "No hi ha solució";

fn main() {
    let filename = "input";

    let file = File::open(filename).unwrap();
    let buf = BufReader::new(file);

    // He trobat aquesta forma més elegant de llegir el fitxer
    let words: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    // També es pot llegir el fitxer definint una constant en un include_str! i
    // després usant-la normalment (el fitxer ha d'estar en el directori 'src')
    //
    // const FILE: &str = include_str!("input");
    // let words = FILE.lines().collect::<Vec<String>>();
    // o
    // let words = FILE.lines().collect::<Vec<_>>();

    let letters = find_chars(words.as_slice());
    println!("{}", letters);
}

/// Fet amb el sistema més bèstia que es fa i desfà: força bruta
/// Segurament hi déu haver una forma millor però no se m'ha acudit
///
/// - He hagut d'estar una bona estona amb la documentació per trobar
///   com fer servir zip, filter, count i map però el codi queda més net
///
///
fn find_chars(input: &[String]) -> String {
    for (index, origen) in input.iter().enumerate() {
        for comparat in input.iter().skip(index + 1) {
            if origen
                .chars() // el separa lletra a lletra
                .zip(comparat.chars()) // empaqueta lletra a lletra els dos valors
                .filter(|(a, b)| a != b) // es queda amb els que són diferents
                .count()
                == 1
            // Compta quants en queden
            {
                // Ara toca composar el resultat (bàsicament és fer el mateix)
                return origen
                    .chars()
                    .zip(comparat.chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _)| a) // De cada tupla del zip, es queda amb un caràcter
                    .collect(); // Empaqueta en un zip
            }
        }
    }
    return NOSOLUTION.to_string();
}
