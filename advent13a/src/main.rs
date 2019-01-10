use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Casella {
    Res,
    Vertical,
    Horitzontal,
    Creuament,
    GiraDreta,
    GiraEsquerra,
}

impl FromStr for Casella {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Casella, Self::Err> {
        match s.as_bytes().get(0) {
            None => panic!("El circuit és buit?"),
            Some(&b' ') => Ok(Casella::Res),
            Some(&b'|') => Ok(Casella::Vertical),
            Some(&b'^') => Ok(Casella::Vertical),
            Some(&b'v') => Ok(Casella::Vertical),
            Some(&b'-') => Ok(Casella::Horitzontal),
            Some(&b'<') => Ok(Casella::Horitzontal),
            Some(&b'>') => Ok(Casella::Horitzontal),
            Some(&b'+') => Ok(Casella::Creuament),
            Some(&b'/') => Ok(Casella::GiraDreta),
            Some(&b'\\') => Ok(Casella::GiraEsquerra),
            Some(&b) => panic!("Això no hi hauria d'existir': 0x{:X}", b),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Fitxa {
    direccio: char,
    posicio: (usize, usize),
    girs: u8,
}

impl Fitxa {
    pub fn new(posicio: (usize, usize), direccio: char) -> Self {
        Fitxa {
            posicio,
            direccio,
            girs: 0,
        }
    }

    pub fn mou(&mut self) -> (usize, usize) {
        let (mut x, mut y) = self.posicio;
        match self.direccio {
            '>' => x = x + 1,
            '<' => x = x - 1,
            '^' => y = y - 1,
            'v' => y = y + 1,
            _ => (),
        }
        self.posicio = (x, y);
        self.posicio
    }

    fn horari(&mut self) {
        match self.direccio {
            '>' => self.direccio = 'v',
            '<' => self.direccio = '^',
            '^' => self.direccio = '>',
            'v' => self.direccio = '<',
            _ => (),
        }
    }

    fn antihorari(&mut self) {
        match self.direccio {
            '>' => self.direccio = '^',
            '<' => self.direccio = 'v',
            '^' => self.direccio = '<',
            'v' => self.direccio = '>',
            _ => (),
        }
    }

    pub fn endavant(&mut self, proper_caracter: Casella) {
        match proper_caracter {
            Casella::Creuament => {
                match self.girs {
                    0 => self.antihorari(),
                    2 => self.horari(),
                    _ => (),
                };
                self.girs = (self.girs + 1) % 3;
            }
            Casella::GiraEsquerra => match self.direccio {
                '>' | '<' => self.horari(),
                '^' | 'v' => self.antihorari(),
                _ => (),
            },
            Casella::GiraDreta => match self.direccio {
                '>' | '<' => self.antihorari(),
                '^' | 'v' => self.horari(),
                _ => (),
            },
            Casella::Res => {
                println!("MALAMENT, MALAMENT");
            }
            _ => (),
        }
    }
}

impl Ord for Fitxa {
    fn cmp(&self, other: &Self) -> Ordering {
        let (sx, sy) = self.posicio;
        let (ox, oy) = other.posicio;

        (sy, sx).cmp(&(oy, ox))
    }
}

impl PartialOrd for Fitxa {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Joc {
    mapa: Vec<Vec<Casella>>,
    fitxes: Vec<Fitxa>,
}

impl Joc {
    pub fn new() -> Joc {
        Joc {
            fitxes: Vec::new(),
            mapa: Vec::new(),
        }
    }

    fn carregar_mapa(&mut self, entrada: Vec<&str>) {
        let mut i = 0;
        for linia in entrada {
            let nova: Vec<String> = linia.chars().map(|c| c.to_string()).collect();

            let nova_linia: Vec<Casella> = nova
                .iter()
                .map(|c| Casella::from_str(&c).unwrap())
                .collect();

            for (x, c) in linia.chars().enumerate() {
                match c {
                    '^' | 'v' | '>' | '<' => {
                        self.fitxes.push(Fitxa::new((x, i), c));
                    }
                    _ => (),
                };
            }

            self.mapa.push(nova_linia.clone());
            i = i + 1;
        }
    }

    fn juga(&mut self) -> HashSet<(usize, usize)> {
        let mut colisions: HashSet<(usize, usize)> = HashSet::new();
        loop {
            // Ho vaig provar sense ordenar però no va funcionar
            self.fitxes.sort_unstable();
            let mut posicions: HashSet<(usize, usize)> = HashSet::new();
            for fitxa in &mut self.fitxes {
                // Comprova si han xocat amb la fitxa abans de moure-la
                // Per evitar que es saltin ...
                //  ...>....  ....>...
                //  ....<...  ...<....
                if posicions.contains(&fitxa.posicio) {
                    colisions.insert(fitxa.posicio);
                }
                let (x, y) = fitxa.mou();
                if posicions.contains(&(x, y)) {
                    colisions.insert((x, y));
                }
                posicions.insert((x, y));
                fitxa.endavant(self.mapa[y][x]);
            }

            if colisions.len() != 0 {
                break;
            }
        }
        colisions
    }
}

fn main() {
    let linies: Vec<_> = include_str!("input").lines().collect();
    let mut joc: Joc = Joc::new();
    joc.carregar_mapa(linies);

    println!("Resultat 1: {:?}", joc.juga());
}
