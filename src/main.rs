use crate::{bitboard::{Bitboard, Connect4Bitboard, Connect4Move}, board::{c4_win, Board, Connect4Board}};

mod board;
mod bitboard;

fn main() {

    let mut board = Connect4Board {
        red: Connect4Bitboard::new(),
        yellow: Connect4Bitboard::new(),
        turn: 0,
    };

    while board.win().is_none() {
        let mv = board.legal_moves()[0];
        board = board.do_move(mv);
        println!("Red: \n{}\n Yellow:\n{}\n", board.red, board.yellow);
    }
    
}
