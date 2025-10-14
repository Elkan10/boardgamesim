use std::fmt::{self, Display};

pub trait Bitboard: Sized + Copy {
    type Symmetry;
    type Move;
    fn do_symmetry(&self, sym: Self::Symmetry) -> Self;
    fn do_move(&self, mv: Self::Move) -> Self;
    fn getp(&self, pos: u8) -> bool;
}



pub enum Connect4Symmetry {
    None,
    S1, //Horizontal Reflection
}

#[derive(Copy, Clone)]
pub struct Connect4Move {
    x: u8,
    y: u8,
}

impl Connect4Move {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}




#[derive(Copy, Clone)]
pub struct Connect4Bitboard {
    pub data: u64, //one byte, one column, uses 7 bytes (0RxxxxxL), in byte 6 bits (00TxxxxB)
}
impl Connect4Bitboard {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Bitboard for Connect4Bitboard {
    type Symmetry = Connect4Symmetry;
    type Move = Connect4Move;

    fn getp(&self, pos: u8) -> bool {
        let row = pos / 7; //integer division rounds down (for positive values)
        let index = 5 - row + 8 * (pos - 7 * row);
        self.data & (1 << index) != 0
    }

    fn do_symmetry(&self, sym: Self::Symmetry) -> Self {
        match sym {
            Connect4Symmetry::None => self.clone(),
            Connect4Symmetry::S1 => Connect4Bitboard { data: self.data.swap_bytes() >> 1 },
        }
    }


    fn do_move(&self, mv: Self::Move) -> Self {
        let data = self.data + (1 << (8 * mv.x + mv.y));
        Connect4Bitboard { data }
    }

}

