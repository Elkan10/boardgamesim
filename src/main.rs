use std::io::stdin;

use crate::strategy::{simulate, Base, Defensive, Greedy, Minimax, Offset, Random, Strategy};

mod board;
mod bitboard;
mod strategy;

fn main() {
    let greedy = Greedy::new();

    let minimax = Minimax::new(Greedy::new(), 11);
    let random = Random::new();
    let above = Base::new(Offset::new(0, 1));

    simulate(&random, &minimax);
}
