
use crate::strategy::{Greedy, Minimax, Offset, simulate};

mod board;
mod bitboard;
mod strategy;

fn main() {
    let minimax = Minimax::new(Greedy::new(), 11);
    let right = Offset::new(true);

    let csv = simulate(&right, &minimax, 10);

    csv.create("out.csv".into()).unwrap();
}
