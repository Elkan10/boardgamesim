use crate::{bitboard::{Bitboard, Connect4Bitboard, Connect4Move}, board::{c4_win, Board, Connect4Board}};

mod board;
mod bitboard;

fn main() {

    let mut board = Connect4Board {
        red: Connect4Bitboard::new(),
        yellow: Connect4Bitboard::new(),
        turn: 0,
    };
    let mut i = 0;
    while board.win().is_none() {
        let mv = board.legal_moves()[i % 7];
        board = board.do_move(mv);
        println!("Board: {}", board);
        i += 1;
    }
    
}
