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

    const SIZE: usize = 2;
    let (soluciox, solucioy, _) = solucio1(SIZE, &graella);
    println!("Solucio 1 -> {},{}", soluciox, solucioy);

    let mut total = 0;
    let mut soluciox = 0;
    let mut solucioy = 0;
    let mut solucion = 0;

    for quadre in 2..=300 {
        let (candidatx, candidaty, max_valor) = solucio1(quadre, &graella);
        if max_valor > total {
            soluciox = candidatx;
            solucioy = candidaty;
            solucion = quadre;
            total = max_valor;
        }
    }
    println!("Solucio2 -> {},{},{}", soluciox, solucioy, solucion + 1);
}

fn solucio1(mida: usize, graella: &Vec<Vec<i32>>) -> (usize, usize, i32) {
    let mut max = 0;
    let mut soluciox = 0;
    let mut solucioy = 0;
    for fila in 0..graella.len() - mida {
        for columna in 0..graella[fila].len() - mida {
            let mut nou_valor = 0;

            for dy in 0..=mida {
                for dx in 0..=mida {
                    nou_valor = nou_valor + graella[fila + dy][columna + dx];
                }
            }

            // ocupacio.entry((columna, fila)).or_insert();
            if nou_valor > max {
                max = nou_valor;
                soluciox = columna;
                solucioy = fila;
            }
        }
    }
    (soluciox, solucioy, max)
}
