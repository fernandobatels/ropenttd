//! Save 1 example

use std::io::BufReader;
use std::fs::File;

use openttd::SaveGame;

fn main() -> Result<(), String> {

    let f = File::open("saves/example1.sav")
        .map_err(|e| e.to_string())?;

    let _ = SaveGame::load(BufReader::new(f))
        .map_err(|e| e.to_string())?;

    Ok(())
}
