use std::io::stdin;
use std::thread;

use Sudoku_Validator::Sudoku;

fn main() {
    //Parent thread which worker threads are spawned from.
    //Spawns 3 worker threads.
    //One thread to check  that each column contains the digits 1 through 9
    //One thread to check that each row contains the digits 1 through 9
    //One thread to check that each 3x3 subgrid contains the digits 1 through 9
    //The parent thread will wait for all 3 worker threads to finish before exiting.
    //The parent thread will also print the result of each worker thread.
    //Nothing will be printed if all worker threads return true.
    
    let mut parent_thread = vec![];

    let mut input_string = String::new();
    stdin()
        .read_line(&mut input_string)
        .ok()
        .expect("Failed to read line");

    let len = input_string.len();

    input_string.truncate(len - 1);

    let file_name = "src/".to_string() + &input_string + ".csv";
    let sudoku = Sudoku::from_file(&file_name);

    for i in 0..9 {
        let sudoku = sudoku.clone();
        parent_thread.push(thread::spawn(move || {
            sudoku.validate_row(i);
        }));
    }

    for i in 0..9 {
        let sudoku = sudoku.clone();
        parent_thread.push(thread::spawn(move || {
            sudoku.validate_column(i);
        }));
    }

    for i in 0..3 {
        for j in 0..3 {
            let sudoku = sudoku.clone();
            parent_thread.push(thread::spawn(move || {
                sudoku.validate_grid(i, j);
            }));
        }
    }

    println!("{:?}", sudoku);
}
