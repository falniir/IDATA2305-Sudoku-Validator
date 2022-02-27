//Struct which represents a 9x9 Sudoku
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy)]
pub struct Sudoku {
    pub grid: [[i32; 9]; 9],
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku { grid: [[0; 9]; 9] }
    }

    //Import a sudoku from a file and return it as a Sudoku struct
    pub fn from_file(filename: &str) -> Self {
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);

        let mut sudoku = Sudoku::new();
        let mut row = 0;

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            let line = line.trim_end_matches(",");

            if line.len() < 9 {
                panic!("Invalid line length: {}", line.len());
            }

            for (column, character) in line.chars().enumerate() {
                let character = character.to_digit(10).expect("Invalid character");
                sudoku.grid[row][column] = character as i32;
            }

            row += 1;
        }

        sudoku
    }

    //Get the next value for a cell
    pub fn get_next_value(&self, row: usize, column: usize) -> i32 {
        let mut values = [1, 2, 3, 4, 5, 6, 7, 8, 9];

        for i in 0..9 {
            if values[i] == values[row] {
                values[i] = 0;
            }
        }

        for i in 0..9 {
            if values[i] == values[column] {
                values[i] = 0;
            }
        }

        let row_start = (row / 3) * 3;
        let column_start = (column / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                if values[row_start + i] == values[column_start + j] {
                    values[row_start + i] = 0;
                }
            }
        }

        for i in 0..9 {
            if values[i] != 0 {
                return values[i];
            }
        }

        panic!("No valid value found");
    }

    //Validates that each row contains the digits 1 through 9 without any duplicates
    pub fn validate_row(&self, row: usize) -> bool {
        let mut values = [0; 9];
        let mut temp = vec![];
        for i in 0..9 {
            values[self.grid[row][i] as usize - 1] += 1;

            for j in 0..9 {
                if self.grid[row][i] == self.grid[row][j] && i != j {
                    temp.push(self.grid[row][i]);
                    temp.sort();
                    if temp != vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        panic!("Invalid row {}", row + 1);
                    }
                }
            }
        }

        true
    }

    //Validate Column of Sudoku
    pub fn validate_column(&self, column: usize) -> bool {
        let mut values = [0; 9];
        let mut temp = vec![];
        for i in 0..9 {
            values[self.grid[i][column] as usize - 1] += 1;
            for j in 0..9 {
                if self.grid[i][column] == self.grid[j][column] && i != j {
                    temp.push(self.grid[i][column]);
                    temp.sort();
                    if temp != vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                        panic!("Invalid column {}", column - 1);
                    }
                }
            }
        }

        true
    }

    //Validate 3x3 subgrid in a sudoku
    pub fn validate_grid(&self, row: usize, column: usize) -> bool {
        let mut values = [0; 9];
        let mut temp = vec![];
        let row_start = (row / 3) * 3;
        let column_start = (column / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                values[self.grid[row_start + i][column_start + j] as usize - 1] += 1;
                for k in 0..9 {
                    if self.grid[row_start + i][column_start + j]
                        == self.grid[row_start + k][column_start + j]
                        && i != k
                    {
                        temp.push(self.grid[row_start + i][column_start + j]);
                        temp.sort();
                        if temp != vec![1, 2, 3, 4, 5, 6, 7, 8, 9] {
                            panic!("Invalid grid {}", (row_start + i) * 9 + column_start + j);
                        }
                    }
                }
            }
        }

        true
    }
}
