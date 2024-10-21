mod puzzle;
mod solver;

use solver::{solve, generate_sudoku};
use std::time::Instant;

/// Entry point of the application.
/// It generates a Sudoku puzzle, prints it, solves it, and prints the solved puzzle along with the time taken.
fn main() {
    // Generate a random Sudoku puzzle
    let mut puzzle = generate_sudoku();

    // Print the generated puzzle
    println!("Puzzle before solving:");
    puzzle.print();

    // Measure the time taken to solve the puzzle
    let start = Instant::now();
    let solved = solve(&mut puzzle);
    let duration = start.elapsed();

    // Print the solved puzzle and the time taken
    if solved {
        println!("Puzzle after solving:");
        puzzle.print();
        println!("Solved in: {:?}", duration);
    } else {
        println!("No solution exists.");
    }
}
