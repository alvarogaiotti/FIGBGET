mod excelwriter;
mod selectors;
mod utils;
use chrono::prelude::*;
use rust_xlsxwriter::{Color, Format, Workbook};

use excelwriter::*;

use std::sync::{mpsc::Sender, Arc};

use scraper::{ElementRef, Html};
use selectors::*;
use ureq::AgentBuilder;
use utils::*;
const BASE: &str = "https://www.federbridge.it/punti/TouSimGrpWeb.asp?GrpCode=f0784";
const CODICE_CIRCOLO: &str = "[F0784]";

fn get_client() -> ureq::Agent {
    // With reqwest
    // let client = Client::builder().redirect(Policy::none()).build().unwrap();
    // let body = client.get(BASE).send().unwrap().text().unwrap();
    let tls_config = Arc::new(native_tls::TlsConnector::new().unwrap());
    AgentBuilder::new().user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.3").tls_connector(tls_config).build()
}

pub fn download_report(start: NaiveDate, end: NaiveDate, sender: Sender<f32>) {
    let intervallo_temporale = Intervallo { start, end };
    let giocatore_selector = CodeNameCircoloSelector::new();
    let mut circolo = Circolo::new();
    let client = get_client();
    let body = if let Ok(body) = client.get(BASE).call() {
        body.into_string().unwrap()
    } else {
        sender.send(-1.0).unwrap();
        return;
    };
    let document = Html::parse_document(&body);
    let selettore_torneo = TournamentSelector::new();
    let tornei = document.select(&selettore_torneo);
    let mut cumulativo_coppie = 0;
    let tornei: Vec<ElementRef<'_>> = tornei.collect();
    let tot_tornei = tornei.len();
    let mut tornei_in_target = 0;

    for (numero_torneo, selected) in tornei.into_iter().enumerate() {
        // Invia al thread principale lo stato dell'avanzamento
        sender
            .send(((numero_torneo + 1) as f32 / tot_tornei as f32).clamp(0.0, 1.0))
            .unwrap();
        let codice_torneo = selected.text().nth(3).unwrap();
        let mut tournament = Tournament::new(codice_torneo);
        let link_element = selected
            .select(&TournamentPageSelector::new())
            .next()
            .unwrap();
        let date_in_html = link_element
            .select(&TournamentDateSelector::new())
            .next()
            .unwrap()
            .inner_html();
        let data_torneo = parse_date(date_in_html);
        match intervallo_temporale.comprende(data_torneo) {
            // The page starts from the top, so we get the latest tournaments first
            // hence, when we continue if we find tournaments with a date later
            // than our target and we break once we find a tournament played
            // before our target date.
            std::cmp::Ordering::Less => {
                sender.send(1.0).unwrap();
                break;
            }
            std::cmp::Ordering::Greater => continue,
            std::cmp::Ordering::Equal => {
                tornei_in_target += 1;
            }
        }

        let link = link_element.value().attr("href").unwrap();
        let tabella = client.get(link).call().unwrap().into_string().unwrap();
        let parsata = Html::parse_document(&tabella);
        for coppia in parsata.select(&PairSelector::new()) {
            cumulativo_coppie += 1;
            let mut copia = coppia
                .select(&giocatore_selector)
                .map(|node| node.text().next().unwrap().trim());
            let (codice1, nome, socio) = get_player_data(&mut copia);
            circolo.presente_o_inserisci(codice1, nome, socio == CODICE_CIRCOLO);
            let (codice2, nome, socio) = get_player_data(&mut copia);
            let coppia = [codice1, codice2];
            circolo.presente_o_inserisci(codice2, nome, socio == CODICE_CIRCOLO);
            tournament.posizioni.push(coppia);
        }
        tournament.dai_premi(&mut circolo);
    }
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let wtr = workbook.add_worksheet();

    // When writing records without Serde, the header record is written just
    // like any other record.
    write_header(wtr).unwrap();
    let mut glob_row = 0;
    let mut giocatori_ordinati: Vec<_> = circolo.values().collect();
    giocatori_ordinati.sort_unstable();
    let mut sum = 0.0;
    for (row, player) in giocatori_ordinati.iter().enumerate() {
        sum += player.tot_premi();
        write_player_record_to_worksheet(player, wtr, row as u32 + 1).unwrap();
        glob_row = row + 1;
    }
    let glob_row = glob_row as u32 + 1;
    wtr.write(glob_row + 1, 0, "Tot tornei").unwrap();
    wtr.write(glob_row + 1, 1, tornei_in_target as u32).unwrap();
    wtr.write(glob_row + 1, 2, "Tot coppie").unwrap();
    wtr.write(glob_row + 1, 3, cumulativo_coppie).unwrap();
    wtr.write(glob_row + 1, 4, "Tot incasso").unwrap();
    wtr.write(glob_row + 1, 5, cumulativo_coppie * 10).unwrap();
    wtr.write(glob_row + 2, 2, "Tot premi").unwrap();
    wtr.write(glob_row + 2, 3, sum).unwrap();
    wtr.write(glob_row + 2, 4, "Tot carte").unwrap();
    wtr.write(glob_row + 2, 5, tornei_in_target as u32 * 15)
        .unwrap();
    wtr.write(glob_row + 2, 6, "Tot quota FIGB").unwrap();
    wtr.write(glob_row + 2, 7, f64::from(cumulativo_coppie) * 1.3)
        .unwrap();

    let tot_incasso = f64::from(cumulativo_coppie) * (10.0 - 1.3)
        - f64::from(tornei_in_target as u32 * 15)
        - f64::from(sum);
    let format = Format::new().set_bold().set_font_color(Color::Red);
    wtr.write_with_format(glob_row + 3, 2, "Incasso netto", &format)
        .unwrap();
    wtr.write_with_format(glob_row + 3, 3, tot_incasso, &format)
        .unwrap();
    wtr.write(glob_row + 4, 2, "Media incasso torneo").unwrap();
    wtr.write(
        glob_row + 4,
        3,
        f64::trunc(tot_incasso / f64::from(tornei_in_target as u32) * 100.0) / 100.0,
    )
    .unwrap();
    wtr.autofit();
    workbook
        .save(format!("Tornei {}-{}.xlsx", start, end))
        .unwrap();

    assert!(f64::from(sum) < f64::from(cumulativo_coppie * 10));
}

