pub struct Puzzle {
    grid: Vec<Vec<u8>>,
}

impl Puzzle {
    pub fn new(grid: Vec<Vec<u8>>) -> Self {
        Self { grid }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for &cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }

    pub fn grid_mut(&mut self) -> &mut Vec<Vec<u8>> {
        &mut self.grid
    }
}