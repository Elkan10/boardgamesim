use std::io::stdin;

use crate::bitboard::Bitboard;
use crate::board::{Board, Win};
use crate::strategy::{counts, Basic, Evaluator, Minimax, Strategy};

mod board;
mod bitboard;
mod strategy;

fn main() {

    let mut board = Board {
        red: Bitboard::new(),
        yellow: Bitboard::new(),
        red_to_play: true,
    };
    let mut i = 0;
    let b = Basic::new();
    let minimax = Minimax::new(Basic::new(), 10);
    while let Win::None = board.win() {
        let mv = minimax.best_move(board);
        board = board.do_move(mv);
        println!("Board: {}", board);
        i += 1;
        let mut _s = String::new();
        stdin().read_line(&mut _s).unwrap();
    }
    
}
