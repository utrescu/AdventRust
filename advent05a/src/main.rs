fn reduce(mut input: Vec<char>) -> Vec<char> {
    for i in (0..input.len()).rev() {
        if i < input.len() - 1
            && input[i] != input[i + 1]
            && input[i].eq_ignore_ascii_case(&input[i + 1])
        {
            // S'han d'eliminar tots dos
            input.remove(i);
            input.remove(i);
        }
    }
    return input;
}

fn main() {
    let inputdata = include_str!("input");
    // Trec el \n del final
    let chars: Vec<char> = inputdata[..inputdata.len() - 1].chars().collect();

    let resultat = reduce(chars);

    println!("{}", resultat.len());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn comprova_que_redueix_be() {
        let proves: Vec<(String, usize)> =
            vec![(String::from("bAaB"), 0), (String::from("abc"), 3)];

        for prova in proves {
            let data = prova.0.chars().collect();
            assert_eq!(reduce(data).len(), prova.1);
        }
    }

}
