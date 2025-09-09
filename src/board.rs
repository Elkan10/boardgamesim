use crate::bitboard::{Bitboard, Connect4Bitboard, Connect4Move, Connect4Symmetry};

pub trait Board {
    type Move;
    type Symmetry;
    fn win(&self) -> Option<u8>;
    fn canonicalize(&self) -> Self;
    fn legal_moves(&self) -> Vec<Self::Move>;
    fn do_move(&self, mv: Self::Move) -> Self;
}

#[derive(Copy, Clone)]
pub struct Connect4Board {
    pub red: Connect4Bitboard,
    pub yellow: Connect4Bitboard,
    pub turn: u8,
}

pub fn c4_win(bb: Connect4Bitboard) -> bool {
    //Vertical
    for byte in bb.data.to_le_bytes() {
        if byte == 0x0f || byte == 0x1e || byte == 0x3c {
            return true;
        }
    }

    //Horizontal
    // start at 0, 1, 2, 3
    for i in 0..=3 {
        let mut count = 0;
        let mut mask = bb.data.to_le_bytes()[i];
        while mask > 0 {
            mask = mask & bb.data.to_le_bytes()[i + count + 1];
            count += 1;
            if count >= 4 {
                return true;
            }
        }
    }

    //Diagonal up
    for i in 0..=3 {
        let mut count = 0;
        let mut mask = bb.data.to_le_bytes()[i];
        while mask > 0 {
            mask = (mask << 1) & bb.data.to_le_bytes()[i + count + 1];
            count += 1;
            if count >= 4 {
                return true;
            }
        }
    }

    //Diagonal down
    for i in 0..=3 {
        let mut count = 0;
        let mut mask = bb.data.to_le_bytes()[i];
        while mask > 0 {
            mask = (mask >> 1) & bb.data.to_le_bytes()[i + count + 1];
            count += 1;
            if count >= 4 {
                return true;
            }
        }
    }
    false
}

impl Board for Connect4Board {
    type Move = Connect4Move;

    type Symmetry = Connect4Symmetry;

    fn win(&self) -> Option<u8> {
        if c4_win(self.red) {
            return Some(0);
        }
        if c4_win(self.yellow) {
            return Some(1);
        }
        None
    }

    fn canonicalize(&self) -> Self {
        let sred = self.red.do_symmetry(Connect4Symmetry::S1);
        let syellow = self.yellow.do_symmetry(Connect4Symmetry::S1);

        let vc = (self.red.data as u128) + ((self.yellow.data as u128) << 64);
        let vo = (sred.data as u128) + ((syellow.data as u128) << 64);

        if vo < vc {
            Self {
                red: sred,
                yellow: syellow,
                turn: self.turn,
            }
        } else {
            self.clone()
        }
    }

    fn legal_moves(&self) -> Vec<Self::Move> {
        let mut out = vec![];
        let mut x = 0;
        for (red, yellow) in self.red.data.to_le_bytes().iter().skip(1).zip(self.yellow.data.to_le_bytes().iter().skip(1)) {
            let b = (red | yellow) + 1;
            let height = b.trailing_zeros();
            if height == 8 {
                out.push(Connect4Move::new(x, 0));
            } else if height >= 6 {
                
            } else {
                out.push(Connect4Move::new(x, height as u8));
            }
            x += 1;
        }
        out
    }

    fn do_move(&self, mv: Self::Move) -> Self {
        if self.turn == 0 {
            Self {
                red: self.red.do_move(mv),
                yellow: self.yellow,
                turn: 1,
            }
        } else {
            Self {
                red: self.red,
                yellow: self.yellow.do_move(mv),
                turn: 0,
            }
        }
    }

}

