pub const WIN_MASKS: [u64; 69] = generate_win_masks();


const fn generate_win_masks() -> [u64; 69] {
    let mut masks = [0u64; 69];
    let mut i = 0;
    let mut x = 0;
    let mut y = 0;

    
    let vert_mask: u64 = 0x0f;
    let horiz_mask: u64 = 0x01010101;
    let diag_dmask: u64 = 0x08040201;
    let diag_umask: u64 = 0x01020408;
    
    while x < 7 {
        while y < 6 {
            // Vertical
            if y < 3 {
                let mask = vert_mask << (8*x + y);
                masks[i] = mask;
                i += 1;
            }
            //Horizontal
            if x < 4 {
                let mask = horiz_mask << (8*x + y);
                masks[i] = mask;
                i += 1;
            }
            //Diagonals
            if y < 3 && x < 4 {
                let mask = diag_dmask << (8*x + y);
                masks[i] = mask;
                i += 1;
                
                let mask = diag_umask << (8*x + y);
                masks[i] = mask;
                i += 1;
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }

    return masks;
}



#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Move {
    pub x: u8,
    pub y: u8,
}

impl Move {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }
}




#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn do_symmetry(&self) -> Self {
        Bitboard { data: self.data.swap_bytes() >> 1 }
    }


    pub fn do_move(&self, mv: Move) -> Self {
        let data = self.data + (1 << (8 * mv.x + mv.y));
        Bitboard { data }
    }

}

