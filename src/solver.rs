use crate::puzzle::Puzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

pub fn solve(puzzle: &mut Puzzle) -> bool {
    let grid = puzzle.grid_mut();
    solve_sudoku(grid)
}

fn solve_sudoku(grid: &mut Vec<Vec<u8>>) -> bool {
    let empty_cells = find_empty_cells(grid);
    solve_sudoku_with_empty_cells(grid, &empty_cells)
}

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