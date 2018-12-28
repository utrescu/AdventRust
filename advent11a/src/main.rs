// use std::collections::HashMap;

fn calcula(y: i32, x: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    (((rack_id * y + serial) * rack_id / 100) % 10) - 5
}

fn main() {
    let input = 3628;
    let mut graella = vec![vec![0; 300]; 300];

    // Generar la graella de valors
    for fila in 0..graella.len() {
        for columna in 0..graella[fila].len() {
            graella[fila][columna] = calcula(fila as i32, columna as i32, input);
        }
    }

    // Ara buscar el pack 3x3 que suma més
    // let mut ocupacio: HashMap<(usize, usize), usize> = HashMap::new();
    let mut max = -1;
    let mut soluciox = 0;
    let mut solucioy = 0;

    for fila in 0..graella.len() - 2 {
        for columna in 0..graella[fila].len() - 2 {
            let nou_valor = graella[fila][columna]
                + graella[fila + 1][columna]
                + graella[fila + 2][columna]
                + graella[fila][columna + 1]
                + graella[fila + 1][columna + 1]
                + graella[fila + 2][columna + 1]
                + graella[fila][columna + 2]
                + graella[fila + 1][columna + 2]
                + graella[fila + 2][columna + 2];

            // ocupacio.entry((columna, fila)).or_insert();
            if nou_valor > max {
                max = nou_valor;
                soluciox = columna;
                solucioy = fila;
            }
        }
    }

    println!("{},{} = {}", soluciox, solucioy, max);
}
