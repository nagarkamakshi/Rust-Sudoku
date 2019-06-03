# Rust-Sudoku

Copyright (c) 2019 Kamakshi Nagar

Sudoku is a popular puzzle game where you get a 9 by 9 grid.  
Every row and every column contains numbers from 1-9 without repetition.
Also, each 3 by 3 grid has numbers 1-9 without repetition.

# Building Instructions
To build this repository, you need Cargo.

You also need to have rand and piston crates. Available at crates.io, Check Cargo.toml

Clone this repository

git clone https://github.com/nagarkamakshi/Rust-Sudoku.git

Use Cargo to build :

cargo build

To Test:

cargo test

To read the documentation:

cargo doc --open

To Play:

cargo run

1. use the mouse curser to point to the cell, fill the numbers using digit keys(1-9).
2. B - BackSpace
3. N - New Game
4. S - Solve the puzzle


# Current Status:

Game has restriction that it doesnt let you fill numbers that are confilt any row, column or box.

If the numbers prevents the backtracking the solver doent work.

# Future Goal:

1. Change the color of the confiling cells.
2. Include Timer for scoring purpose.
3. difficuly level for generating puzzles.

# Author Information

Kamakshi Nagar 
kamakshi@pdx.edu

# License

This work is released under the "MIT License". Please see
the file `LICENSE` in this distribution for license terms.



