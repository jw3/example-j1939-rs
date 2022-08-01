use canparse::pgn::PgnLibrary;
use clap::Parser;
use example_j1939_rs::pgn;
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

#[derive(Parser)]
#[clap(name = "CanEchoPGN", version = GIT_VERSION)]
pub struct Opts {
    #[clap(long, default_value = "data/j1939_utf8.dbc")]
    pub dbc: String,

    #[clap(default_value = "vcan0")]
    pub socket: String,
}

// canplayer -v -g 100 -I data/lfe0.log vcan0=can0
fn main() {
    let opts: Opts = Opts::parse();

    let lib = PgnLibrary::from_dbc_file(opts.dbc).unwrap();
    let rx = socketcan::CANSocket::open(&opts.socket).expect("could not open vcan");

    println!("listening...");
    loop {
        let frame = rx.read_frame().unwrap();
        let pgnid = pgn(frame.id());
        if let Some(def) = lib.get_pgn(pgnid) {
            println!("{def:?}");
        }
    }
}
