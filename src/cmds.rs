#[derive(Debug)]
pub enum Color {
    Empty,
    White,
    Black,
}

#[derive(Debug)]
pub enum Res {
    Win,
    Lose,
    Tie,
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    To(i32, i32),
    Pass,
}

pub fn move_to_string(m : Move) -> String {
    match m {
        Move::Pass => "PASS".to_string(),
        Move::To(i, j) => {
            let ci = (i + ('A' as i32) - 1) as u8 as char;
            let cj = (j + ('1' as i32) - 1) as u8 as char;
            ci.to_string() + cj.to_string().as_str()
        }
    }
}

pub fn idx_to_move(mv: &u64) -> Move {
    if *mv == 0 {
        Move::Pass
    } else {
        Move::To((8 - mv.trailing_zeros() % 8) as i32, (8 - mv.trailing_zeros() / 8) as i32)
    }
}

pub fn move_to_idx(mv: &Move) -> u64 {
    match mv {
        Move::Pass => 0,
        Move::To(i, j) => {
            (1 as u64) << ((8 - j) * 8 + (8 - i))
        }
    }
}

#[derive(Debug)]
pub enum Cmd {
    Open(String),
    Start(Color, String, i32),
    End(Res, i32, i32, String),
    Bye(Vec<(String, (i32, i32, i32))>),
    Move(Move),
    Ack(i32),
}