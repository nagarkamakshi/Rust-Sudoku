
use rand::prelude::*;///for generating random numbers

#[derive(Debug, Clone, Copy,PartialEq)]
/// A 9*9 grid for Sudoku is represented here.
///
///
struct Matrix9 {
    data: [[u8; 9]; 9],
}
impl Matrix9 {
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
    fn new() -> Self {
        Self { data: [[0; 9]; 9] }
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
        self.data[ind[1]][ind[0]] = val;
    }
    
    ///fill the grid with non-conflicting numbers
    ///to generate a valid sudoku
    ///https://stackoverflow.com/questions/4066075/proper-data-structure-to-represent-a-sudoku-puzzle
    fn fillvalues(&mut self) -> Self {
        let v = [1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for b in (0..).step_by(3).take(3) {
            for row in 0..3 {
                for col in 0..9 {
                    self.data[b + row][col] = v[col + (row * 3) + (b / 3)];
                }
            }
        }
        Self { data: self.data }
    }
    ///once grid is full with valid numbers
    ///shuffle them Randomly to get a new sudoku
    fn shuffle(&mut self) -> Self {
        let v = &mut Matrix9::random_shuffle();
        //println!("array is {:?}",v );
        let first = v[1] as usize + v[2] as usize;
        let second = v[1] as usize + v[3] as usize;
        //swaping  the rows
        if v[0] == 1 {
            for col in 0..9 {
                let temp = self.data[first][col];
                self.data[first][col] = self.data[second][col];
                self.data[second][col] = temp;
            }
        }
        //swapping the colums
        else if v[0] == 2 {
            for row in 0..9 {
                let temp = self.data[row][first];
                self.data[row][first] = self.data[row][second];
                self.data[row][second] = temp;
            }
        }
        Self { data: self.data }
    }
    ///helper function for shuffle()
    ///Randomly pick if row,column or values in boxes need to shuffle
    fn random_shuffle() -> [u8; 4] {
        let mut rng = thread_rng();

        let row_or_col = [1, 2].choose(&mut rng).unwrap();
        let box_num = [0, 3, 6].choose(&mut rng).unwrap();
        let choice1 = [0, 1, 2].choose(&mut rng).unwrap();
        let mut choice2 = [0, 1, 2].choose(&mut rng).unwrap();
        while choice2 == choice1 {
            choice2 = [0, 1, 2].choose(&mut rng).unwrap();
        }
        [*row_or_col, *box_num, *choice1, *choice2]
    }
    ///once the grid is shuffled remove some numbers
    ///in order to get Sudoku ready to play
    fn remove_random(&mut self) -> Self {
        let numbers: Vec<_> = (0..9).collect();
        //let mut ok_cells:Vec<_> = vec![];
        //println!("numbers= {:?}",numbers );
        let mut rng = thread_rng();
        for _ in 0..59 {
            //Randomly pick if row,column or values in boxes need to shuffle
            let row = numbers.choose(&mut rng).unwrap();
            let col = numbers.choose(&mut rng).unwrap();
            self.data[*row][*col] = 0;
        }
        Self { data: self.data }
    }
    ///helper function for solver()
    ///returns true if a number is valid for a given cell
    fn check_safe(&self, row: usize, col: usize, num: u8) -> bool {
            if num>9{
                return false;
            }
            else{
            return self.check_rows(row, num)
            && self.check_cols(col, num)
            && self.check_box(row - row % 3, col - col % 3, num)
        }
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given row
    fn check_rows(&self, row: usize, num: u8) -> bool {
        for col in 0..9 {
            if self.data[row][col] == num {
                return false;
            }
        }
        true
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given column
    fn check_cols(&self, col: usize, num: u8) -> bool {
        for row in 0..9 {
            if self.data[row][col] == num {
                return false;
            }
        }
        true
    }
    /// helper function for check_safe()
    /// returns true if number is valid for the given square (3*3)
    fn check_box(&self, brow: usize, bcol: usize, num: u8) -> bool {
        for row in 0..3 {
            for col in 0..3 {
                if self.data[row + brow][col + bcol] == num {
                    return false;
                }
            }
        }
        true
    }
    /// helper function for solver()
    /// returns (row,col) of the empty cell
    fn find_empty(&self) -> (usize, usize) {
        for i in 0..9 {
            for j in 0..9 {
                if self.data[i][j] == 0 {
                    return (i, j);
                }
            }
        }
        (9, 9)
    }
    /// solves the sudoku using backtracking
    /// returns true if there is solution for a given sudoku
    fn solver(&mut self) -> bool {
        let x = self.find_empty();
        if x == (9, 9) {
            return true;
        }
        let row = x.0;
        let col = x.1;
        for num in 1..10 {
            if self.check_safe(row, col, num) {
                self.data[row][col] = num;
                if self.solver() {
                    return true;
                }
                self.data[row][col] = 0;
            }
        }
        false
    }

}

