# Sudoku Solver Web Application

A high-performance web-based Sudoku solver powered by Rust backend with an ultra-optimized solving algorithm.

## 🚀 Features

- **Manual Entry**: Click any cell and enter numbers 1-9
- **Random Generation**: Generate random Sudoku puzzles with varying difficulty
- **Ultra-Fast Solving**: Solve puzzles in microseconds with performance timing display
- **Premium UI**: Modern dark theme with glassmorphism and smooth animations
- **Keyboard Navigation**: Use arrow keys to navigate between cells

## 🎯 Algorithm Features

The solver uses several advanced techniques for optimal performance:

- **Bitset-based Constraint Tracking**: O(1) validation using bit manipulation
- **Constraint Propagation**: Naked and hidden singles detection
- **MRV Heuristic**: Minimum Remaining Values for smart cell selection
- **Optimized Backtracking**: Efficient recursive solving with early termination

## 🛠️ Setup & Usage

### 1. Build the Rust Backend

First, make sure you have Rust installed. Then build the project:

```powershell
cargo build --release
```

### 2. Start the Server

Run the Rust web server:

```powershell
cargo run --bin server
```

The server will start at `http://localhost:8080` and you should see:
```
🚀 Starting Sudoku Solver Server...
📍 Server will be available at: http://localhost:8080
📚 API Endpoints:
   - POST http://localhost:8080/api/solve
   - GET  http://localhost:8080/api/generate
   - GET  http://localhost:8080/api/health
```

### 3. Open the Web Interface

Simply open `web/index.html` in your web browser. The interface will automatically connect to the Rust backend server.

## 📡 API Endpoints

### POST /api/solve
Solve a Sudoku puzzle.

**Request:**
```json
{
  "grid": [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    ...
  ]
}
```

**Response:**
```json
{
  "success": true,
  "solved_grid": [[5, 3, 4, 6, 7, 8, 9, 1, 2], ...],
  "solve_time_micros": 123,
  "solve_time_ms": 0.123,
  "error": null
}
```

### GET /api/generate
Generate a random Sudoku puzzle.

**Response:**
```json
{
  "grid": [[5, 3, 0, 0, 7, 0, 0, 0, 0], ...]
}
```

### GET /api/health
Check server status.

**Response:**
```json
{
  "status": "ok",
  "service": "sudoku-solver"
}
```

## 🎮 How to Use

1. **Manual Entry**: Click on any cell and type a number (1-9)
2. **Generate Puzzle**: Click the "Generate Puzzle" button to get a random puzzle
3. **Solve**: Click "Solve Puzzle" to see the solution with timing information
4. **Clear**: Click "Clear Grid" to reset everything
5. **Keyboard Navigation**: Use arrow keys to move between cells

## 🔧 Technology Stack

- **Backend**: Rust with Actix-web framework
- **Frontend**: Vanilla HTML/CSS/JavaScript
- **Communication**: REST API with JSON
- **Styling**: Modern CSS with glassmorphism and animations

## 📊 Performance

Typical solve times on modern hardware:
- Easy puzzles: < 1ms (often in microseconds)
- Medium puzzles: 1-5ms
- Hard puzzles: 5-15ms
- Extreme puzzles: 10-50ms

The exact time depends on the number of empty cells and the puzzle complexity.

## 📝 Notes

- Make sure the Rust server is running before using the web interface
- The frontend uses `localhost:8080` by default - update `API_BASE_URL` in `app.js` if needed
- CORS is enabled for development - configure appropriately for production

## 🦀 Powered by Rust

Built with ❤️ using Rust's performance and safety guarantees.
