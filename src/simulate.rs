use std::{fs::{File, OpenOptions}, io::{self, Write}};

use crate::{bitboard::{Bitboard, Move}, board::{Board, Win}, strategy::Strategy};

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

#[derive(Clone, Copy)]
pub struct BatchSettings {
    pub count: u32,
    pub batch_size: u32,
}

pub struct Progress {
    current: u32,
    current_bar: usize,
    total: u32,
}

impl Progress {
    pub fn new(total: u32) -> Self {
        Self { current: 0, current_bar: 0, total }
    }

    pub fn inc(&mut self) -> io::Result<()> {
        self.current += 1;
        let progress = ((self.current * 50) / (self.total)) as usize;
        if progress > self.current_bar {
            self.current_bar = progress;
            let bar = "=".repeat(progress) + &" ".repeat(50 - progress);
            print!("\r[{}]", bar);
            io::stdout().flush()?;
        }
        Ok(())
    }
}



/// Total: strats.len() * 2 * settings.count
pub fn simulate_v1(strat: &impl Strategy, strats: &[&dyn Strategy], settings: BatchSettings, prog: &mut Progress) -> io::Result<()> {
    CSV::new().create("out.csv")?;
    for other in strats {
        simulate_batches(strat, *other, settings, prog)?;
        simulate_batches(*other, strat, settings, prog)?;
    }

    Ok(())
}

/// Total: strats.len() * strats.len() * settings.count
pub fn simulate_all(strats: &[&dyn Strategy], settings: BatchSettings, prog: &mut Progress) -> io::Result<()> {
    CSV::new().create("out.csv")?;
    for red in strats {
        for yellow in strats {
            simulate_batches(*red, *yellow, settings, prog)?;
        }
    }
    Ok(())
}


/// Total: settings.count
pub fn simulate_batches(red: &dyn Strategy, yellow: &dyn Strategy, settings: BatchSettings, prog: &mut Progress) -> io::Result<()> {
    let path = "out.csv";
    let mut i = 0;
    while i < settings.count {
        let batch = settings.batch_size.min(settings.count - i);
        simulate(red, yellow, batch, prog).append(&path)?;
        i += settings.batch_size;
    }

    Ok(())
}

/// Total: count
pub fn simulate(red: &dyn Strategy, yellow: &dyn Strategy, count: u32, prog: &mut Progress) -> CSV {
    let mut csv = CSV::new();
    for _ in 0..count {
        let row = play(red, yellow, false);
        prog.inc();
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
    let mut mv = Move {x: 254, y: 254};
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
