
use std::fmt::{Display, Write};

use crate::bitboard::{Bitboard, Move, WIN_MASKS};

pub const RED: &str = "\x1b[31m";
pub const YELLOW: &str = "\x1b[33m";
pub const NO_COLOR: &str = "\x1b[0m";


#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Board {
    pub red: Bitboard,
    pub yellow: Bitboard,
    pub red_to_play: bool,
}

pub enum Win {
    None,
    Red,
    Yellow,
    Tie
}

impl Display for Win {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Win::None => write!(f, "None"),
            Win::Red => write!(f, "Red"),
            Win::Yellow => write!(f, "Yellow"),
            Win::Tie => write!(f, "Tie"),
        }
    }
}

impl Board {
    pub fn win(&self) -> Win {
        for mask in WIN_MASKS {
            let red = self.red.data & mask;
            let yellow = self.yellow.data & mask;

            if red == mask {
                return Win::Red;
            }
            if yellow == mask {
                return Win::Yellow;
            }
        }
                
        if (self.yellow.data | self.red.data) == 0x3f3f3f3f3f3f3f {
            return Win::Tie;
        }
        
        Win::None
    }

    pub fn is_tie(&self) -> bool {
        (self.yellow.data | self.red.data) == 0x3f3f3f3f3f3f3f
    }

    pub fn canonicalize(&self) -> Board {
        let sred = self.red.do_symmetry();
        let syellow = self.yellow.do_symmetry();

        let vc = (self.red.data as u128) + ((self.yellow.data as u128) << 64);
        let vo = (sred.data as u128) + ((syellow.data as u128) << 64);

        if vo < vc {
                Self {
                    red: sred,
                    yellow: syellow,
                    red_to_play: self.red_to_play,
                }
        } else {
            self.clone() 
        }
    }

    pub fn flipped(&self) -> Board {
        Board {
            red: self.red,
            yellow: self.yellow,
            red_to_play: !self.red_to_play
        }
    }

    pub fn column(&self, x: u8) -> Option<Move> {
        if x > 6 {
            return None;
        }
        let red = self.red.data.to_le_bytes()[x as usize];
        let yellow = self.yellow.data.to_le_bytes()[x as usize];
        let b = (red | yellow) + 1;
        let y = b.trailing_zeros() as u8;
        if y < 6 {
            return Some(Move::new(x, y))
        }

        None
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        vec![3, 2, 4, 1, 5, 0, 6].iter().filter_map(|x| self.column(*x)).collect()
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

