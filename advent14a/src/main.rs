fn problem1(input: usize, recipes: &mut Vec<usize>) -> &[usize] {
    let mut elf1 = 0;
    let mut elf2 = 1;
    while recipes.len() < input + 10 {
        let mut nou_valor = recipes[elf1] + recipes[elf2];
        if nou_valor >= 10 {
            recipes.push(nou_valor / 10);
            nou_valor %= 10;
        }
        recipes.push(nou_valor);
        elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();
    }
    &recipes[input..input + 10]
}

fn problem2(input: String, recipes: &mut Vec<usize>) -> usize {
    let objectiu: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut elf1 = 0;
    let mut elf2 = 1;

    loop {
        let mut dosdigits = 0;
        let mut nou_valor = recipes[elf1] + recipes[elf2];
        if nou_valor >= 10 {
            recipes.push(nou_valor / 10);
            nou_valor %= 10;
            dosdigits = 1;
        }
        recipes.push(nou_valor);
        elf1 = (elf1 + 1 + recipes[elf1]) % recipes.len();
        elf2 = (elf2 + 1 + recipes[elf2]) % recipes.len();

        // Ara buscar-los (només si ha crescut prou perquè hi sigui)
        let llargada = recipes.len();
        if llargada > objectiu.len() {
            if &recipes[llargada - dosdigits - objectiu.len()..llargada - dosdigits]
                == &objectiu[..]
            {
                return llargada - dosdigits - objectiu.len();
            }
        }
    }
}

fn main() {
    let mut recipes: Vec<usize> = vec![3, 7];
    // Ho imprimeixo tot junt
    println!(
        "Solució 1: {}",
        problem1(074501, &mut recipes)
            .iter()
            .fold(String::new(), |acc, &num| acc + &num.to_string() + "")
    );

    // Segon problema
    let mut recipes2: Vec<usize> = vec![3, 7];
    println!(
        "Solució 2: {}",
        problem2(String::from("074501"), &mut recipes2)
    );
}
