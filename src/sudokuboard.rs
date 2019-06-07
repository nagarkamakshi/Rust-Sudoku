//! # Sudoku Game
//!
//! A 9*9 Sudoku Puzzle
use rand::prelude::*; ///for generating random numbers
static mut PUZZLE : [[u8; 9]; 9]  = [[0; 9]; 9];
#[derive(Debug, Clone, Copy,PartialEq)]
/// A 9*9 grid for Sudoku is represented here.
///
pub struct Matrix9 {
    /// Stires sudokuboard data
    pub data: [[u8; 9]; 9],
}
impl Matrix9{
    /// Returns a Matrix with the data given to it
    ///
    /// # Arguments
    ///
    /// * `data` - A 9*9 array that holds the number 1..9 of the Matrix
    ///
    /// # Example
    ///
    /// let sudoku = Matrix9::new();
    /// returns a matrix filled with 0's in it.
    /// ```
    pub fn new()->Self{
        Self{data:[[0;9];9]}
    }
    /// Gets the character at cell location.
    pub fn char(&self, ind: [usize; 2]) -> Option<char> {
            Some(match self.data[ind[1]][ind[0]] {
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => return None,
            })
        }
    /// Set cell value.
    pub fn set(&mut self, ind: [usize; 2], val: u8) {
        if self.check_safe(ind[1],ind[0],val) {
            self.data[ind[1]][ind[0]] = val;
        }
        else{
            println!("this number is not valid");
        }
    }
    /// fill the grid with non-conflicting numbers
    /// to generate a valid sudoku
    /// https://stackoverflow.com/questions/4066075/proper-data-structure-to-represent-a-sudoku-puzzle
    pub fn fillvalues(&mut self)->Self{
        let v = [1,2,3,4,5,6,7,8,9,1,2,3,4,5,6,7,8,9];
        for b in (0..).step_by(3).take(3){
        for row in 0..3{
            for col in 0..9{
                self.data[b+row][col] = v[col+(row*3)+(b/3)];
            }
        }
        }
        Self{data:self.data}
    }
    ///Another approach the fill the grid
    ///It fills the diagonal boxes Randomly
    /// then backtracking can be using to fill the rest
    pub fn _fill_diagonal_values(&mut self) -> Self {
        let mut rng = thread_rng();
        let numbers: Vec<_> = (1..10).collect();
        let mut v: Vec<u8> = Vec::new();
        let mut v2: Vec<u8> = Vec::new();
        let mut v3: Vec<u8> = Vec::new();
        let mut number = numbers.choose(&mut rng).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                while v.iter().any(|c| c == number) {
                    number = numbers.choose(&mut rng).unwrap();
                }
                self.data[i][j] = *number;
                v.push(*number);
            }
        }
        for i in 3..6 {
            for j in 3..6 {
                while v2.iter().any(|c| c == number) {
                    number = numbers.choose(&mut rng).unwrap();
                }
                self.data[i][j] = *number;
                v2.push(*number);
            }
        }
        for i in 6..9 {
            for j in 6..9 {
                while v3.iter().any(|c| c == number) {
                    number = numbers.choose(&mut rng).unwrap();
                }
                self.data[i][j] = *number;
                v3.push(*number);
            }
        }

        Self { data: self.data }
    }
    ///once grid is full with valid numbers
    ///shuffle them Randomly to get a unique sudoku
    pub fn shuffle(&mut self) -> Self{
        for _ in 0..500{
        let v = &mut Matrix9::random_shuffle();
        let first = v[1] as usize +v[2] as usize;
        let second = v[1] as usize +v[3] as usize;
        //swaping  the rows
        if v[0] == 1{
            for col in 0..9{
            let temp = self.data[first][col];
            self.data[first][col]=self.data[second][col];
            self.data[second][col]=temp;
            }
        }
        //swapping the colums
        else if v[0] == 2{
            for row in 0..9{
            let temp = self.data[row][first];
            self.data[row][first]=self.data[row][second];
            self.data[row][second]=temp;
            }
        }
        }
        Self{data:self.data}
    }
    ///helper function for shuffle()
    ///Randomly pick if row,column or values in boxes need to shuffle
    pub fn random_shuffle()-> [u8;4] {
        let mut rng = thread_rng();
        //Randomly pick if row,column or values in boxes need to shuffle
        let row_or_col = [1,2].choose(&mut rng).unwrap();
        let box_num = [0,3,6].choose(&mut rng).unwrap();
        let choice1 = [0,1,2].choose(&mut rng).unwrap();
        let mut choice2 =  [0,1,2].choose(&mut rng).unwrap();
            while choice2==choice1{
                    choice2 = [0,1,2].choose(&mut rng).unwrap();
                }
        let v = [*row_or_col,*box_num,*choice1,*choice2];
        v
    }
    ///once the grid is shuffled remove some numbers
    ///in order to get Sudoku ready to play
    pub fn remove_random(&mut self) ->Self{
        self.fillvalues();
        self.shuffle();
        let numbers:Vec<_> = (0..9).collect();
        let mut rng = thread_rng();
        for _ in 0..59{
            //Randomly pick if row,column or values in boxes need to shuffle
            let row  = numbers.choose(&mut rng).unwrap();
            let col =  numbers.choose(&mut rng).unwrap();
            self.data[*row][*col]= 0;
        }
        // Safety: There will always be some value in the PUZZLE
        unsafe{
            PUZZLE = self.data;
        }
        Self{data:self.data}
    }
    ///helper function for solver()
    ///returns true if a number is valid for a given cell
    pub fn check_safe(&self,row:usize,col:usize,num:u8)->bool{
        if self.check_rows(row,num)
            && self.check_cols(col,num)
            && self.check_box(row - row % 3,col - col % 3,num)
            && self.data[row][col]==0{
                //println!("{} is ok ", num );
            return true;
        }
        false
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given row
    pub fn check_rows(&self,row:usize,num: u8) -> bool {
        for col in 0..9 {
            if self.data[row][col] == num { return false; }
        }
        true
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given column
    pub fn check_cols(&self,col:usize,num: u8) -> bool {
        for row in 0..9 {
            if self.data[row][col] == num { return false; }
        }
        true
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given square (3*3)
    pub fn check_box(&self,brow:usize,bcol:usize,num: u8) -> bool {
        for row in 0..3{
            for col in 0..3{
                if self.data[row+brow][col+bcol] == num {return false; }
            }
        }
        true
    }
    /// helper function for solver()
    /// returns (row,col) of the empty cell
    pub fn find_empty(&self)->(usize,usize){
        for i in 0..9{
            for j in 0..9{
                if self.data[i][j]==0 {return (i,j);}
            }
        }
        (9,9)
    }
    /// solves the sudoku using backtracking
    /// returns true if there is solution for a given sudoku
    pub fn solver(&mut self) -> bool{
        let x = self.find_empty();
        if x==(9,9) {
        return true;
         }
        let row = x.0;
        let col = x.1;
            for num in 1..10{
                if self.check_safe(row,col,num){
                    self.data[row][col]=num;
                    if self.solver(){
                        return true;
                    }
                    self.data[row][col]=0;
                    }
                }
            return false;
        }
    /// prints the solution of the sudoku
    pub fn print_solution(&mut self){
        if !self.solver(){
            println!("some entries prevent backtracking" );
        }
    }
    /// Generate a new sudoku
    pub fn generate(&mut self){
        self.fillvalues();
        self.shuffle();
        self.remove_random();
    }
    /// Set the cell data to 0
    pub fn backspace(&mut self, ind: [usize; 2]){
            unsafe{
                if PUZZLE[ind[1]][ind[0]]==0{
                self.data[ind[1]][ind[0]] = 0;
            }
            else{
                println!("cant remove fixed cell" );
            }
        }
    }
}
