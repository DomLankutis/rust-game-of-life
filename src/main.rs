mod screen;
extern crate rand;
extern crate colored;

use std::{io, thread, time};
use screen::Screen;

fn main() {
    let tick = time::Duration::from_secs(1) / 2;
    let (x, y) = (get_i32(), get_i32());
    let mut screen = Screen::gen_screen(x, y);

    loop {
        screen.draw();
        cell_step(&mut screen);
        thread::sleep(tick);
    }
}

fn modulo (a: i32, b: i32) -> i32 {
    let r = a % b;
    if r < 0 {
        return if b > 0 { r + b } else { r - b }
    }
    r
}

fn round_check(y: usize , x: usize, grid: &Vec<Vec<bool>>) -> i32 {
    let mut alive = 0;
    let size = (grid.len() as i32 , grid[0].len() as i32 );
    for y_mod in -1..2 {
        for x_mod in -1..2 {
            if grid[modulo(y as i32 + y_mod, size.0) as usize][modulo(x as i32 + x_mod, size.1) as usize] == true && !(y_mod == 0 && x_mod == 0) {
                alive += 1;
            }
        }
    }
    alive
}

fn cell_step(screen: &mut Screen) {
    let mut alive = Vec::new();
    let mut dead = Vec::new();

    for y in screen.get_grid().iter().enumerate() {
        for x in y.1.iter().enumerate() {
            let cells_alive = round_check(y.0, x.0, &screen.get_grid());
            if cells_alive == 3 {
                alive.append(&mut vec![[y.0, x.0]]);
            }
                else if cells_alive < 2 && *x.1 == true {
                    dead.append(&mut vec![[y.0, x.0]]);
                }
                    else if cells_alive > 3 && *x.1 == true {
                        dead.append(&mut vec![[y.0, x.0]]);
                    }
                        else if cells_alive == 2 && *x.1 == true {
                            alive.append(&mut vec![[y.0, x.0]]);
                        }
        }
    }

    let mut new_grid = screen.get_grid().clone();

    for c in dead {
        new_grid[c[0]][c[1]] = false;
    }
    for c in alive {
        new_grid[c[0]][c[1]] = true;
    }

    screen.set_grid(new_grid);
}

fn get_i32() -> usize {
    let mut num = String::new();
    io::stdin().read_line(&mut num)
        .expect("Failed to read line");
    let x: usize = num.trim().parse()
        .expect("Please provide a number");
    return x;
}

