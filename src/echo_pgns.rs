use canparse::pgn::{ParseMessage, PgnLibrary, SpnDefinition};
use example_j1939_rs::pgn;

// canplayer -v -g 100 -I data/lfe0.log vcan0=can0
fn main() {
    let lib = PgnLibrary::from_dbc_file("data/j1939_utf8.dbc").unwrap();

    // todo;; externalize socket address
    let rx = socketcan::CANSocket::open("vcan0").expect("could not open vcan");

    println!("listening...");
    loop {
        let frame = rx.read_frame().unwrap();
        let pgnid = pgn(frame.id());
        if let Some(def) = lib.get_pgn(pgnid) {
            println!("{def:?}");
        }
    }
}
