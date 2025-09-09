use std::fmt::{self, Display};

pub trait Bitboard: Sized + Copy + Display {
    type Symmetry;
    type Move;
    fn do_symmetry(&self, sym: Self::Symmetry) -> Self;
    fn do_move(&self, mv: Self::Move) -> Self;
    fn getc(&self) -> u64;
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
    pub data: u64, //one byte, one column, uses 7 bytes (0LxxxxxR), in byte 6 bits (00TxxxxB)
}
impl Connect4Bitboard {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Bitboard for Connect4Bitboard {
    type Symmetry = Connect4Symmetry;
    type Move = Connect4Move;

    fn getc(&self) -> u64 {
        self.data
    }

    fn do_symmetry(&self, sym: Self::Symmetry) -> Self {
        match sym {
            Connect4Symmetry::None => self.clone(),
            Connect4Symmetry::S1 => Connect4Bitboard { data: self.data.swap_bytes() >> 1 },
        }
    }


    fn do_move(&self, mv: Self::Move) -> Self {
        let data = self.data + (1 << (8 * (mv.x + 1) + mv.y));
        Connect4Bitboard { data }
    }

}

impl Display for Connect4Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in (0..6).rev() {
            let mut j = i + 8;
            while j < 64 {
                if self.data & (1 << j) == 0 {
                    print!("o")
                } else {
                    print!("x")
                }
                j += 8;
            }
            print!("\n");
        }
        fmt::Result::Ok(())
    }
}
