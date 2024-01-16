//! OpenTTD reader

use argopt::{cmd_group, subcmd};
use tabled::{Style, Table, Tabled};

use ropenttd::*;

#[cmd_group(commands = [company])]
fn main() -> Result<(), Error> {}

/// Details the main/default company
#[subcmd]
fn company(
    file: String,
) -> Result<(), Error> {
    let mut sv = SaveGame::load_from_file(file)?;

    let co = sv.company()?;

    let data = vec![
        CompanyPrintable {
            name: co.name.clone(),
            president: co.president.clone(),
            currency: co.money.currency.name.to_string(),
            money: co.money.value,
            loan: co.loan.value,
            inaugurated_year: co.inaugurated_year,
        }
    ];

    let printable = Table::new(data).with(Style::blank());

    println!("{}", printable);

    Ok(())
}

#[derive(Tabled)]
struct CompanyPrintable {
    pub name: String,
    pub inaugurated_year: u32,
    pub president: String,
    pub currency: String,
    pub money: i64,
    pub loan: i64,
}
