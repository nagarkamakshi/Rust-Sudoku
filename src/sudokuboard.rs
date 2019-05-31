
use rand::prelude::*; ///for generating random numbers
//use ncurses::*;

#[derive(Debug, Clone, Copy)]
/// A 9*9 grid for Sudoku is represented here.
///
///
pub struct Matrix9 {
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
        Self{data: [[0;9];9]}
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
            println!("index {:?}", ind);
            println!("data {:?}",self.data);
            self.data[ind[1]][ind[0]] = val;
    }
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
    pub fn shuffle(&mut self) -> Self{
        let v = &mut Matrix9::random_shuffle();
        //println!("array is {:?}",v );
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
        /*for row in 0..2{
                self.data.swap(row,row+1);
        } */
        Self{data:self.data}
    }
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
    pub fn remove_random(&mut self) ->Self{
        let numbers:Vec<_> = (0..9).collect();
        //let mut ok_cells:Vec<_> = vec![];
        //println!("numbers= {:?}",numbers );
        let mut rng = thread_rng();
        for _ in 0..59{
            //Randomly pick if row,column or values in boxes need to shuffle
            let row  = numbers.choose(&mut rng).unwrap();
            let col =  numbers.choose(&mut rng).unwrap();
            //let temp = self.data[*row][*col];
            self.data[*row][*col]= 0;
            /*if !self.solver(){
                println!("this removal is not ok" );
                //self.data[*row][*col]= temp;
            }
            else{
                ok_cells.push((*row,*col));
                println!("cells are ok to remove {:?}",ok_cells);
            } */
        }
        //for i in ok_cells.iter(){

        //}
        Self{data:self.data}
    }
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
    pub fn check_rows(&self,row:usize,num: u8) -> bool {
        for col in 0..9 {
            if self.data[row][col] == num { return false; }
        }
        true
    }
    pub fn check_cols(&self,col:usize,num: u8) -> bool {
        for row in 0..9 {
            if self.data[row][col] == num { return false; }
        }
        true
    }
    pub fn check_box(&self,brow:usize,bcol:usize,num: u8) -> bool {
        for row in 0..3{
            for col in 0..3{
                if self.data[row+brow][col+bcol] == num {return false; }
            }
        }
        true
    }
    pub fn find_empty(&self)->(usize,usize){
        for i in 0..9{
            for j in 0..9{
                if self.data[i][j]==0 {return (i,j);}
            }
        }
        (9,9)
    }
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

}
