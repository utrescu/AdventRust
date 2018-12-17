/// Retorna (avança, llista de resultats dels fills)
///
///
fn navega2(input: &[usize]) -> (usize, Vec<usize>) {
    let num_fills = input[0];
    let num_metadades = input[1];

    // Com a mínim he d'avançar dues posicions
    let mut size = 2;
    let mut valors: Vec<usize> = Vec::new();
    let mut suma = 0;
    if num_fills != 0 {
        let mut fills: Vec<usize> = Vec::new();
        for _ in 0..num_fills {
            let (avanca, tmpsuma) = navega2(&input[size..]);
            size = size + avanca;
            fills.push(tmpsuma.iter().sum());
        }
        for valor in &input[size..size + num_metadades] {
            if *valor - 1 < fills.len() {
                suma = suma + fills[*valor - 1];
            }
        }
    } else {
        for valor in &input[size..size + num_metadades] {
            suma = suma + valor;
        }
    }
    valors.push(suma);
    (size + num_metadades, valors)
}

fn main() {
    let inputdata = include_str!("input");
    let input: Vec<usize> = inputdata
        .trim()
        .split(' ')
        .into_iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let (_, resultat) = navega2(&input);
    println!("It's {}", resultat[0]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exemple_de_mostra() {
        let data = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (_, resultat) = navega2(&data);
        println!("RESULTAT: {:?}", resultat);

        assert_eq!(resultat[0], 66);
    }
}
