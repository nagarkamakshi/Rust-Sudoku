//! Sudokuboard controller.
//This code is extended from piston crate tutorial
//for sudoku game.
//https://github.com/PistonDevelopers/Piston-Tutorials
// This work is released under the "MIT License".
// Please see the file LICENSE in the source
// distribution of this software for license terms.
use piston::input::GenericEvent;

use crate::sudokuboard::Matrix9;

/// Handles events for Sudoku game.
pub struct SudokuboardController {
    /// Stores the sudokuboard state.
    pub sudokuboard: Matrix9,
    /// Selected cell.
    pub selected_cell: Option<[usize; 2]>,
    /// Stores last mouse cursor position.
    cursor_pos: [f64; 2],
}

impl SudokuboardController {
    /// Creates a new sudokuboard controller.
    pub fn new(sudokuboard: Matrix9) -> SudokuboardController {
        SudokuboardController {
            sudokuboard: sudokuboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.

    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: f64, e: &E) {
        use piston::input::{Button, Key, MouseButton};

        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            // Check that coordinates are inside board boundaries.
            if x >= 0.0 && x < size && y >= 0.0 && y < size {
                // Compute the cell position.
                let cell_x = (x / size * 9.0) as usize;
                let cell_y = (y / size * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
            }
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if let Some(ind) = self.selected_cell {
                // Set cell value.
                match key {
                    Key::D1 => self.sudokuboard.set(ind, 1) ,
                    Key::D2 => self.sudokuboard.set(ind, 2),
                    Key::D3 => self.sudokuboard.set(ind, 3),
                    Key::D4 => self.sudokuboard.set(ind, 4),
                    Key::D5 => self.sudokuboard.set(ind, 5),
                    Key::D6 => self.sudokuboard.set(ind, 6),
                    Key::D7 => self.sudokuboard.set(ind, 7),
                    Key::D8 => self.sudokuboard.set(ind, 8),
                    Key::D9 => self.sudokuboard.set(ind, 9),
                    Key::S => self.sudokuboard.print_solution(),
                    Key::N => self.sudokuboard.generate(),
                    Key::B => self.sudokuboard.backspace(ind),
                    _ => {}
                }
            }
        }
    }
}
