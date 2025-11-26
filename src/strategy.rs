use std::{fs::{File, OpenOptions}, io::{self, Write}};

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


pub struct CSVRow {
    red: String,
    yellow: String,
    winner: Win,
    move_count: u8,
}

impl CSVRow {
    fn new(red: String, yellow: String, winner: Win, move_count: u8) -> Self {
        Self { red, yellow, winner, move_count }
    }

    pub fn out(&self) -> String {
        format!("{},{},{},{}\r\n", self.red, self.yellow, self.winner, self.move_count)
    }
}

pub struct CSV {
    rows: Vec<CSVRow>,
}

impl CSV {
    pub fn new() -> Self {
        Self {
            rows: vec![],
        }
    }

    pub fn add(&mut self, row: CSVRow) {
        self.rows.push(row);
    }
    
    pub fn create(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"Red,Yellow,Win,Move Count\r\n")?;
        for row in &self.rows {
            file.write_all(&row.out().into_bytes())?;
        }
        Ok(())
    }

    pub fn append(&self, path: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(path)?;

        for row in &self.rows {
            file.write_all(&row.out().into_bytes())?;
        }

        return Ok(())
    }
}


pub fn simulate_all(strats: &[&dyn Strategy], count: u32, batch_size: u32) -> io::Result<()> {
    CSV::new().create("out.csv")?;
    let mut prog = 0;
    let total = (strats.len() * strats.len()) as u32;
    for red in strats {
        for yellow in strats {
            simulate_batches(*red, *yellow, count, batch_size, prog, total)?;
            prog += 1;
        }
    }
    Ok(())
}

pub fn simulate_batches(red: &dyn Strategy, yellow: &dyn Strategy, count: u32, batch_size: u32, prog: u32, total: u32) -> io::Result<()> {
    let path = "out.csv";
    let mut i = 0;
    while i < count {
        let batch = batch_size.min(count - i);
        simulate(red, yellow, batch).append(&path)?;
        i += batch_size;
        let progress = (((i + prog * count) * 50) / (count * total)) as usize;
        let bar = "=".repeat(progress) + &" ".repeat(50 - progress);
        print!("\r[{}]", bar);
        io::stdout().flush()?;
    }

    Ok(())
}


pub fn simulate(red: &dyn Strategy, yellow: &dyn Strategy, count: u32) -> CSV {
    let mut csv = CSV::new();
    for _ in 0..count {
        let row = play(red, yellow, false);
        csv.add(row);
    }
    csv
}

pub fn play(red: &dyn Strategy, yellow: &dyn Strategy, print: bool) -> CSVRow {
    if print {
        println!("Simulating {} vs {}", red.name(), yellow.name())
    }
    let mut board = Board {
        red: Bitboard::new(),
        yellow: Bitboard::new(),
        red_to_play: true,
    };
    let mut mv = Move {x: 8, y: 0};
    let mut move_count = 0;
    while let Win::None = board.win() {
        mv = if board.red_to_play {
            red.best_move_wb(board, mv)
        } else {
            yellow.best_move_wb(board, mv)
        };
        board = board.do_move(mv);
        move_count += 1;

        if print {
            println!("---------{}", board);
        }
    }

    CSVRow::new(red.name(), yellow.name(), board.win(), move_count)
}

pub trait Strategy {
    fn best_move(&self, board: Board, last_move: Move) -> Move;
    fn name(&self) -> String;

    
    fn best_move_wb(&self, board: Board, last_move: Move) -> Move {
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
        return "D".to_owned() + &self.mirror.name();
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
        //underflow protection
        if !self.right && x == 0 {
            return *board.legal_moves().choose(&mut rand::rng()).unwrap();
        }
        while x < 7 {
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
            return "Or".into()
        } else {
            return "Ol".into()
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

struct BaseRandom {}

impl BaseRandom {
    fn new() -> Self {
        Self {  }
    }
}

impl Strategy for BaseRandom {
    fn best_move(&self, board: Board, last_move: Move) -> Move {
        Random::new().best_move(board, last_move)
    }

    fn name(&self) -> String {
        "Rb".into()
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
        let mut vals: Vec<(Move, i32)> = board.legal_moves().into_iter().map(|mv| (mv,minimax(board.do_move(mv), &self.eval, self.depth, alpha, -alpha))).collect();

        if board.red_to_play {
            vals.sort_by_key(|(_, x)| -x);
            let max = vals[0].1;
            let top: Vec<(Move, i32)> = vals.into_iter().take_while(|(_, x)| *x == max).collect();
            return top.choose(&mut rand::rng()).unwrap().0
        } else {
            vals.sort_by_key(|(_, x)| *x);
            let min = vals[0].1;
            let bot: Vec<(Move, i32)> = vals.into_iter().take_while(|(_, x)| *x == min).collect();
            return bot.choose(&mut rand::rng()).unwrap().0
        }
    }

    fn best_move_wb(&self, board: Board, last_move: Move) -> Move {
        self.best_move(board, last_move)
    }

    fn name(&self) -> String {
        "M".to_owned() + &self.depth.to_string()
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

