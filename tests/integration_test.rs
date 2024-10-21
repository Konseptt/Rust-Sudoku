use suko_solver::solver::{generate_sudoku, solve};
use suko_solver::puzzle::Puzzle;

#[test]
fn test_generate_and_solve_sudoku() {
    let mut puzzle = generate_sudoku();

    println!("Generated Sudoku Puzzle:");
    puzzle.print();

    let solved = solve(&mut puzzle);

    if solved {
        println!("Solved Sudoku Puzzle:");
        puzzle.print();
    } else {
        println!("No solution exists for the given Sudoku puzzle.");
    }

    assert!(solved, "The generated Sudoku puzzle could not be solved.");
}
