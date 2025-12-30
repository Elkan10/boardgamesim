use std::{cell::RefCell, collections::HashMap, hash::Hash};
use rand::seq::IndexedRandom;

use crate::{bitboard::{Move, WIN_MASKS}, board::{Board, Win}};


pub trait Strategy {
    fn best_move(&self, board: Board, last_move: Move) -> Move;
    fn name(&self) -> String;

    
    fn best_move_wb(&self, board: Board, last_move: Move) -> Move {
        let legal = board.legal_moves();

        for mask in WIN_MASKS {
            let (my_mask, other_mask) = if board.red_to_play {
                (board.red.data & mask, board.yellow.data & mask)
            } else {
                (board.yellow.data & mask, board.red.data & mask)
            };
            if my_mask.count_ones() == 3 && other_mask == 0 {
                let v = mask ^ my_mask;
                let i = v.trailing_zeros();
                let x = i / 8;
                let y = i - 8 * x;
                let mv = Move {x: x as u8, y: y as u8};
                if legal.contains(&mv) {
                    return mv;
                }
            }
            if other_mask.count_ones() == 3 && my_mask == 0 {
                let v = mask ^ other_mask;
                let i = v.trailing_zeros();
                let x = i / 8;
                let y = i - 8 * x;
                let mv = Move {x: x as u8, y: y as u8};
                if legal.contains(&mv) {
                    return mv;
                }
            }
        }

        self.best_move(board, last_move)
    }
}

pub trait Evaluator {
    fn eval(&self, board: Board) -> i32;
    fn name(&self) -> String;
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

    fn name(&self) -> String {
        self.name()
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

    fn name(&self) -> String {
        return "d".to_owned() + &self.mirror.name();
    }
}

pub struct Offset {
    right: bool,
}
impl Offset {
    pub fn new(right: bool) -> Self {
        Self { right }
    }
}

impl Strategy for Offset {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        let mut x = last_move.x;
        while x < 7 {
            //underflow protection
            if !self.right && x == 0 {
                return *board.legal_moves().choose(&mut rand::rng()).unwrap();
            }
            if self.right {
                x += 1;
            } else {
                x -= 1;
            }
            match board.column(x) {
                Some(mv) => return mv,
                _ => {},
            }
        }
        return *board.legal_moves().choose(&mut rand::rng()).unwrap();
    }

    fn name(&self) -> String {
        if self.right {
            return "hS".into()
        } else {
            return "vS".into()
        }
    }
}

pub struct Above {}
impl Above {
    pub fn new() -> Self {
        Self { }
    }
}

impl Strategy for Above {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        let offset = Move {x: last_move.x, y: last_move.y + 1};
        if board.legal_moves().contains(&offset) {
            return offset;
        } else {
            return *board.legal_moves().choose(&mut rand::rng()).unwrap();
        }
    }

    fn name(&self) -> String {
        "A".into()
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
        if board.is_tie() {
            return 0;
        }
        let mut score = 0;

        for mask in WIN_MASKS {
            let red = board.red.data & mask;
            let yellow = board.yellow.data & mask;
            if red == mask {
               return 10000; 
            }
            if yellow == mask {
                return -10000;
            }

            if (red == 0 && yellow == 0) || (red != 0 && yellow != 0) {
                continue;
            }

            if red == 0 {
                match yellow.count_ones() {
                    1 => {score -= 1;}
                    2 => {score -= 5;}
                    3 => {score -= 100;}
                    _ => {}
                }
            } else {
                match red.count_ones() {
                    1 => {score += 1;}
                    2 => {score += 5;}
                    3 => {score += 100;}
                    _ => {}
                }
            }

        }

        return score;
    }

    fn name(&self) -> String {
        "G".into()
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

    fn best_move_wb(&self, board: Board, last_move: Move) -> Move {
        self.best_move(board, last_move)
    }

    fn name(&self) -> String {
        "R".into()
    }
}

