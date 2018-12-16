/// Retorna (avança, suma)
///
///
fn navega(input: &[usize]) -> (usize, usize) {
    let num_fills = input[0];
    let num_metadades = input[1];

    // Com a mínim he d'avançar dues posicions
    let mut size = 2;
    let mut suma = 0;
    if num_fills != 0 {
        for _ in 0..num_fills {
            let (avanca, tmpsuma) = navega(&input[size..]);
            size = size + avanca;
            suma = suma + tmpsuma;
        }
    }
    for valor in &input[size..size + num_metadades] {
        suma = suma + valor;
    }
    (size + num_metadades, suma)
}

fn main() {
    let inputdata = include_str!("input");
    let input: Vec<usize> = inputdata
        .trim()
        .split(' ')
        .into_iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    let (_, resultat) = navega(&input);
    println!("{}", resultat);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exemple_de_mostra() {
        let data = [2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2];
        let (_, resultat) = navega(&data);

        assert_eq!(resultat, 138);
    }
}
