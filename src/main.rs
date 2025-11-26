use suko_solver::puzzle::Puzzle;
use suko_solver::solver::solve;
use std::time::Instant;

fn main() {
    println!("=== Sudoku Solver Performance Benchmark ===\n");
    
    // Test 1: Easy puzzle
    let easy_puzzle = vec![
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
    
    benchmark_puzzle("Easy Puzzle", &easy_puzzle);
    
    // Test 2: Hard puzzle (one of the hardest known)
    let hard_puzzle = vec![
        vec![8, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 3, 6, 0, 0, 0, 0, 0],
        vec![0, 7, 0, 0, 9, 0, 2, 0, 0],
        vec![0, 5, 0, 0, 0, 7, 0, 0, 0],
        vec![0, 0, 0, 0, 4, 5, 7, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 3, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 6, 8],
        vec![0, 0, 8, 5, 0, 0, 0, 1, 0],
        vec![0, 9, 0, 0, 0, 0, 4, 0, 0],
    ];
    
    benchmark_puzzle("Hard Puzzle (AI Escargot)", &hard_puzzle);
    
    // Test 3: Medium difficulty
    let medium_puzzle = vec![
        vec![0, 0, 0, 2, 6, 0, 7, 0, 1],
        vec![6, 8, 0, 0, 7, 0, 0, 9, 0],
        vec![1, 9, 0, 0, 0, 4, 5, 0, 0],
        vec![8, 2, 0, 1, 0, 0, 0, 4, 0],
        vec![0, 0, 4, 6, 0, 2, 9, 0, 0],
        vec![0, 5, 0, 0, 0, 3, 0, 2, 8],
        vec![0, 0, 9, 3, 0, 0, 0, 7, 4],
        vec![0, 4, 0, 0, 5, 0, 0, 3, 6],
        vec![7, 0, 3, 0, 1, 8, 0, 0, 0],
    ];
    
    benchmark_puzzle("Medium Puzzle", &medium_puzzle);
    
    println!("\n=== Multiple Iterations Test ===");
    let iterations = 100;
    let mut total_time = 0u128;
    
    for _ in 0..iterations {
        let mut puzzle = Puzzle::new(easy_puzzle.clone());
        let start = Instant::now();
        solve(&mut puzzle);
        total_time += start.elapsed().as_micros();
    }
    
    println!("{} iterations of easy puzzle:", iterations);
    println!("  Average time: {:.2}μs", total_time as f64 / iterations as f64);
    println!("  Total time: {:.2}ms", total_time as f64 / 1000.0);
    println!("  Throughput: {:.0} solves/second", iterations as f64 / (total_time as f64 / 1_000_000.0));
}

fn benchmark_puzzle(name: &str, grid: &Vec<Vec<u8>>) {
    println!("Testing: {}", name);
    println!("\nBefore:");
    print_grid_pretty(grid);
    
    let mut puzzle = Puzzle::new(grid.clone());
    let start = Instant::now();
    let solved = solve(&mut puzzle);
    let duration = start.elapsed();
    
    if solved {
        println!("\n  ✓ Solved in {:.2}μs ({:.4}ms)", 
                 duration.as_micros(), 
                 duration.as_secs_f64() * 1000.0);
        println!("\nAfter:");
        print_grid_pretty(puzzle.grid());
    } else {
        println!("  ✗ Failed to solve");
    }
    
    println!();
}

fn print_grid_pretty(grid: &Vec<Vec<u8>>) {
    for (i, row) in grid.iter().enumerate() {
        if i % 3 == 0 && i != 0 {
            println!("  ------+-------+------");
        }
        print!("  ");
        for (j, &cell) in row.iter().enumerate() {
            if j % 3 == 0 && j != 0 {
                print!("| ");
            }
            if cell == 0 {
                print!(". ");
            } else {
                print!("{} ", cell);
            }
        }
        println!();
    }
}
