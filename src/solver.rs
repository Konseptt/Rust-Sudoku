use crate::puzzle::Puzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

/// Solves the given Sudoku puzzle using an optimized backtracking algorithm
/// with constraint propagation and MRV heuristic.
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
    solve_sudoku_optimized(grid)
}

/// Represents constraint state using bitsets for efficient validation.
struct ConstraintState {
    rows: [u16; 9],    // Bitset for each row (bits 1-9 represent numbers 1-9)
    cols: [u16; 9],    // Bitset for each column
    boxes: [u16; 9],   // Bitset for each 3x3 box
}

impl ConstraintState {
    /// Creates a new constraint state from the current grid.
    fn from_grid(grid: &Vec<Vec<u8>>) -> Self {
        let mut state = ConstraintState {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
        };

        for (i, row) in grid.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell != 0 {
                    let bit = 1u16 << cell;
                    state.rows[i] |= bit;
                    state.cols[j] |= bit;
                    let box_idx = (i / 3) * 3 + (j / 3);
                    state.boxes[box_idx] |= bit;
                }
            }
        }
        state
    }

    /// Places a number at the given position.
    #[inline]
    fn place(&mut self, row: usize, col: usize, num: u8) {
        let bit = 1u16 << num;
        self.rows[row] |= bit;
        self.cols[col] |= bit;
        let box_idx = (row / 3) * 3 + (col / 3);
        self.boxes[box_idx] |= bit;
    }

    /// Removes a number from the given position.
    #[inline]
    fn remove(&mut self, row: usize, col: usize, num: u8) {
        let bit = 1u16 << num;
        self.rows[row] &= !bit;
        self.cols[col] &= !bit;
        let box_idx = (row / 3) * 3 + (col / 3);
        self.boxes[box_idx] &= !bit;
    }

    /// Gets possible values for a cell using bitsets.
    #[inline]
    fn get_possible_values(&self, row: usize, col: usize) -> u16 {
        let box_idx = (row / 3) * 3 + (col / 3);
        let used = self.rows[row] | self.cols[col] | self.boxes[box_idx];
        // All bits 1-9 minus used bits
        0b1111111110 & !used
    }

    /// Counts the number of possible values for a cell.
    #[inline]
    fn count_possible(&self, row: usize, col: usize) -> u32 {
        self.get_possible_values(row, col).count_ones()
    }
}

/// Optimized Sudoku solver using constraint propagation and MRV heuristic.
fn solve_sudoku_optimized(grid: &mut Vec<Vec<u8>>) -> bool {
    let mut state = ConstraintState::from_grid(grid);
    solve_with_constraints(grid, &mut state)
}

/// Recursive solver with constraint tracking.
fn solve_with_constraints(grid: &mut Vec<Vec<u8>>, state: &mut ConstraintState) -> bool {
    // Find the best cell to fill using MRV heuristic
    let mut best_cell: Option<(usize, usize)> = None;
    let mut min_choices = 10;

    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                let choices = state.count_possible(i, j);
                if choices == 0 {
                    return false; // No valid values, backtrack immediately
                }
                if choices < min_choices {
                    min_choices = choices;
                    best_cell = Some((i, j));
                    if min_choices == 1 {
                        // Can't do better than 1 choice, use this cell
                        break;
                    }
                }
            }
        }
        if min_choices == 1 {
            break;
        }
    }

    // If no empty cell found, puzzle is solved
    let (row, col) = match best_cell {
        Some(cell) => cell,
        None => return true,
    };

    // Try each possible value
    let possible = state.get_possible_values(row, col);
    for num in 1..=9u8 {
        let bit = 1u16 << num;
        if (possible & bit) != 0 {
            // Place the number
            grid[row][col] = num;
            state.place(row, col, num);

            // Recursively solve
            if solve_with_constraints(grid, state) {
                return true;
            }

            // Backtrack
            grid[row][col] = 0;
            state.remove(row, col, num);
        }
    }

    false
}

/// Legacy validation function (kept for compatibility but not used in optimized solver).
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
    
    // Use a randomized solver for generation to create variety
    generate_filled_grid(&mut grid);

    // Remove numbers to create the puzzle (ensuring 40 unique cells are removed)
    let mut rng = thread_rng();
    let mut removed = 0;
    while removed < 40 {
        let row = rng.gen_range(0..9);
        let col = rng.gen_range(0..9);
        if grid[row][col] != 0 {
            grid[row][col] = 0;
            removed += 1;
        }
    }

    Puzzle::new(grid)
}

/// Helper function to generate a filled Sudoku grid with randomization.
fn generate_filled_grid(grid: &mut Vec<Vec<u8>>) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            if grid[i][j] == 0 {
                let mut nums: Vec<u8> = (1..=9).collect();
                nums.shuffle(&mut thread_rng());
                
                for &num in &nums {
                    if is_valid(grid, i, j, num) {
                        grid[i][j] = num;
                        if generate_filled_grid(grid) {
                            return true;
                        }
                        grid[i][j] = 0;
                    }
                }
                return false;
            }
        }
    }
    true
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
        assert!(puzzle.grid().iter().flatten().filter(|&&cell| cell == 0).count() >= 40);
    }
}
