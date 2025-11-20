use rand::seq::IndexedRandom;

use crate::{bitboard::{Bitboard, Move, WIN_MASKS}, board::{Board, Win}};


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
    fn new(board: Board) -> Self {
        let mut h = Counts {r4: 0, r3: 0, r2: 0, r1: 0, y4: 0, y3: 0, y2: 0, y1: 0};

        for mask in WIN_MASKS {
            let red = board.red.data & mask;
            let yellow = board.yellow.data & mask;
            h.add(red, yellow);
        }

        return h;
    }

    fn add(&mut self, red_mask: u64, yellow_mask: u64) {
        match (red_mask.count_ones(), yellow_mask.count_ones()) {
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

pub struct Base<S: Strategy> {
    strat: S,
}

impl<S: Strategy> Base<S> {
    pub fn new(strat: S) -> Self {
        Self { strat }
    }
}

impl<S: Strategy> Strategy for Base<S> {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        let legal = board.legal_moves();

        for mask in WIN_MASKS {
            let red_mask = board.red.data & mask;
            let yellow_mask = board.yellow.data & mask;
            if red_mask.count_ones() == 3 && yellow_mask.count_ones() == 0 {
                let v = mask ^ red_mask;
                let i = v.trailing_zeros();
                let x = i / 8;
                let y = i - 8 * x;
                let mv = Move {x: x as u8, y: y as u8};
                if legal.contains(&mv) {
                    return mv;
                }
            }
            if yellow_mask.count_ones() == 3 && red_mask.count_ones() == 0 {
                let v = mask ^ yellow_mask;
                let i = v.trailing_zeros();
                let x = i / 8;
                let y = i - 8 * x;
                let mv = Move {x: x as u8, y: y as u8};
                if legal.contains(&mv) {
                    return mv;
                }
            }
        }

        self.strat.best_move(board, last_move)
    }
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

impl<E> Strategy for E where E: Evaluator {
    fn best_move(&self, board: Board, _last_move: Move) -> Move {
        let vals = board.legal_moves().into_iter().map(|mv| (mv, self.eval(board.do_move(mv))));
        if board.red_to_play {
            vals.max_by_key(|x| x.1).unwrap().0
        } else {
            vals.min_by_key(|x| x.1).unwrap().0 
        }
    }
}


pub struct Defensive<M: Strategy> {
    mirror: M,
}
impl<M: Strategy> Defensive<M> {
    pub fn new(mirror: M) -> Self {
        Self { mirror }
    }
}

impl<M: Strategy> Strategy for Defensive<M> {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        self.mirror.best_move(board.flipped(), last_move)
    }
}

pub struct Offset {
    x_offset: u8,
    y_offset: u8,
}
impl Offset {
    pub fn new(x_offset: u8, y_offset: u8) -> Self {
        Self { x_offset, y_offset }
    }
}

impl Strategy for Offset {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        let offset = Move {x: last_move.x + self.x_offset, y: last_move.y + self.y_offset};
        if board.legal_moves().contains(&offset) {
            return offset;
        } else {
            // TODO: Go in the direction until possible
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
            crate::board::Win::Red => return 10000,
            crate::board::Win::Yellow => return -10000,
            crate::board::Win::Tie => return 0,
        }
        let h = Counts::new(board);
        let red = h.r3 as i32 * 100 + h.r2 as i32 * 5 + h.r1 as i32;
        let yellow = h.y3 as i32 * 100 + h.y2 as i32 * 5 + h.y1 as i32;
        red - yellow
    }
}


pub struct Random {}

impl Random {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Strategy for Random {
    fn best_move(&self, board: Board, _last_move: Move) -> Move {
        *board.legal_moves().choose(&mut rand::rng()).unwrap()
    }
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

impl<E: Evaluator> Strategy for Minimax<E> {
    fn best_move(&self, board: Board, _last_move: Move) -> Move {
        let alpha = i32::MIN + 1;
        // TODO: Randomize
        let vals = board.legal_moves().into_iter().map(|mv| (mv,minimax(board.do_move(mv), &self.eval, self.depth, alpha, -alpha)));
        if board.red_to_play {
            let (mv, _) = vals.max_by_key(|x| x.1).unwrap();
            return mv
        } else {
            let (mv, _) = vals.min_by_key(|x| x.1).unwrap();
            return mv
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

