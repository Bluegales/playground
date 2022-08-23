use colored::*;

pub trait Game {
    fn new() -> Self;
    fn dimension(&self) -> (i32, i32);
    fn size(&self) -> i32;
    fn print_field(&self, x: i32, y: i32) -> ();
    fn print_border(&self, x: i32, y: i32) {
        print!("{}", "  ".on_bright_green());
    }
    fn print_board(&self) {
        let (x_max, y_max) = self.dimension();
        let board_x = x_max * 2 + 1;
        let board_y = y_max * 2 + 1;
        // TOP ROW
        for x in 0..board_x {
            if x == 0 {
                print!("╔");
            } else if x == board_x - 1 {
                print!("╗");
            } else if x % 2 == 0 {
                print!("╤");
            } else {
                print!("═══");
            }
        }
        print!("\n");
        // CENTER
        for y in 0..board_y - 2{
            let y_row = y % 2;
            for x in 0..board_x {
                let x_row = x % 2;
                if x == 0 {
                    if y_row == 1 {
                        print!("╟");
                    } else {
                        print!("║");
                    }
                } else if x == board_x - 1 {
                    if y_row == 1 {
                        print!("╢");
                    } else {
                        print!("║");
                    }
                } else if y_row == 1 && x_row == 0 {
                    print!("┼");
                } else if x_row == 0 {
                    print!("│");
                } else if y_row == 1 {
                    print!("───");
                } else {
                    print!(" {} ", "♜");
                }
            }
            print!("\n");
        }
        // BOTTOM ROW
        for x in 0..board_x {
            if x == 0 {
                print!("╚");
            } else if x == board_x - 1 {
                print!("╝");
            } else if x % 2 == 0 {
                print!("╧");
            } else {
                print!("═══");
            }
        }
        print!("\n");
    }

    //std::Vec<i32> board;
    // fn get_input(&self);
}
