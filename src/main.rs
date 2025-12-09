
use crate::{simulate::{BatchSettings, Progress, simulate_all}, strategy::{Greedy, Minimax, Offset}};

mod board;
mod bitboard;
mod strategy;
mod simulate;

fn main() {
    let minimax = Minimax::new(Greedy::new(), 5);
    let right = Offset::new(true);
    let greedy = Greedy::new();

    let settings = BatchSettings {
        count: 1,
        batch_size: 1
    };

    let mut prog = Progress::new(3 * 3 * 1);

    simulate_all(&[&minimax, &right, &greedy], settings, &mut prog).unwrap();
}
