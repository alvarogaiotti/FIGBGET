use rust_xlsxwriter::*;

use crate::utils::Player;

pub fn write_header(
    worksheet: &mut Worksheet,
) -> Result<&mut rust_xlsxwriter::Worksheet, rust_xlsxwriter::XlsxError> {
    worksheet.write_row(
        0,
        0,
        [
            "Giocatore",
            "Socio",
            "Primo",
            "Secondo",
            "Terzo",
            "Quarto",
            "Tot Premi",
            "Tot Tornei",
            "EV",
        ],
    )
}

pub fn write_player_record_to_worksheet<'a>(
    player: &Player,
    worksheet: &'a mut Worksheet,
    row: u32,
) -> Result<&'a mut rust_xlsxwriter::Worksheet, rust_xlsxwriter::XlsxError> {
    worksheet.write(row, 0, player.nome())?;

    let socio = match player.socio {
        true => "Yes",
        false => "No",
    };
    worksheet.write(row, 1, socio)?;

    let primo = player.primo.iter().sum::<u32>();
    worksheet.write(row, 2, primo)?;
    let secondo = player.secondo.iter().sum::<u32>();
    worksheet.write(row, 3, secondo)?;
    let terzo = player.terzo.iter().sum::<u32>();
    worksheet.write(row, 4, terzo)?;
    let quarto = player.quarto.iter().sum::<u32>();
    worksheet.write(row, 5, quarto)?;
    let tot_premi = player.tot_premi();
    worksheet.write(row, 6, tot_premi)?;
    let tot_giocati = player.giocati;
    worksheet.write(row, 7, tot_giocati)?;

    let ev = (tot_premi as f64 - 5f64 * tot_giocati as f64) / tot_giocati as f64;
    worksheet.write(row, 8, f64::trunc(ev * 100.0) / 100.0)
}
