use std::collections::hash_map::Entry;
use std::collections::HashMap;

//const PREMIATI: [u8; 11] = [1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4];

pub struct Circolo {
    circolo: HashMap<String, Player>,
}

impl std::ops::Deref for Circolo {
    type Target = HashMap<String, Player>;
    fn deref(&self) -> &Self::Target {
        &self.circolo
    }
}

impl std::ops::DerefMut for Circolo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.circolo
    }
}
impl Circolo {
    pub fn new() -> Self {
        Self {
            circolo: HashMap::new(),
        }
    }

    pub fn presente_o_inserisci(&mut self, codice: &str, nome: &str, socio: bool) {
        match self.circolo.entry(codice.to_string()) {
            Entry::Occupied(_) => {}
            Entry::Vacant(e) => {
                let player = Player::new(nome.to_string(), socio);
                e.insert(player);
            }
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub nome: String,
    pub primo: [u32; 11],
    pub secondo: [u32; 11],
    pub terzo: [u32; 11],
    pub quarto: [u32; 11],
    pub socio: bool,
    pub giocati: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.nome == other.nome
    }
}

impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.socio != other.socio {
            if self.socio {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            self.nome.cmp(&other.nome)
        }
    }
}

impl Player {
    const PREMI: [[f32; 11]; 4] = [
        // 5   6    7   8     9     10    11    12    13    14    15
        // Primo
        [5.0, 5.0, 5.0, 5.0, 10.0, 10.0, 10.0, 10.0, 10.0, 15.0, 15.0],
        // Secondo
        [0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 5.0, 5.0, 5.0, 5.0],
        // Terzo
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.0, 5.0, 5.0, 5.0],
        // Quarto
        [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    ];
    pub fn new(nome: String, socio: bool) -> Self {
        Self {
            nome,
            primo: [0; 11],
            secondo: [0; 11],
            terzo: [0; 11],
            quarto: [0; 11],
            socio,
            giocati: 0,
        }
    }
    pub fn tot_premi(&self) -> f32 {
        self.primo
            .iter()
            .zip(Self::PREMI.first().unwrap().iter())
            .fold(0.0, |acc, (num_coppie, premio)| {
                acc + f32::from(*num_coppie as u8) * premio
            })
            + self
                .secondo
                .iter()
                .zip(Self::PREMI.iter().nth(1).unwrap().iter())
                .fold(0.0, |acc, (num_coppie, premio)| {
                    acc + f32::from(*num_coppie as u8) * premio
                })
            + self
                .terzo
                .iter()
                .zip(Self::PREMI.iter().nth(2).unwrap().iter())
                .fold(0.0, |acc, (num_coppie, premio)| {
                    acc + f32::from(*num_coppie as u8) * premio
                })
            + self
                .quarto
                .iter()
                .zip(Self::PREMI.iter().nth(3).unwrap().iter())
                .fold(0.0, |acc, (num_coppie, premio)| {
                    acc + f32::from(*num_coppie as u8) * premio
                })
    }
    fn premia(&mut self, posizione: usize, partecipanti: usize) {
        match posizione {
            0 => self.primo[partecipanti - 6] += 1,
            1 => self.secondo[partecipanti - 6] += 1,
            2 => self.terzo[partecipanti - 6] += 1,
            3 => self.quarto[partecipanti - 6] += 1,
            _ => {}
        }
    }
    pub fn gioca(&mut self) {
        self.giocati += 1;
    }
    #[allow(dead_code)]
    pub fn as_record(&self) -> String {
        let socio = match self.socio {
            true => "Yes",
            false => "No",
        };

        let primo = self.primo.iter().sum::<u32>();
        let secondo = self.secondo.iter().sum::<u32>();
        let terzo = self.terzo.iter().sum::<u32>();
        let quarto = self.quarto.iter().sum::<u32>();
        let tot_premi = self.tot_premi();
        let tot_giocati = self.giocati;
        let ev = (tot_premi as f64 - 5f64 * tot_giocati as f64) / tot_giocati as f64;
        format!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {8:.2}",
            self.nome, socio, primo, secondo, terzo, quarto, tot_giocati, tot_premi, ev
        )
    }
    pub fn nome(&self) -> &str {
        &self.nome
    }
}

#[derive(Debug)]
pub struct Tournament<'a> {
    _codice: String,
    pub posizioni: Vec<[&'a str; 2]>,
}

impl<'a> Tournament<'a> {
    pub fn new(codice: &str) -> Self {
        Self {
            _codice: codice.to_string(),
            posizioni: Vec::new(),
        }
    }

    pub fn dai_premi(&self, circolo: &mut Circolo) {
        let partecipanti = self.posizioni.len();
        self.posizioni
            .iter()
            .enumerate()
            .for_each(|(posizione, coppia)| {
                let primo = circolo.get_mut(coppia[0]).unwrap();
                primo.premia(posizione, partecipanti);
                primo.gioca();
                let secondo = circolo.get_mut(coppia[1]).unwrap();
                secondo.premia(posizione, partecipanti);
                secondo.gioca();
            })
    }
}
