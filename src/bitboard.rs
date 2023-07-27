#[derive(Clone, Debug)]
pub struct Board {
    black: u64,
    white: u64,
}

#[inline]
pub fn put(bits: u64, r: u8, c: u8) -> u64 {
    bits | (1 << (r * 8 + c))
}

#[inline]
pub fn get(bits: u64, r: u8, c: u8) -> bool {
    bits >> (r * 8 + c) & 1 == 1
}

impl Board {
    const MASKS: [(u8, u64); 4] = [
        (1, 0x7e7e7e7e7e7e7e7e),
        (8, 0x00ffffffffffff00),
        (7, 0x007e7e7e7e7e7e00),
        (9, 0x007e7e7e7e7e7e00)
    ];

    pub fn new() -> Board {
        Board{black: put(put(0, 3, 4), 4, 3),
              white: put(put(0, 3, 3), 4, 4)}
    }

    pub fn print(&self) {
        println!(" |A B C D E F G H");
        println!("-+---------------");
        for i in 0..8 {
            print!("{}|", i + 1);
            for j in 0..8 {
                let mask = (1 as u64) << ((7 - i) * 8 + (7 - j));
                if self.black & mask != 0 {
                    print!("X ");
                } else if self.white & mask != 0 {
                    print!("O ");
                } else {
                    print!("  ");
                }
            }
            println!();
        }
    }

    pub fn exchange(&mut self) {
        let tmp = self.white.clone();
        self.white = self.black.clone();
        self.black = tmp;
    }

    pub fn count(&self) -> (u32, u32) {
        (self.black.count_ones(), self.white.count_ones())
    }

    pub fn legals(&self) -> (u64, [(u64, u64); 4]) {
        let blank: u64 = !(self.black.clone() | self.white.clone());
        let mut legals: u64 = 0;
        let mut hints: [(u64, u64); 4] = [(0, 0); 4];

        #[inline]
        fn calc_legal(white: &u64, black: &u64, shift: &u8, mask: &u64) -> (u64, u64) {
            let w = &(white & mask);
            let t1 = &(black >> shift & w);
            let t2 = &(black << shift & w);
            let t1 = &(t1 | w & (t1 >> shift));
            let t2 = &(t2 | w & (t2 << shift));
            let t1 = &(t1 | w & (t1 >> shift));
            let t2 = &(t2 | w & (t2 << shift));
            let t1 = &(t1 | w & (t1 >> shift));
            let t2 = &(t2 | w & (t2 << shift));
            let t1 = &(t1 | w & (t1 >> shift));
            let t2 = &(t2 | w & (t2 << shift));
            let t1 = t1 | w & (t1 >> shift);
            let t2 = t2 | w & (t2 << shift);
            ((t1 >> shift), (t2 << shift))
        }

        for i in 0..4 {
            let idx = i as usize;
            hints[idx] = calc_legal(&self.white, &self.black, &Board::MASKS[idx].0, &Board::MASKS[idx].1);
            hints[idx].0 &= blank;
            hints[idx].1 &= blank;
            legals |= hints[idx].0.clone();
            legals |= hints[idx].1.clone();
        }

        (legals, hints)
    }

    pub fn next(&mut self, mv: u64, hints: [(u64, u64); 4]) {
        debug_assert!(mv.count_ones() == 1);
        #[inline]
        fn calc_rev(white: &u64, black: &u64, shift: &u8, mask: &u64, mv: &u64, hint: (u64, u64)) -> u64 {
            let mut rev0: u64 = 0;
            let mut mov0 = (*mv).clone();
            if (hint.0 & mov0) == mov0 {
                mov0 = (mov0 << shift) & mask;
                while mov0 != 0 && (mov0 & white) != 0 {
                    rev0 |= mov0;
                    mov0 = (mov0 << shift) & mask;
                }
            }
            let mut rev1: u64 = 0;
            let mut mov1 = (*mv).clone();
            if (hint.1 & mov1) == mov1 {
                mov1 = (mov1 >> shift) & mask;
                while mov1 != 0 && (mov1 & white) != 0 {
                    rev1 |= mov1;
                    mov1 = (mov1 >> shift) & mask;
                }
            }
            rev0 | rev1
        }

        let mut rev: u64 = 0;
        for i in 0..4 {
            let idx = i as usize;
            rev |= calc_rev(&self.white, &self.black, &Board::MASKS[idx].0, &Board::MASKS[idx].1, &mv, hints[idx]);
        }

        self.black |= mv | rev;
        self.white ^= rev;
    }
}

