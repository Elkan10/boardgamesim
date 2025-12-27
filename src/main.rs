
use crate::{simulate::{BatchSettings, Progress, simulate_all}, strategy::{Above, BaseRandom, Defensive, Greedy, Minimax, Offset, Random, Strategy}};

mod board;
mod bitboard;
mod strategy;
mod simulate;

fn main() {
    let minimax = Minimax::new(Greedy::new(), 10);
    let right = Offset::new(true);
    let left = Offset::new(false);
    let greedy = Greedy::new();
    let random = Random::new();
    let random_wb = BaseRandom::new();
    let above = Above::new();
    let defensive = Defensive::new(Greedy::new());

    let strats: [&dyn Strategy; 8] = [&right, &left, &greedy, &random, &random_wb, &above, &defensive, &minimax];

    let settings = BatchSettings {
        count: 100,
        batch_size: 5
    };

    let mut prog = Progress::new(8 * 8 * 100, 200);

    simulate_all(&strats, settings, &mut prog).unwrap();
}
