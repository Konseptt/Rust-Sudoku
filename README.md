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

## License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.

## Contact

For any questions or suggestions, feel free to open an issue or contact the project maintainer.
