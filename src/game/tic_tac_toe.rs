use crate::game::traits::{DefaultBoard, Game};

use termbox::Cell;

#[derive(Copy, Clone, PartialEq)]
enum Piece {
    Empty = 0,
    White = 1,
    Black = 2,
}

pub struct TicTacToe {
    board: [Piece; 9],
    map: Vec<Cell>,
}

impl DefaultBoard for TicTacToe {}

impl Game for TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: [Piece::Empty; 9],
            map: <TicTacToe as DefaultBoard>::new_board(13, 7),
        }
    }
    fn dimension(&self) -> (usize, usize) {
        (3, 3)
    }
    fn board_size(&self) -> (usize, usize) {
        (13, 7)
    }
    fn map(&self) -> &Vec<termbox::Cell> {
        &self.map
    }
    fn map_mut(&mut self) -> &mut Vec<termbox::Cell> {
        &mut self.map
    }
    fn key(&self, key: i32) -> () {

    }
    fn select_cell<'a>(&'a mut self, x: usize, y: usize) {
        DefaultBoard::select_cell(self, x, y);
    }
    fn is_finished(&self) -> bool {
        if self.board.contains(&Piece::Empty) {
            return false;
        }
        return true;
    }
    fn get_field(&self, x: usize, y: usize) -> &str {
        match self.board[(y * 3 + x) as usize] {
            Piece::Empty => (" O "),
            Piece::White => (" W "),
            Piece::Black => (" B "),
        }
    }
    // fn draw_board(&self) -> Vec<termbox::Cell> {
    //     DefaultBoard::draw_board(self)
    // }

    fn input(&self) -> () {
        use std::io::{stdin, stdout, Write};
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
    }
}
