use crate::bitboard::Board;
use crate::cmds::{Color, Move};

pub fn search(board: &Board) -> (u64, [(u64, u64); 4]) {
    let (mvs, hints) = board.legals();
    if mvs == 0 {
        (0, hints)
    } else {
        let mut mov: u64 = 0;
        for mv in (0..64).map(|i| 1 << i).filter(|&m| mvs & m == m) {
            mov = mv;
            break
        }
        (mov, hints)
    }
}