use super::cell::Cell;
use super::cell::CellValue;
use ncurses::{clear, endwin, getch, initscr, printw, refresh};
use std::io::{stdout, Write};

fn clear_screen() {
    print!("\x1B[2J");
    stdout().flush().unwrap();
}

pub struct Board {
    board: [[Cell; 8]; 8],
    current_pointer: [i8; 2],
    selected_cell: [i8; 2],
    selected_value: CellValue,
}

impl Board {
    pub fn new() -> Board {
        let mut new_board = [[Cell::new(CellValue::Black); 8]; 8];
        for row in 0..8 {
            for col in 0..8 {
                if (row + col) % 2 == 0 {
                    continue;
                }
                if row < 2 {
                    new_board[row][col].set_value(CellValue::Red);
                    continue;
                }
                if row > 5 {
                    new_board[row][col].set_value(CellValue::Blue);
                    continue;
                }
                new_board[row][col].set_value(CellValue::Empty);
            }
        }

        new_board[0][1].set_selected();

        return Board {
            board: new_board,
            current_pointer: [1, 0],
            selected_cell: [0, 0],
            selected_value: CellValue::Black,
        };
    }
    pub fn render_board(&self) {
        for row in self.board.iter() {
            for cell in row {
                printw(cell.render_top_or_bottom().as_str());
            }
            printw("\n");
            for cell in row {
                printw(cell.render_mid().as_str());
            }
            printw("\n");
            for cell in row {
                printw(cell.render_top_or_bottom().as_str());
            }
            printw("\n");
        }
    }

    fn select_current_pointer(&mut self) {
        self.board[self.current_pointer[1] as usize][self.current_pointer[0] as usize]
            .set_selected();
    }

    fn deselect_current_pointer(&mut self) {
        self.board[self.current_pointer[1] as usize][self.current_pointer[0] as usize]
            .set_deselected();
    }

    fn move_pointer_y(&mut self, value: i8) {
        let new_y = self.current_pointer[1] + value;
        if new_y > 7 || new_y < 0 {
            return;
        }
        self.deselect_current_pointer();
        self.current_pointer[1] = new_y;
        if self.current_pointer[0] % 2 == 1 {
            //odd
            self.current_pointer[0] = self.current_pointer[0] - value.abs();
        } else {
            self.current_pointer[0] = self.current_pointer[0] + value.abs();
        }
        self.select_current_pointer();
    }

    fn move_pointer_x(&mut self, value: i8) {
        let new_x = self.current_pointer[0] + 2 * value;
        if new_x > 7 || new_x < 0 {
            return;
        }
        self.deselect_current_pointer();
        self.current_pointer[0] = new_x;
        self.select_current_pointer();
    }

    fn select_cell(&mut self) {
        let current_selcted_value = self.board[self.current_pointer[1] as usize]
            [self.current_pointer[0] as usize]
            .get_value();
        if (self.board[self.current_pointer[1] as usize][self.current_pointer[0] as usize]
            .can_set_value(current_selcted_value))
        {
            return;
        }
        self.selected_cell = self.current_pointer;
        self.selected_value = current_selcted_value;
    }

    fn move_piece(&mut self) {
        let diffX = (self.current_pointer[0] - self.selected_cell[0]).abs();
        let diffY = (self.current_pointer[1] - self.selected_cell[1]).abs();

        if diffX < 1 || diffX > 2 || diffY < 1 || diffY > 2 || diffY != diffX {
            return;
        }
        if (diffY == 2) {
            let midX = (self.current_pointer[0] + self.selected_cell[0]) / 2;
            let midY = (self.current_pointer[1] + self.selected_cell[1]) / 2;
            if is_same_player(
                selected_value,
                self.board[midY as usize][midX as usize].get_value(),
            ) {
                return;
            }
            self.board[midY as usize][midX as usize].set_value(CellValue::Empty);
        }

        if self.board[self.current_pointer[1] as usize][self.current_pointer[0] as usize]
            .can_set_value(self.selected_value)
        {
            self.board[self.selected_cell[1] as usize][self.selected_cell[0] as usize]
                .set_value(CellValue::Empty);
            self.board[self.current_pointer[1] as usize][self.current_pointer[0] as usize]
                .set_value(self.selected_value);
            self.selected_value = CellValue::Black;
        };
    }

    fn handle_keyboard(&mut self, input: i32) {
        printw(input.to_string().as_str());
        match input {
            65 => self.move_pointer_y(-1),
            66 => self.move_pointer_y(1),
            68 => self.move_pointer_x(-1),
            67 => self.move_pointer_x(1),
            10 => {
                if (self.selected_value == CellValue::Black) {
                    self.select_cell();
                } else {
                    self.move_piece();
                }
            }
            _ => return,
        }
    }

    pub fn game_loop(&mut self) {
        initscr();
        loop {
            clear();
            self.render_board();
            refresh();
            let character = getch();
            self.handle_keyboard(character);
        }
        endwin();
    }
}
