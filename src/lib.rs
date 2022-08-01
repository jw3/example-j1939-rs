// CanId{ DP, PF, PS, SA } => Pgn{ PF, PS }
pub fn pgn(long_id: u32) -> u32 {
    (long_id >> 8) & 0x1FFFF
}
