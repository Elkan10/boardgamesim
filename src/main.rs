
use crate::strategy::{Greedy, Minimax, Offset, play, simulate_all};

mod board;
mod bitboard;
mod strategy;

fn main() {
    let minimax = Minimax::new(Greedy::new(), 5);
    let right = Offset::new(true);
    let greedy = Greedy::new();

    simulate_all(&[&minimax, &right, &greedy], 10, 5).unwrap();
}
