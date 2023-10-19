mod utils;
use std::io;
use std::{collections::hash_map::Entry, fs::File};

use reqwest::{blocking::Client, redirect::Policy};
use scraper::{Html, Selector};
use utils::*;
const BASE: &str = "https://www.federbridge.it/punti/TouSimGrpWeb.asp?GrpCode=f0784";
const CODICE_CIRCOLO: &str = "[F0784]";

fn main() {
    let client = Client::builder().redirect(Policy::none()).build().unwrap();
    let body = client.get(BASE).send().unwrap().text().unwrap();
    let document = Html::parse_document(&body);
    let tr_selector = Selector::parse("tr.ALTbase20").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let pair_selector = Selector::parse("tr.BGCLibere.ALTbase25,tr.BGCTDLibere.ALTbase25").unwrap();
    let code_selector = Selector::parse("td.COLceleste").unwrap();
    let player_selector = Selector::parse("td.Capitalize.POSbase0").unwrap();
    let mixed_selector =
        Selector::parse("td.Capitalize.POSbase0, td.COLceleste, td.Capitalize.POSbase0>span")
            .unwrap();
    let mut stringa = String::new();
    let mut circolo = Circolo::new();
    for selected in document.select(&tr_selector) {
        //stdin.read_line(&mut stringa).unwrap();
        let codice_torneo = selected.text().nth(3).unwrap();
        println!("{}", codice_torneo);
        let mut tournament = Tournament::new(codice_torneo);
        let link = selected
            .select(&a_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();
        let tabella = client.get(link).send().unwrap().text().unwrap();
        let parsata = Html::parse_document(&tabella);
        //let anagrafica: HashMap<String, String> = HashMap::new();
        for (posizione, coppia) in parsata.select(&pair_selector).enumerate() {
            let mut copia = coppia
                .select(&mixed_selector)
                .map(|node| node.text().next().unwrap().trim());
            copia.clone().for_each(|t| println!("{t}"));
            let codice1 = copia.by_ref().next().expect("codice giocatore non trovato");
            let nome = copia.by_ref().next().expect("nome non trovato");
            let socio = copia.by_ref().next().expect("codice circolo non trovato");
            circolo.presente_o_inserisci(codice1, nome, socio == CODICE_CIRCOLO);
            let codice2 = copia.by_ref().next().expect("codice giocatore non trovato");
            let nome = copia.by_ref().next().expect("nome non trovato");
            let socio = copia.by_ref().next().expect("codice circolo non trovato");
            let coppia = [codice1, codice2];
            circolo.presente_o_inserisci(codice2, nome, socio == CODICE_CIRCOLO);
            tournament.posizioni.push(coppia);

            // for testo in coppia.select(&mixed_selector) {
            //     println!("{}", testo.text().next().unwrap().trim());
            // }
            //for real_code in code_elem {
            //    println!("{}", real_code.text().next().unwrap());
            //}
            //for real_player in player_elem {
            //    println!("{}", real_player.text().next().unwrap());
            //}
        }
        tournament.dai_premi(&mut circolo);
    }
    for player in circolo.values() {
        println!("{} => {}", player.nome, player.tot_premi());
    }
    let file = File::create("Tornei.csv").unwrap();
    let mut wtr = csv::WriterBuilder::new().delimiter(b';').from_writer(file);

    // When writing records without Serde, the header record is written just
    // like any other record.
    wtr.write_record(&[
        "Giocatore",
        "Socio",
        "Primo",
        "Secondo",
        "Terzo",
        "Quarto",
        "Tot Tornei",
        "Tot Premi",
        "EV",
    ])
    .unwrap();
    for player in circolo.values() {
        let record = player.as_record();
        wtr.write_record(
            std::convert::TryInto::<[&str; 9]>::try_into(record.split(", ").collect::<Vec<&str>>())
                .unwrap(),
        )
        .unwrap();
    }
    wtr.flush().unwrap();
}
