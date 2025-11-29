use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use crate::api::{generate_puzzle, health_check, solve_puzzle};

/// Configure the web application routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/solve", web::post().to(solve_puzzle))
            .route("/generate", web::get().to(generate_puzzle))
            .route("/health", web::get().to(health_check)),
    );
}

/// Start the Actix-web server
pub async fn start_server() -> std::io::Result<()> {
    println!("🚀 Starting Sudoku Solver Server...");
    println!("📍 Server will be available at: http://localhost:8080");
    println!("📚 API Endpoints:");
    println!("   - POST http://localhost:8080/api/solve");
    println!("   - GET  http://localhost:8080/api/generate");
    println!("   - GET  http://localhost:8080/api/health");
    println!("\n💡 Open web/index.html in your browser to use the web interface");
    println!("Press Ctrl+C to stop the server\n");

    HttpServer::new(|| {
        // Configure CORS to allow browser access
        let cors = Cors::permissive(); // Allow all origins for development
        
        App::new()
            .wrap(cors)
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
