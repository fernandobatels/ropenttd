# OpenTTD reader

[![Crate](https://img.shields.io/crates/v/ropenttd.svg)](https://crates.io/crates/ropenttd)
[![API](https://docs.rs/ropenttd/badge.svg)](https://docs.rs/ropenttd)

A demonstration crate to read the [OpenTTD](https://github.com/OpenTTD/OpenTTD) save files. Only use this project with a backup copy of your saves.

## Examples

Default company info
``` bash
cargo run company saves/example1.sav
 name                 inaugurated_year   president   currency   money     loan
 Petfield Transport   1950               D. Nelson   GBP        3647337   0
```

## TODO

- [X] load save game ottx
- [X] parse the main company
- [ ] parse the others companies
- [ ] parse the trains
- [ ] parse the planes
- [ ] parse the trucks
- [ ] parse the ships
- [ ] parse the economy
- [ ] parse the map
- [ ] parse the cities
- [ ] parse the industries
- [ ] parse the stations
- [ ] parse the ports
- [ ] parse the airports
- [ ] parse the roads
- [ ] parse the rails
- [ ] render the map in some way
- [ ] support old save games
- [ ] support other formats(ottd, ottn..)
- [X] lib mode
- [X] cli mode
