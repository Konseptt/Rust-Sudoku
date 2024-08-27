mod puzzle;
mod solver;

use solver::{solve, generate_sudoku};
use std::time::Instant;

fn main() {
    let mut puzzle = generate_sudoku();

    println!("Puzzle before solving:");
    puzzle.print();

    let start = Instant::now();
    let solved = solve(&mut puzzle);
    let duration = start.elapsed();

    if solved {
        println!("Puzzle after solving:");
        puzzle.print();
        println!("Solved in: {:?}", duration);
    } else {
        println!("No solution exists.");
    }
}