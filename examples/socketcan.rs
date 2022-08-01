use canparse::pgn::{ParseMessage, PgnLibrary, SpnDefinition};
use example_j1939_rs::pgn;

// canplayer -v -g 100 -I data/lfe0.log vcan0=can0
fn main() {
    let lib = PgnLibrary::from_dbc_file("data/j1939_utf8.dbc").unwrap();

    // open the virtual socket
    let rx = socketcan::CANSocket::open("vcan0").expect("could not open vcan");

    // the target signals
    let avg_fuel_rate: &SpnDefinition = lib.get_spn("EngAverageFuelEconomy").unwrap();
    let instant_fuel_rate: &SpnDefinition = lib.get_spn("EngInstantaneousFuelEconomy").unwrap();

    // pgns containing the signals
    let listening_for = vec![pgn(2566845182_u32)];

    println!("listening...");
    loop {
        let frame = rx.read_frame().unwrap();
        if listening_for.contains(&pgn(frame.id())) {
            let x = avg_fuel_rate.parse_message(frame.data()).unwrap();
            let y = instant_fuel_rate.parse_message(frame.data()).unwrap();
            println!("AVG FUEL RATE: {}, INST FUEL RATE {}", x, y);
        } else {
            println!("skipped {} => {}", frame.id(), pgn(frame.id()));
        }
    }
}
