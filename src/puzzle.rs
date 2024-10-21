/// Represents a Sudoku puzzle.
pub struct Puzzle {
    grid: Vec<Vec<u8>>,
}

impl Puzzle {
    /// Creates a new `Puzzle` with the given grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D vector representing the Sudoku grid.
    ///
    /// # Returns
    ///
    /// A new `Puzzle` instance.
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        Self { grid }
    }

    /// Prints the Sudoku puzzle to the console.
    pub fn print(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    /// Returns a mutable reference to the grid.
    ///
    /// # Returns
    ///
    /// A mutable reference to the 2D vector representing the Sudoku grid.
    pub fn grid_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.grid
    }
}
