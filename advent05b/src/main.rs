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
    let allcharacters = "abcdefghijklmnopqrstuvwxyz";
    let inputdata = include_str!("input");
    // Trec el \n del final
    let chars: Vec<char> = inputdata[..inputdata.len() - 1].chars().collect();

    let mut min = chars.len();

    for lletra in allcharacters.chars() {
        let removed = chars
            .iter()
            .filter(|c| !c.eq_ignore_ascii_case(&lletra))
            .cloned()
            .collect();
        let resultat = reduce(removed);
        if resultat.len() < min {
            min = resultat.len();
        }
    }
    println!("La millor opció és {}", min);
}
