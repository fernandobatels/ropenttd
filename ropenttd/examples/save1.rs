//! Save 1 example

use ropenttd::*;

fn main() -> Result<(), Error> {

    let mut sv = SaveGame::load_from_file("saves/example1.sav".to_string())?;

    let cp = sv.company()?;

    println!("{:?}", cp);

    Ok(())
}
