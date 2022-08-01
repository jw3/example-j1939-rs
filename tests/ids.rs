// https://www.csselectronics.com/pages/j1939-pgn-conversion-tool

use canparse::pgn::PgnLibrary;
use example_j1939_rs::pgn;

// socketcan parses this as the frame id
const LFE_CAN_ID: u32 = 0x18FEF200;
// pgn library identifies LFE as this id
const LFE_PGN_ID: u32 = 0xFEF2;
// the dbc file contains this literal id
const LFE_DBC_ID: u32 = 2566845182;

#[test]
fn conversions() {
    // CAN ID => PGN ID
    assert_eq!(pgn(LFE_CAN_ID), LFE_PGN_ID);

    // DBC ID => PGN ID
    assert_eq!(pgn(LFE_DBC_ID), LFE_PGN_ID);

    // given those two conversions you can compare CAN ID to DBC ID
    // which allows you to construct a list of interesting ids from the dbc
    // and filter the messages at runtime by can id based on that list
    assert_eq!(pgn(LFE_CAN_ID), pgn(LFE_DBC_ID));
}

#[test]
fn lookup_lfe_from_pgnid() {
    let lib = PgnLibrary::from_dbc_file("data/j1939_utf8.dbc").unwrap();
    let lfe_def = lib.get_pgn(LFE_PGN_ID);
    assert!(lfe_def.is_some());
    println!("{lfe_def:?}")
}

#[test]
fn lookup_lfe_from_dbc_id() {
    // map to pgn first
    let pgn_id = pgn(LFE_DBC_ID);

    let lib = PgnLibrary::from_dbc_file("data/j1939_utf8.dbc").unwrap();
    let lfe_def = lib.get_pgn(pgn_id);
    assert!(lfe_def.is_some());
    println!("{lfe_def:?}")
}
