use rand::seq::IndexedRandom;

use crate::{bitboard::{Bitboard, Move}, board::{Board, Win}};

#[derive(Debug)]
pub struct Counts {
    r4: u8,
    r3: u8,
    r2: u8,
    r1: u8,
    y4: u8,
    y3: u8,
    y2: u8,
    y1: u8,
}
impl Counts {
    fn new() -> Self {
        Self { r3: 0, r2: 0, r1: 0, y3: 0, y2: 0, y1: 0, r4: 0, y4: 0 }
    }

    fn add(&mut self, red: u32, yellow: u32) {
        match (red, yellow) {
            (1,0) => {self.r1 += 1;}
            (2,0) => {self.r2 += 1;}
            (3,0) => {self.r3 += 1;}
            (4,0) => {self.r4 += 1;}
            (0,1) => {self.y1 += 1;}
            (0,2) => {self.y2 += 1;}
            (0,3) => {self.y3 += 1;}
            (0,4) => {self.y4 += 1;}
            _ => {}
        }
    }
}

pub fn counts(board: Board) -> Counts {
    let mut h = Counts::new();
    let vert_mask: u64 = 0x0f;
    let horiz_mask: u64 = 0x01010101;
    let diag_dmask: u64 = 0x08040201;
    let diag_umask: u64 = 0x01020408;
    for x in 0..7 {
        for y in 0..6 {
            // Vertical
            if y < 3 {
                let mask = vert_mask << (8*x + y);
                let red = (board.red.data & mask).count_ones();
                let yellow = (board.yellow.data & mask).count_ones();
                h.add(red, yellow);
            }
            //Horizontal
            if x < 4 {
                let mask = horiz_mask << (8*x + y);
                let red = (board.red.data & mask).count_ones();
                let yellow = (board.yellow.data & mask).count_ones();
                h.add(red, yellow);
            }
            //Diagonals
            if y < 3 && x < 4 {
                let mask = diag_dmask << (8*x + y);
                let red = (board.red.data & mask).count_ones();
                let yellow = (board.yellow.data & mask).count_ones();
                h.add(red, yellow);

                let mask = diag_umask << (8*x + y);
                let red = (board.red.data & mask).count_ones();
                let yellow = (board.yellow.data & mask).count_ones();
                h.add(red, yellow);
            }
        }
    }

    return h;
}


pub fn simulate(red: &impl Strategy, yellow: &impl Strategy) {
    let mut board = Board {
        red: Bitboard::new(),
        yellow: Bitboard::new(),
        red_to_play: true,
    };
    let mut mv = Move {x: 8, y: 0};
    while let Win::None = board.win() {
        mv = if board.red_to_play {
            red.best_move(board, mv)
        } else {
            yellow.best_move(board, mv)
        };
        board = board.do_move(mv);
        println!("---------{}", board);
    }
}

pub trait Strategy {
    fn best_move(&self, board: Board, last_move: Move) -> Move;
}

pub trait Evaluator {
    fn eval(&self, board: Board) -> i32;
}

pub struct Minimax<E: Evaluator> {
    eval: E,
    depth: u32,
}

impl<E: Evaluator> Minimax<E> {
    pub fn new(eval: E, depth: u32) -> Self {
        Self { eval, depth }
    }
}

pub struct Above {}
impl Above {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Strategy for Above {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        let above = Move {x: last_move.x, y: last_move.y + 1};
        if board.legal_moves().contains(&above) {
            return above;
        } else {
            return *board.legal_moves().choose(&mut rand::rng()).unwrap();
        }
    }
}


pub struct Greedy {}

impl Greedy {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Evaluator for Greedy {
    fn eval(&self, board: Board) -> i32 {
        match board.win() {
            crate::board::Win::None => {},
            crate::board::Win::Red => return 1000,
            crate::board::Win::Yellow => return -1000,
            crate::board::Win::Tie => return 0,
        }
        let h = counts(board);
        let red = h.r3 as i32 * 10 + h.r2 as i32 * 3 + h.r1 as i32;
        let yellow = h.y3 as i32 * 10 + h.y2 as i32 * 3 + h.y1 as i32;
        red - yellow
    }
}

impl<E: Evaluator> Strategy for Minimax<E> {
    fn best_move(&self, board: Board, _last_move: Move) -> Move {
        let alpha = i32::MIN + 1;        
        let vals = board.legal_moves().into_iter().map(|mv| (mv,minimax(board.do_move(mv), &self.eval, self.depth, alpha, -alpha)));
        if board.red_to_play {
            vals.max_by_key(|x| x.1).unwrap().0
        } else {
            vals.min_by_key(|x| x.1).unwrap().0 
        }
    }
}

fn minimax<E: Evaluator>(board: Board, eval: &E, depth: u32, mut alpha: i32, mut beta: i32) -> i32 {
    if depth == 0 {
        return eval.eval(board);
    }
    match board.win() {
        Win::None => {},
        _ => return eval.eval(board),
    }
    if board.red_to_play {
        let mut val = i32::MIN + 1;
        let mut children: Vec<Board> = board.legal_moves().iter().map(|x| board.do_move(*x)).collect();
        children.sort_by_key(|b| -eval.eval(*b));
        for child in children {
            val = val.max(minimax(child, eval, depth - 1, alpha, beta));
            if val >= beta {
                break
            }
            alpha = alpha.max(val);
        }
        return val;
    }
    else {
        let mut val = i32::MAX - 1;
        let mut children: Vec<Board> = board.legal_moves().iter().map(|x| board.do_move(*x)).collect();
        children.sort_by_key(|b| eval.eval(*b));
        for child in children {
            val = val.min(minimax(child, eval, depth - 1, alpha, beta));
            if val <= alpha {
                break
            }
            beta = beta.min(val);
        }
        return val;
    }
}

