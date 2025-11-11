
use std::fmt::{Display, Write};

use crate::bitboard::{Bitboard, Move, Symmetry};

pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const NO_COLOR: &str = "\x1b[0m";


#[derive(Copy, Clone)]
pub struct Board {
    pub red: Bitboard,
    pub yellow: Bitboard,
    pub red_to_play: bool,
}

pub fn c4_win(bb: Bitboard) -> bool {
    //Vertical
    for byte in bb.data.to_le_bytes() {
        if byte & 0x0f == 0x0f || byte & 0x1e == 0x1e || byte & 0x3c == 0x3c {
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

pub enum Win {
    None,
    Red,
    Yellow,
    Tie
}

impl Board {
    pub fn win(&self) -> Win {
        if c4_win(self.red) {
            return Win::Red;
        }
        if c4_win(self.yellow) {
            return Win::Yellow;
        }
        if (self.yellow.data | self.red.data) == 0x3f3f3f3f3f3f3f {
            return Win::Tie;
        }
        Win::None
    }

    fn canonicalize(&self) -> (Symmetry, Board) {
        let sred = self.red.do_symmetry(Symmetry::S1);
        let syellow = self.yellow.do_symmetry(Symmetry::S1);

        let vc = (self.red.data as u128) + ((self.yellow.data as u128) << 64);
        let vo = (sred.data as u128) + ((syellow.data as u128) << 64);

        if vo < vc {
            (
                Symmetry::S1,
                Self {
                    red: sred,
                    yellow: syellow,
                    red_to_play: self.red_to_play,
                },
            )
        } else {
            (Symmetry::None, self.clone(), ) 
        }
    }

    pub fn flipped(&self) -> Board {
        Board {
            red: self.red,
            yellow: self.yellow,
            red_to_play: !self.red_to_play
        }
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut out = vec![];
        let mut x = 0;
        for (red, yellow) in self.red.data.to_le_bytes().iter().zip(self.yellow.data.to_le_bytes()) {
            if x >= 7 {
                break;
            }
            let b = (red | yellow) + 1;
            let height = b.trailing_zeros();
            if height < 6 {
                out.push(Move::new(x, height as u8));
            }
            x += 1;
        }
        out
    }

    pub fn do_move(&self, mv: Move) -> Self {
        if self.red_to_play {
            Self {
                red: self.red.do_move(mv),
                yellow: self.yellow,
                red_to_play: false,
            }
        } else {
            Self {
                red: self.red,
                yellow: self.yellow.do_move(mv),
                red_to_play: true,
            }
        }
    }

}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..42 {
            if i % 7 == 0 {
                write!(f, "\n")?;
            }
            let red = self.red.getp(i);
            let yellow = self.yellow.getp(i);
            match (red, yellow) {
                (true, true) => f.write_char('E'),
                (true, false) => write!(f, " {}● ", RED),
                (false, true) => write!(f, " {}● ", YELLOW),
                (false, false) => write!(f, " {}○ ", NO_COLOR),
            }?
        }
        write!(f, "{}", NO_COLOR)?;
        Ok(())
    }
}

