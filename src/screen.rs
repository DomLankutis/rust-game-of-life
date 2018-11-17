use rand::Rng;
use colored::*;
use std::collections::HashMap;
use std::process::{Command, Output};

pub struct Screen {
    grid: Vec<Vec<bool>>,
    symbols: HashMap<bool, ColoredString>,
    clear: Output,
}

impl Screen {
    pub fn gen_screen(x: usize, y: usize) -> Screen {
        let mut rng = rand::thread_rng();
        let mut screen = Vec::new();
        for xx in 0..x {
            screen.push(Vec::new());
            for _ in 0..y {
                let v: bool = rng.gen();
                if v {
                    screen[xx].push(false);
                } else {
                    screen[xx].push(true);
                }
            }
        }

        let mut symbols = HashMap::new();
        symbols.insert(true, "\u{2588}".black());
        symbols.insert(false, "\u{2588}".white());

        let clear = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", "cls"])
                .output()
                .expect("failed to execute")
        } else {
            Command::new("sh")
                .args(&["-c", "clear"])
                .output()
                .expect("failed to execute")
        };

        Screen {
            grid: screen,
            symbols,
            clear,
        }
    }

    pub fn get_grid(&self) -> &Vec<Vec<bool>> {
        &self.grid
    }

    pub fn set_grid(&mut self, new_grid: Vec<Vec<bool>>) {
        self.grid = new_grid;
    }

    pub fn draw(&self) {
        println!("{}", String::from_utf8_lossy(&self.clear.stdout));
        for y in &self.grid {
            for x in y {
                print!("{}", self.symbols[x]);
            }
            print!("\n");
        }
    }
}