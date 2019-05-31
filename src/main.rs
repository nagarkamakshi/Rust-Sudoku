//This code is extended from piston crate tutorial
//for sudoku game.
//https://github.com/PistonDevelopers/Piston-Tutorials
// This work is released under the "MIT License".
// Please see the file LICENSE in the source
// distribution of this software for license terms.

//! A Sudoku game.

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, Filter, GlGraphics, GlyphCache, TextureSettings};

pub use board::Matrix9;
pub use sudokuboard_controller::SudokuboardController;
pub use sudokuboard_view::{SudokuboardView, SudokuboardViewSettings};

mod sudokuboard;
mod sudokuboard_controller;
mod sudokuboard_view;

fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Sudoku", [512; 2])
        .opengl(opengl)
        .exit_on_esc(true);
    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    let mut gl = GlGraphics::new(opengl);

    let sudokuboard = Matrix9::new();
    let mut sudokuboard_controller = SudokuboardController::new(sudokuboard.fillvalues());
    let sudokuboard_view_settings = SudokuboardViewSettings::new();
    let sudokuboard_view = SudokuboardView::new(sudokuboard_view_settings);

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    while let Some(e) = events.next(&mut window) {
        sudokuboard_controller.event(sudokuboard_view.settings.position,
                                   sudokuboard_view.settings.size,
                                   &e);
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                sudokuboard_view.draw(&sudokuboard_controller, glyphs, &c, g);
            });
        }
    }
}
