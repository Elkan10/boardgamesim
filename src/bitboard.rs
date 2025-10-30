pub enum Symmetry {
    None,
    S1, //Horizontal Reflection
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}


impl Move {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    pub fn decanonicalize(&self, s: Symmetry) -> Self {
        match  s {
            Symmetry::None => self.clone(),
            Symmetry::S1 => Self {
                x: 7 - self.x,
                y: self.y,
            },
        }
    }
}




#[derive(Copy, Clone)]
pub struct Bitboard {
    pub data: u64, //one byte, one column, uses 7 bytes (0RxxxxxL), in byte 6 bits (00TxxxxB)
}
impl Bitboard {
    pub fn new() -> Self {
        Self { data: 0 }
    }
}

impl Bitboard {
    pub fn getp(&self, pos: u8) -> bool {
        let row = pos / 7; //integer division rounds down (for positive values)
        let index = 5 - row + 8 * (pos - 7 * row);
        self.data & (1 << index) != 0
    }

    pub fn do_symmetry(&self, sym: Symmetry) -> Self {
        match sym {
            Symmetry::None => self.clone(),
            Symmetry::S1 => Bitboard { data: self.data.swap_bytes() >> 1 },
        }
    }


    pub fn do_move(&self, mv: Move) -> Self {
        let data = self.data + (1 << (8 * mv.x + mv.y));
        Bitboard { data }
    }

}

