use std::sync::Arc;

use termbox;

use termbox::{Cell, Event, Termbox, BLACK, BLUE, BOLD, KEY_ESC, WHITE};

pub trait Game {
    fn new() -> Self;
    fn dimension(&self) -> (usize, usize);
    fn board_size(&self) -> (usize, usize);
    fn get_field(&self, x: usize, y: usize) -> &str;
    fn is_finished(&self) -> bool;
    fn key(&self, key: i32) -> ();
    fn input(&self) -> ();
    fn map(&self) -> &Vec<termbox::Cell>;
    fn map_mut(&mut self) -> &mut Vec<termbox::Cell>;
    fn select_cell<'a>(&'a mut self, x: usize, y: usize);
    //fn draw_board(&self) -> Vec<Cell>;
    fn run(&self) {
        while !self.is_finished() {
            self.input();
        }
    }
}

pub trait DefaultBoard: Game {
    fn new_board(size_x: usize, size_y: usize) -> Vec<Cell> {
        let mut buffer = Vec::new();
        fn cell(c: char) -> termbox::Cell {
            termbox::Cell {
                ch: c as u32,
                fg: WHITE,
                bg: BLACK,
            }
        }
        buffer.resize(size_x * size_y, cell(' '));
        buffer[0] = cell('╔');
        buffer[size_x - 1] = cell('╗');
        let botline = (size_y - 1) * size_x;
        buffer[botline] = cell('╚');
        buffer[botline + size_x - 1] = cell('╝');
        for x in 1..size_x - 1 {
            if x % 4 == 0 {
                buffer[x] = cell('╤');
                buffer[botline + x] = cell('╧');
            } else {
                buffer[x] = cell('═');
                buffer[botline + x] = cell('═');
            }
        }
        for y in 1..size_y - 1 {
            let i = y * size_x;
            if y % 2 == 0 {
                buffer[i] = cell('╟');
                buffer[i + size_x - 1] = cell('╢');
            } else {
                buffer[i] = cell('║');
                buffer[i + size_x - 1] = cell('║');
            }
            for x in 1..size_x - 1 {
                if y % 2 == 0 && x % 4 == 0 {
                    buffer[i + x] = cell('┼');
                } else if x % 4 == 0 {
                    buffer[i + x] = cell('│');
                } else if y % 2 == 0 {
                    buffer[i + x] = cell('─');
                }
            }
        }
        buffer
    }
    fn select_cell<'a>(&'a mut self, x: usize, y: usize) {
        let x_start = x * 4;
        let y_start = y * 2;
        let (x_size, _) = self.board_size();
        //let mut buffer = self.map_mut()[i];
        //buffer.bg = termbox::BLUE;
        for y in 0..3 {
            let i: usize = (y_start + y) * x_size;
            for x in 0..5 {
                if y != 1 || x == 0 || x == 4 {
                    // self.map_mut()[i + x + x_start].fg = termbox::WHITE;
                    // self.map_mut()[i + x + x_start].bg = termbox::WHITE;
                } else {
                    self.map_mut()[i + x + x_start].fg = termbox::BLACK;
                    self.map_mut()[i + x + x_start].bg = termbox::BLUE;
                }
            }
        }
        //let e = &mut buffer[i];
        //e.bg = termbox::BLUE;
    }
}
