use crate::game::traits::Game;
use colored::*;

#[derive(Copy, Clone)]
enum Piece {
    Empty = 0,
    White = 1,
    Black = 2,
}

pub struct TicTacToe {
    board: [Piece; 9],
}

impl Game for TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: [Piece::Empty; 9],
        }
    }
    fn dimension(&self) -> (i32, i32) {
        (5, 5)
    }
    fn size(&self) -> i32 {
        9
    }
    fn print_field(&self, x: i32, y : i32) {
        match self.board[(y * 3 + x) as usize] {
            Piece::Empty => print!("  "),
            Piece::White => print!("{}", "  ".on_blue()),
            Piece::Black => print!("{}", "  ".on_red()),
        }
    }
}
