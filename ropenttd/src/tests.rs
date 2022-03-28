//! Tests with real save files

use crate::*;

#[test]
fn save1() -> Result<(), Error> {

    let mut sv = SaveGame::load_from_file("saves/example1.sav".to_string())?;

    let cp = sv.company()?;

    assert_eq!(CompanyInfo {
        name: "Petfield Transport".to_string(),
        president: "D. Nelson".to_string(),
        money: Money {
            original: 3_647_337,
            value: 3_647_337,
            currency: Currency {
                name: "GBP",
                exchange_rate: 1
            }
        },
        loan: Money {
            original: 0,
            value: 0,
            currency: Currency {
                name: "GBP",
                exchange_rate: 1
            }
        },
        inaugurated_year: 1950
    }, cp);

    Ok(())
}
