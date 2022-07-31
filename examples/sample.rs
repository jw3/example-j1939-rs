// from https://github.com/jmagnuson/canparse/blob/master/tests/pgnlibrary.rs

extern crate canparse;

use canparse::pgn::{PgnLibrary, SpnDefinition, ParseMessage};

fn main() {

    // Parse dbc file into PgnLibrary
    let lib = PgnLibrary::from_dbc_file("data/sample.dbc").unwrap();

    // Pull signal definition for engine speed
    let enginespeed_def: &SpnDefinition = lib
        .get_spn("Engine_Speed").unwrap();

    // Parse frame containing engine speed
    let msg: [u8; 8] = [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
    let engine_speed: f32 = enginespeed_def.parse_message(&msg).unwrap();

    assert_eq!(engine_speed , 2728.5);
    println!("Engine speed: {}", engine_speed);
}
