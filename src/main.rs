mod game;

use std::cell::Cell;
use std::io::copy;

use game::tic_tac_toe::TicTacToe;

use crate::game::traits::Game;
use termbox;

use termbox::{
  Termbox,
  Event,
  BLACK,
  WHITE,
  BOLD,
  KEY_ESC,
  KEY_ARROW_UP,
  MouseButton,
};

fn main () {
  let mut game = TicTacToe::new();

  let map = game.map();

  // Open the terminal
  let mut tb : termbox::Termbox = Termbox::open().unwrap();

  tb.set_mouse_enabled(true);
  // Clear the screen to black
  tb.set_clear_attributes(BLACK, BLACK);
  tb.clear();

  let (x_size, y_size) = game.board_size();
  tb.blit(0, 0, x_size as i32, y_size as i32, &map);

  tb.present();

  loop {
    match tb.poll_event() {
      Event::Key(event) => {
        if event.key == KEY_ESC {
          break;
        }
        if event.key == KEY_ARROW_UP {
          game.select_cell(2, 1);
          tb.blit(0, 0, x_size as i32, y_size as i32, game.map());
          tb.present();
        }
      },
      Event::Mouse(event) => {
        if event.button == MouseButton::Left {
          game.select_cell((event.x / 5) as usize, (event.y / 3) as usize);
          tb.blit(0, 0, x_size as i32, y_size as i32, game.map());
          tb.present();
        }
      }

      _ => {},
    }
  }

  // Display a message
  // fn cell_buffer_mut<'a>(&'a mut self) -> &'a mut [Cell]
  // tb.put_str(0, 0, "Hello, world!", WHITE | BOLD, BLACK);
  // tb.put_str(0, 1, "Press Esc to continue", WHITE, BLACK);
  // let board_data = game.draw_board();
  //game.select_cell(1, 1);



  //let buffer : &mut [termbox::Cell] = tb.cell_buffer_mut();

  //let (x_size, y_size) = game.board_size();
  //print!("a{}\n", board_data.len());
  //assert!(x_size * y_size == board_data.len() as i32);
  //buffer[10] = termbox::Cell{ch:42, bg: BLACK, fg:WHITE};
  //tb.put_str(0, 2, &game.board_string(), WHITE, BLACK);


  // Wait for the user to press Esc

}

// mod game;

// use game::tic_tac_toe::TicTacToe;

// use crate::game::traits::Game;

// fn main() {
//   let game = TicTacToe::new();
//   game.run();
//   //game.print_board();
//   //println!("Hello, world!");
// }