pub struct BaseRandom {}

impl BaseRandom {
    pub fn new() -> Self {
        Self {  }
    }
}

impl Strategy for BaseRandom {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        Random::new().best_move(board, last_move)
    }

    fn name(&self) -> String {
        "bR".into()
    }
}





pub struct Minimax<E: Evaluator> {
    eval: E,
    depth: u8,
    pos_table: RefCell<HashMap<Board, PosEntry>>,
}

pub enum Bound {
    Upper,
    Lower,
    Exact,
}

pub struct PosEntry {
    depth: u8,
    value: i16,
    bound: Bound,
}

impl PosEntry {
    pub fn new(depth: u8, value: i16, bound: Bound) -> Self {
        Self { depth, value, bound }
    }
}

impl<E: Evaluator> Minimax<E> {
    pub fn new(eval: E, depth: u8) -> Self {
        Self { eval, depth, pos_table: RefCell::new(HashMap::new()) }
    }
}

impl<E: Evaluator> Strategy for Minimax<E> {
    fn best_move(&self, board: Board, _last_move: Move) -> Move {
        let alpha = i16::MIN;
        let beta = i16::MAX;
        let mut vals: Vec<(Move, i16)> = board.legal_moves().into_iter().map(|mv| (mv,minimax(board.do_move(mv), &self.eval, &mut self.pos_table.borrow_mut(), self.depth, alpha, beta))).collect();
        if board.red_to_play {
            vals.sort_by_key(|(_, x)| -x);
            let max = vals[0].1;
            let top: Vec<(Move, i16)> = vals.into_iter().take_while(|(_, x)| *x == max).collect();
            return top.choose(&mut rand::rng()).unwrap().0
        } else {
            vals.sort_by_key(|(_, x)| *x);
            let min = vals[0].1;
            let bot: Vec<(Move, i16)> = vals.into_iter().take_while(|(_, x)| *x == min).collect();
            return bot.choose(&mut rand::rng()).unwrap().0;
        }
    }

    fn best_move_wb(&self, board: Board, last_move: Move) -> Move {
        self.best_move(board, last_move)
    }

    fn name(&self) -> String {
        "M".to_owned() + &self.depth.to_string()
    }
}

fn minimax<E: Evaluator>(board: Board, eval: &E, pos_table: &mut HashMap<Board, PosEntry>, depth: u8, mut alpha: i16, mut beta: i16) -> i16 {
    match board.win() {
        Win::None => {},
        _ => return eval.eval(board) as i16,
    }

    if depth == 0 {
        return eval.eval(board) as i16;
    }

    
    if let Some(entry) = pos_table.get(&board.canonicalize()) {
        if entry.depth == depth {
            match entry.bound {
                Bound::Exact => return entry.value.into(),
                Bound::Lower => {
                    alpha = alpha.max(entry.value);
                },
                Bound::Upper => {
                    beta = beta.min(entry.value);
                },
            }
            if alpha >= beta {
                return entry.value;
            }
        }
    }

    let (alpha_orig, beta_orig) = (alpha, beta);
    let legal = board.legal_moves();
    let children = legal.iter().map(|x| board.do_move(*x));
    let mut val;
    
    if board.red_to_play {
        val = i16::MIN;

        for child in children {
            val = val.max(minimax(child, eval, pos_table, depth - 1, alpha, beta));
            alpha = alpha.max(val);
            if alpha >= beta {
                break
            }
        }
    }
    else {
        val = i16::MAX;

        for child in children {
            val = val.min(minimax(child, eval, pos_table, depth - 1, alpha, beta));
            beta = beta.min(val);
            if alpha >= beta {
                break
            }
        }
    }
        
    let bound = if val >= beta_orig {
        Bound::Lower
    } else if val <= alpha_orig {
        Bound::Upper
    } else {
        Bound::Exact
    };
    pos_table.insert(board.canonicalize(), PosEntry::new(depth, val, bound));
    return val;
}

