use crate::puzzle::Puzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

/// Solves the given Sudoku puzzle.
///
/// # Arguments
///
/// * `puzzle` - A mutable reference to the `Puzzle` to be solved.
///
/// # Returns
///
/// `true` if the puzzle is solved successfully, `false` otherwise.
pub fn solve(puzzle: &mut Puzzle) -> bool {
    let grid = puzzle.grid_mut();
    solve_sudoku(grid)
}

/// Solves the Sudoku puzzle using backtracking algorithm.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the 2D vector representing the Sudoku grid.
///
/// # Returns
///
/// `true` if the puzzle is solved successfully, `false` otherwise.
fn solve_sudoku(grid: &mut Vec<Vec<u8>>) -> bool {
    let empty_cells = find_empty_cells(grid);
    solve_sudoku_with_empty_cells(grid, &empty_cells)
}

/// Solves the Sudoku puzzle with the given empty cells using backtracking algorithm.
///
/// # Arguments
///
/// * `grid` - A mutable reference to the 2D vector representing the Sudoku grid.
/// * `empty_cells` - A slice of tuples representing the coordinates of empty cells.
///
/// # Returns
///
/// `true` if the puzzle is solved successfully, `false` otherwise.
fn solve_sudoku_with_empty_cells(grid: &mut Vec<Vec<u8>>, empty_cells: &[(usize, usize)]) -> bool {
    if empty_cells.is_empty() {
        return true; // No empty cell means the puzzle is solved
    }

    let (row, col) = empty_cells[0];
    let remaining_cells = &empty_cells[1..];

    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(&mut thread_rng());

    for &num in &nums {
        if is_valid(grid, row, col, num) {
            grid[row][col] = num;
            if solve_sudoku_with_empty_cells(grid, remaining_cells) {
                return true;
            }
            grid[row][col] = 0; // Reset on backtrack
        }
    }

    false
}

/// Finds the empty cells in the Sudoku grid.
///
/// # Arguments
///
/// * `grid` - A reference to the 2D vector representing the Sudoku grid.
///
/// # Returns
///
/// A vector of tuples representing the coordinates of empty cells.
fn find_empty_cells(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut empty_cells = Vec::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 0 {
                empty_cells.push((i, j));
            }
        }
    }
    empty_cells
}

/// Checks if placing a number in the given cell is valid.
///
/// # Arguments
///
/// * `grid` - A reference to the 2D vector representing the Sudoku grid.
/// * `row` - The row index of the cell.
/// * `col` - The column index of the cell.
/// * `num` - The number to be placed in the cell.
///
/// # Returns
///
/// `true` if the number can be placed in the cell, `false` otherwise.
fn is_valid(grid: &Vec<Vec<u8>>, row: usize, col: usize, num: u8) -> bool {
    // Check row
    if grid[row].contains(&num) {
        return false;
    }

    // Check column
    if grid.iter().any(|r| r[col] == num) {
        return false;
    }

    // Check 3x3 subgrid
    let (box_row, box_col) = (row / 3 * 3, col / 3 * 3);
    for i in 0..3 {
        for j in 0..3 {
            if grid[box_row + i][box_col + j] == num {
                return false;
            }
        }
    }

    true
}

/// Generates a random Sudoku puzzle.
///
/// # Returns
///
/// A new `Puzzle` instance with a randomly generated Sudoku puzzle.
pub fn generate_sudoku() -> Puzzle {
    let mut grid = vec![vec![0; 9]; 9];
    solve_sudoku(&mut grid);

    // Remove numbers to create the puzzle
    let mut rng = thread_rng();
    for _ in 0..40 { // Remove 40 numbers
        let row = rng.gen_range(0..9);
        let col = rng.gen_range(0..9);
        grid[row][col] = 0;
    }

    Puzzle::new(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let mut puzzle = Puzzle::new(vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ]);

        assert!(solve(&mut puzzle));
    }

    #[test]
    fn test_generate_sudoku() {
        let puzzle = generate_sudoku();
        assert!(puzzle.grid_mut().iter().flatten().filter(|&&cell| cell == 0).count() >= 40);
    }

    #[test]
    fn test_is_valid() {
        let grid = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        assert!(is_valid(&grid, 0, 2, 4));
        assert!(!is_valid(&grid, 0, 2, 5));
    }

    #[test]
    fn test_find_empty_cells() {
        let grid = vec![
            vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
            vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
            vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
            vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
            vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
            vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
            vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
            vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
            vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];

        let empty_cells = find_empty_cells(&grid);
        assert_eq!(empty_cells.len(), 51);
    }
}
