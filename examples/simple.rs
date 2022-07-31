use canparse::pgn::{PgnLibrary, SpnDefinition, ParseMessage};

fn main() {
    // long id to pgn
    let pgn: u32 = (2566845182 >> 8) & 0x1FFFF;
    println!("{pgn}");

    let lib = PgnLibrary::from_dbc_file("data/j1939_utf8.dbc").unwrap();

    let fuel_consumption = lib.get_pgn(pgn).unwrap();
    dbg!(fuel_consumption);

    let engine_avg_fuel_rate_def: &SpnDefinition = lib
        .get_spn("EngAverageFuelEconomy").unwrap();

    let engine_instant_fuel_rate_def: &SpnDefinition = lib
        .get_spn("EngInstantaneousFuelEconomy").unwrap();

    let engine_fuel_rate_def: &SpnDefinition = lib
        .get_spn("EngFuelRate").unwrap();

    dbg!(engine_fuel_rate_def);

    // Format ------>  |Fuel rate|-| Instant |-| Average |Throttle Pos|
    let msg: [u8; 8] = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77];

    let engine_avg_fuel_rate: f32 = engine_avg_fuel_rate_def.parse_message(&msg).unwrap();
    let engine_instant_fuel_rate: f32 = engine_instant_fuel_rate_def.parse_message(&msg).unwrap();
    let engine_fuel_rate: f32 = engine_fuel_rate_def.parse_message(&msg).unwrap();

    println!("Engine average fuel rate: {}", engine_avg_fuel_rate);
    println!("Engine instant fuel rate: {}", engine_instant_fuel_rate);
    println!("Engine fuel rate: {}", engine_fuel_rate);
}
