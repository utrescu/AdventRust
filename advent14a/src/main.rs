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

fn main() {
    let mut recipes: Vec<usize> = vec![3, 7];
    // Ho imprimeixo tot junt
    println!(
        "Solució 1: {}",
        problem1(074501, &mut recipes)
            .iter()
            .fold(String::new(), |acc, &num| acc + &num.to_string() + "")
    );
}
