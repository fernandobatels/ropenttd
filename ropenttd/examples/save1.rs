//! Save 1 example

use std::io::BufReader;
use std::fs::File;

use ropenttd::*;

fn main() -> Result<(), String> {

    let f = File::open("saves/example1.sav")
        .map_err(|e| e.to_string())?;
    let mut bfr = BufReader::new(f);

    let mut sv = SaveGame::load(&mut bfr)
        .map_err(|e| e.to_string())?;

    let cp = sv.company()
        .map_err(|e| e.to_string())?;

    println!("{}", cp.name);

    Ok(())
}
