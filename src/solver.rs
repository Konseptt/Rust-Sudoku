use crate::puzzle::Puzzle;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

/// Solves the given Sudoku puzzle using an ultra-optimized backtracking algorithm
/// with constraint propagation, naked/hidden singles, and MRV heuristic.
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
    
    // Convert to flat array for better cache locality
    let mut flat_grid = [0u8; 81];
    for i in 0..9 {
        for j in 0..9 {
            flat_grid[i * 9 + j] = grid[i][j];
        }
    }
    
    let result = solve_sudoku_ultra(& mut flat_grid);
    
    // Convert back to 2D grid
    if result {
        for i in 0..9 {
            for j in 0..9 {
                grid[i][j] = flat_grid[i * 9 + j];
            }
        }
    }
    
    result
}

/// Represents constraint state using bitsets for efficient validation.
/// Uses bitmasks where bits 1-9 represent numbers 1-9.
#[derive(Clone)]
struct ConstraintState {
    rows: [u16; 9],    // Bitset for each row
    cols: [u16; 9],    // Bitset for each column
    boxes: [u16; 9],   // Bitset for each 3x3 box
}

impl ConstraintState {
    /// Creates a new constraint state from a flat grid.
    #[inline]
    fn from_grid(grid: &[u8; 81]) -> Self {
        let mut state = ConstraintState {
            rows: [0; 9],
            cols: [0; 9],
            boxes: [0; 9],
        };

        for (idx, &cell) in grid.iter().enumerate().take(81) {
            if cell != 0 {
                let (row, col) = (idx / 9, idx % 9);
                let bit = 1u16 << cell;
                state.rows[row] |= bit;
                state.cols[col] |= bit;
                let box_idx = (row / 3) * 3 + (col / 3);
                state.boxes[box_idx] |= bit;
            }
        }
        state
    }

    /// Places a number at the given position.
    #[inline(always)]
    fn place(&mut self, row: usize, col: usize, num: u8) {
        let bit = 1u16 << num;
        self.rows[row] |= bit;
        self.cols[col] |= bit;
        let box_idx = (row / 3) * 3 + (col / 3);
        self.boxes[box_idx] |= bit;
    }

    /// Removes a number from the given position.
    #[inline(always)]
    fn remove(&mut self, row: usize, col: usize, num: u8) {
        let bit = 1u16 << num;
        self.rows[row] &= !bit;
        self.cols[col] &= !bit;
        let box_idx = (row / 3) * 3 + (col / 3);
        self.boxes[box_idx] &= !bit;
    }

    /// Gets possible values for a cell using bitsets.
    #[inline(always)]
    fn get_possible_values(&self, row: usize, col: usize) -> u16 {
        let box_idx = (row / 3) * 3 + (col / 3);
        let used = self.rows[row] | self.cols[col] | self.boxes[box_idx];
        // All bits 1-9 minus used bits
        0b1111111110 & !used
    }

    /// Counts the number of possible values for a cell.
    #[inline(always)]
    fn count_possible(&self, row: usize, col: usize) -> u32 {
        self.get_possible_values(row, col).count_ones()
    }
    
    /// Extracts the single value from a bitset (assumes exactly one bit is set).
    #[inline(always)]
    fn extract_single_value(bitset: u16) -> u8 {
        bitset.trailing_zeros() as u8
    }
}

/// Ultra-optimized Sudoku solver with naked singles, hidden singles, and efficient MRV.
fn solve_sudoku_ultra(grid: &mut [u8; 81]) -> bool {
    let mut state = ConstraintState::from_grid(grid);
    
    // Apply naked singles and hidden singles before backtracking
    if !apply_constraint_propagation(grid, &mut state) {
        return false;
    }
    
    solve_with_optimizations(grid, &mut state)
}

/// Applies naked singles (cells with one possibility) and hidden singles
/// (numbers that can only go in one place in a unit).
#[inline]
fn apply_constraint_propagation(grid: &mut [u8; 81], state: &mut ConstraintState) -> bool {
    let mut progress = true;
    
    while progress {
        progress = false;
        
        // Naked singles: fill cells with only one possibility
        // Collect the updates first to avoid borrowing conflicts
        let mut updates = Vec::new();
        for (idx, &cell) in grid.iter().enumerate().take(81) {
            if cell == 0 {
                let (row, col) = (idx / 9, idx % 9);
                let possible = state.get_possible_values(row, col);
                
                if possible == 0 {
                    return false; // No valid values
                }
                
                if possible.count_ones() == 1 {
                    let num = ConstraintState::extract_single_value(possible);
                    updates.push((idx, row, col, num));
                }
            }
        }
        
        // Apply the updates
        for (idx, row, col, num) in updates {
            grid[idx] = num;
            state.place(row, col, num);
            progress = true;
        }
        
        // Hidden singles: numbers that can only go in one place
        if apply_hidden_singles(grid, state) {
            progress = true;
        }
    }
    
    true
}