fn get_player_data<'a>(
    copia: &mut std::iter::Map<
        scraper::element_ref::Select<'a, '_>,
        impl FnMut(ElementRef<'a>) -> &'a str,
    >,
) -> (&'a str, &'a str, &'a str) {
    let codice = copia.by_ref().next().expect("codice giocatore non trovato");
    let nome = copia.by_ref().next().expect("nome non trovato");
    let socio = copia.by_ref().next().expect("codice circolo non trovato");
    (codice, nome, socio)
}

fn parse_date(stringa_data: String) -> NaiveDate {
    let data = std::convert::TryInto::<[&str; 2]>::try_into(
        stringa_data.split("&nbsp;").skip(1).collect::<Vec<&str>>(),
    )
    .unwrap();
    let mese = match data[1] {
        "gennaio" => 1,
        "febbraio" => 2,
        "marzo" => 3,
        "aprile" => 4,
        "maggio" => 5,
        "giugno" => 6,
        "luglio" => 7,
        "agosto" => 8,
        "settembre" => 9,
        "ottobre" => 10,
        "novembre" => 11,
        "dicembre" => 12,
        _ => panic!("parsing mese torneo errato"),
    };

    let year = chrono::Utc::now().year();

    NaiveDate::from_ymd_opt(year, mese, data[0].parse().unwrap()).unwrap()
}

pub struct Intervallo {
    start: NaiveDate,
    end: NaiveDate,
}

impl Intervallo {
    pub fn comprende(&self, data: NaiveDate) -> std::cmp::Ordering {
        if self.start > data {
            std::cmp::Ordering::Less
        } else if self.end < data {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
