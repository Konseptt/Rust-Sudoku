use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::time::Instant;

use crate::puzzle::Puzzle;
use crate::solver::{generate_sudoku, solve};

/// Request structure for solving a Sudoku puzzle
#[derive(Debug, Deserialize)]
pub struct SolveRequest {
    pub grid: Vec<Vec<u8>>,
}

/// Response structure for solve endpoint
#[derive(Debug, Serialize)]
pub struct SolveResponse {
    pub success: bool,
    pub solved_grid: Option<Vec<Vec<u8>>>,
    pub solve_time_micros: u128,
    pub solve_time_ms: f64,
    pub error: Option<String>,
}

/// Response structure for generate endpoint
#[derive(Debug, Serialize)]
pub struct GenerateResponse {
    pub grid: Vec<Vec<u8>>,
}

/// POST /api/solve - Solve a Sudoku puzzle
/// 
/// Accepts a 9x9 grid and returns the solved grid with timing information
pub async fn solve_puzzle(req: web::Json<SolveRequest>) -> impl Responder {
    // Validate grid dimensions
    if req.grid.len() != 9 || req.grid.iter().any(|row| row.len() != 9) {
        return HttpResponse::BadRequest().json(SolveResponse {
            success: false,
            solved_grid: None,
            solve_time_micros: 0,
            solve_time_ms: 0.0,
            error: Some("Grid must be 9x9".to_string()),
        });
    }

    // Validate cell values (0-9 only)
    for row in &req.grid {
        for &cell in row {
            if cell > 9 {
                return HttpResponse::BadRequest().json(SolveResponse {
                    success: false,
                    solved_grid: None,
                    solve_time_micros: 0,
                    solve_time_ms: 0.0,
                    error: Some("Cell values must be 0-9".to_string()),
                });
            }
        }
    }

    let mut puzzle = Puzzle::new(req.grid.clone());
    
    // Time the solve operation
    let start = Instant::now();
    let success = solve(&mut puzzle);
    let duration = start.elapsed();
    
    let solve_time_micros = duration.as_micros();
    let solve_time_ms = duration.as_secs_f64() * 1000.0;

    if success {
        HttpResponse::Ok().json(SolveResponse {
            success: true,
            solved_grid: Some(puzzle.grid().clone()),
            solve_time_micros,
            solve_time_ms,
            error: None,
        })
    } else {
        HttpResponse::Ok().json(SolveResponse {
            success: false,
            solved_grid: None,
            solve_time_micros,
            solve_time_ms,
            error: Some("Unable to solve puzzle - no valid solution exists".to_string()),
        })
    }
}

/// GET /api/generate - Generate a random Sudoku puzzle
/// 
/// Returns a randomly generated puzzle with approximately 40 empty cells
pub async fn generate_puzzle() -> impl Responder {
    let puzzle = generate_sudoku();
    
    HttpResponse::Ok().json(GenerateResponse {
        grid: puzzle.grid().clone(),
    })
}

/// GET /api/health - Health check endpoint
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "sudoku-solver"
    }))
}