/// Finds and fills hidden singles (numbers that can only go in one cell within a unit).
#[inline]
fn apply_hidden_singles(grid: &mut [u8; 81], state: &mut ConstraintState) -> bool {
    let mut progress = false;
    
    // Check rows
    for row in 0..9 {
        for num in 1..=9u8 {
            let bit = 1u16 << num;
            if (state.rows[row] & bit) != 0 {
                continue; // Already placed
            }
            
            let mut possible_positions = Vec::with_capacity(9);
            for col in 0..9 {
                let idx = row * 9 + col;
                if grid[idx] == 0 {
                    let possible = state.get_possible_values(row, col);
                    if (possible & bit) != 0 {
                        possible_positions.push((row, col, idx));
                    }
                }
            }
            
            if possible_positions.len() == 1 {
                let (r, c, idx) = possible_positions[0];
                grid[idx] = num;
                state.place(r, c, num);
                progress = true;
            }
        }
    }
    
    // Check columns
    for col in 0..9 {
        for num in 1..=9u8 {
            let bit = 1u16 << num;
            if (state.cols[col] & bit) != 0 {
                continue;
            }
            
            let mut possible_positions = Vec::with_capacity(9);
            for row in 0..9 {
                let idx = row * 9 + col;
                if grid[idx] == 0 {
                    let possible = state.get_possible_values(row, col);
                    if (possible & bit) != 0 {
                        possible_positions.push((row, col, idx));
                    }
                }
            }
            
            if possible_positions.len() == 1 {
                let (r, c, idx) = possible_positions[0];
                grid[idx] = num;
                state.place(r, c, num);
                progress = true;
            }
        }
    }
    
    // Check boxes
    for box_idx in 0..9 {
        for num in 1..=9u8 {
            let bit = 1u16 << num;
            if (state.boxes[box_idx] & bit) != 0 {
                continue;
            }
            
            let box_row = (box_idx / 3) * 3;
            let box_col = (box_idx % 3) * 3;
            let mut possible_positions = Vec::with_capacity(9);
            
            for i in 0..3 {
                for j in 0..3 {
                    let row = box_row + i;
                    let col = box_col + j;
                    let idx = row * 9 + col;
                    if grid[idx] == 0 {
                        let possible = state.get_possible_values(row, col);
                        if (possible & bit) != 0 {
                            possible_positions.push((row, col, idx));
                        }
                    }
                }
            }
            
            if possible_positions.len() == 1 {
                let (r, c, idx) = possible_positions[0];
                grid[idx] = num;
                state.place(r, c, num);
                progress = true;
            }
        }
    }
    
    progress
}

/// Recursive solver with optimized MRV cell selection.
fn solve_with_optimizations(grid: &mut [u8; 81], state: &mut ConstraintState) -> bool {
    // Find the best cell using MRV heuristic with early termination
    let mut best_cell: Option<(usize, usize, usize)> = None;
    let mut min_choices = 10;

    for (idx, &cell) in grid.iter().enumerate().take(81) {
        if cell == 0 {
            let (row, col) = (idx / 9, idx % 9);
            let choices = state.count_possible(row, col);
            
            if choices == 0 {
                return false; // No valid values, backtrack immediately
            }
            
            if choices < min_choices {
                min_choices = choices;
                best_cell = Some((idx, row, col));
                
                if min_choices == 1 {
                    // Can't do better than 1 choice
                    break;
                }
            }
        }
    }

    // If no empty cell found, puzzle is solved
    let (idx, row, col) = match best_cell {
        Some(cell) => cell,
        None => return true,
    };

    // Try each possible value
    let possible = state.get_possible_values(row, col);
    
    for num in 1..=9u8 {
        let bit = 1u16 << num;
        if (possible & bit) != 0 {
            // Place the number
            grid[idx] = num;
            state.place(row, col, num);

            // Recursively solve
            if solve_with_optimizations(grid, state) {
                return true;
            }

            // Backtrack
            grid[idx] = 0;
            state.remove(row, col, num);
        }
    }

    false
}


/// Legacy validation function (kept for compatibility in generator).
fn is_valid(grid: &[Vec<u8>], row: usize, col: usize, num: u8) -> bool {
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
    
    #[test]
    fn test_hard_puzzle() {
        // One of the hardest known sudoku puzzles
        let mut puzzle = Puzzle::new(vec![
            vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 3, 6, 0, 0, 0, 0, 0],
            vec![0, 7, 0, 0, 9, 0, 2, 0, 0],
            vec![0, 5, 0, 0, 0, 7, 0, 0, 0],
            vec![0, 0, 0, 0, 4, 5, 7, 0, 0],
            vec![0, 0, 0, 1, 0, 0, 0, 3, 0],
            vec![0, 0, 1, 0, 0, 0, 0, 6, 8],
            vec![0, 0, 8, 5, 0, 0, 0, 1, 0],
            vec![0, 9, 0, 0, 0, 0, 4, 0, 0],
        ]);

        assert!(solve(&mut puzzle));
    }
}
