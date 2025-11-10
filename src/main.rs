use std::io::stdin;

use crate::bitboard::{Bitboard, Move};
use crate::board::{Board, Win};
use crate::strategy::{simulate, Above, Greedy, Minimax, Strategy};

mod board;
mod bitboard;
mod strategy;

fn main() {

    let mut board = Board {
        red: Bitboard::new(),
        yellow: Bitboard::new(),
        red_to_play: true,
    };

    let minimax = Minimax::new(Greedy::new(), 10);
    let above = Above::new();

    simulate(&above, &minimax);
}
