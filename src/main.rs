mod game;

use game::tic_tac_toe::TicTacToe;

use crate::game::traits::Game;

fn main() {
  let game = TicTacToe::new();
  game.print_board();
  //println!("Hello, world!");
}
