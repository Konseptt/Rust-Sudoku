# Sudoku Solver and Generator

This project is a Sudoku solver and generator written in Rust. It generates a random Sudoku puzzle, prints it, solves it with animation, and prints the solved puzzle along with the time it took to solve.

## Features

- **Generate Sudoku Puzzle**: Generates a random Sudoku puzzle with a specified number of empty cells.
- **Solve Sudoku Puzzle**: Solves the generated Sudoku puzzle using a backtracking algorithm.
- **Animation**: Displays the solving process step-by-step in the terminal.
- **Timing**: Measures and displays the time taken to solve the puzzle.

## Prerequisites

- Rust (latest stable version)

## Getting Started

### Clone the Repository

```sh
git clone https://github.com/yourusername/sudoku_solver.git
cd sudoku_solver
```

### Build the Project

```sh
cargo build
```

### Run the Project

```sh
cargo run
```

## Project Structure

- **`main.rs`**: Entry point of the application. It generates a Sudoku puzzle, prints it, solves it, and prints the solved puzzle along with the time taken.
- **`puzzle.rs`**: Contains the `Puzzle` struct and its associated methods.
- **`solver.rs`**: Contains the functions to solve and generate Sudoku puzzles.

## How It Works

1. **Generate Puzzle**: The program starts by generating a random Sudoku puzzle.
2. **Print Puzzle**: The generated puzzle is printed to the terminal.
3. **Solve Puzzle**: The solving process begins, and the program uses a backtracking algorithm to find the solution.
4. **Animation**: During the solving process, each step is displayed in the terminal to show the progress.
5. **Measure Time**: The time taken to solve the puzzle is measured and displayed.
6. **Print Solved Puzzle**: Once solved, the final puzzle is printed along with the time taken to solve it.

## Examples and Usage

Here are some examples and usage instructions to help you understand how to use the project effectively.

### Example 1: Generating and Solving a Sudoku Puzzle

```rust
use suko_solver::solver::{generate_sudoku, solve};
use suko_solver::puzzle::Puzzle;

fn main() {
    let mut puzzle = generate_sudoku();

    println!("Generated Sudoku Puzzle:");
    puzzle.print();

    if solve(&mut puzzle) {
        println!("Solved Sudoku Puzzle:");
        puzzle.print();
    } else {
        println!("No solution exists for the given Sudoku puzzle.");
    }
}
```

### Example 2: Customizing the Number of Empty Cells

You can customize the number of empty cells in the generated Sudoku puzzle by modifying the `generate_sudoku` function in `src/solver.rs`.

```rust
pub fn generate_sudoku() -> Puzzle {
    let mut grid = vec![vec![0; 9]; 9];
    solve_sudoku(&mut grid);

    // Remove numbers to create the puzzle
    let mut rng = thread_rng();
    for _ in 0..30 { // Change the number of empty cells here
        let row = rng.gen_range(0..9);
        let col = rng.gen_range(0..9);
        grid[row][col] = 0;
    }

    Puzzle::new(grid)
}
```

## Contributing

Contributions are welcome! Please follow the guidelines below to contribute to the project.

### Guidelines

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Make your changes and ensure that the code passes all tests.
4. Submit a pull request with a clear description of your changes.

### Code Style

- Use `rustfmt` to format your code.
- Follow the Rust API guidelines.

### Testing

- Add unit tests for new features and bug fixes.
- Run all tests to ensure that your changes do not break existing functionality.

### Pull Request Process

1. Ensure that your changes pass all tests.
2. Submit a pull request with a clear description of your changes.
3. Address any feedback or requested changes from the maintainers.

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contact

For any questions or suggestions, feel free to open an issue or contact the project maintainer.
