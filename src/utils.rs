use std::cell::OnceCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

const PREMIATI: [u8; 11] = [1, 2, 2, 2, 3, 3, 3, 3, 3, 3, 4];

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
    primo: [usize; 11],
    secondo: [usize; 11],
    terzo: [usize; 11],
    quarto: [usize; 11],
    socio: bool,
    giocati: usize,
}

impl Player {
    const PREMI: [[usize; 4]; 11] = [
        [15, 0, 0, 0],
        [20, 10, 0, 0],
        [20, 15, 0, 0],
        [25, 15, 0, 0],
        [25, 15, 10, 0],
        [30, 15, 10, 0],
        [30, 25, 10, 0],
        [35, 25, 10, 0],
        [35, 25, 10, 10],
        [40, 25, 10, 10],
        [40, 25, 15, 10],
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
    pub fn tot_premi(&self) -> usize {
        self.primo
            .iter()
            .zip(Self::PREMI.iter())
            .map(|(num_coppie, premi)| num_coppie * premi[0])
            .sum::<usize>()
            + self
                .secondo
                .iter()
                .zip(Self::PREMI.iter())
                .map(|(num_coppie, premi)| num_coppie * premi[1])
                .sum::<usize>()
            + self
                .terzo
                .iter()
                .zip(Self::PREMI.iter())
                .map(|(num_coppie, premi)| num_coppie * premi[2])
                .sum::<usize>()
            + self
                .quarto
                .iter()
                .zip(Self::PREMI.iter())
                .map(|(num_coppie, premi)| num_coppie * premi[2])
                .sum::<usize>()
    }
    fn premia(&mut self, posizione: usize, partecipanti: usize) {
        dbg!(partecipanti);
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
    pub fn as_record(&self) -> String {
        let socio = match self.socio {
            true => "Yes",
            false => "No",
        };

        let primo = self.primo.iter().sum::<usize>();
        let secondo = self.secondo.iter().sum::<usize>();
        let terzo = self.terzo.iter().sum::<usize>();
        let quarto = self.quarto.iter().sum::<usize>();
        let tot_premi = self.tot_premi();
        let tot_giocati = self.giocati;
        let ev = (tot_premi as f64 - 5f64 * tot_giocati as f64) / tot_giocati as f64;
        format!(
            "{}, {}, {}, {}, {}, {}, {}, {}, {8:.2}",
            self.nome, socio, primo, secondo, terzo, quarto, tot_giocati, tot_premi, ev
        )
    }
}

#[derive(Debug)]
pub struct Tournament<'a> {
    codice: String,
    pub posizioni: Vec<[&'a str; 2]>,
}

impl<'a> Tournament<'a> {
    pub fn new(codice: &str) -> Self {
        Self {
            codice: codice.to_string(),
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
