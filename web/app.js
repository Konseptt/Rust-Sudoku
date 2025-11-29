// ===== CONFIGURATION =====
const API_BASE_URL = 'http://localhost:8080/api';

// ===== STATE =====
let currentGrid = Array(9).fill(null).map(() => Array(9).fill(0));
let isGeneratedPuzzle = Array(9).fill(null).map(() => Array(9).fill(false));

// ===== DOM ELEMENTS =====
const gridElement = document.getElementById('sudoku-grid');
const clearBtn = document.getElementById('clear-btn');
const generateBtn = document.getElementById('generate-btn');
const solveBtn = document.getElementById('solve-btn');
const statusMessage = document.getElementById('status-message');
const timingDisplay = document.getElementById('timing-display');
const loadingOverlay = document.getElementById('loading-overlay');

// ===== INITIALIZATION =====
function init() {
    createGrid();
    attachEventListeners();
    checkServerHealth();
}

function createGrid() {
    for (let i = 0; i < 81; i++) {
        const cell = document.createElement('input');
        cell.type = 'text';
        cell.maxLength = 1;
        cell.className = 'sudoku-cell';
        cell.dataset.index = i;
        
        cell.addEventListener('input', handleCellInput);
        cell.addEventListener('keydown', handleCellNavigation);
        
        gridElement.appendChild(cell);
    }
}

function attachEventListeners() {
    clearBtn.addEventListener('click', clearGrid);
    generateBtn.addEventListener('click', generatePuzzle);
    solveBtn.addEventListener('click', solvePuzzle);
}

// ===== GRID MANAGEMENT =====
function handleCellInput(e) {
    const cell = e.target;
    const value = cell.value;
    
    // Only allow numbers 1-9
    if (value !== '' && (!/^[1-9]$/.test(value))) {
        cell.value = '';
        return;
    }
    
    const index = parseInt(cell.dataset.index);
    const row = Math.floor(index / 9);
    const col = index % 9;
    
    currentGrid[row][col] = value === '' ? 0 : parseInt(value);
    
    // Clear any error state
    cell.classList.remove('error');
    cell.classList.remove('solved');
}

function handleCellNavigation(e) {
    const cell = e.target;
    const index = parseInt(cell.dataset.index);
    let newIndex = index;
    
    switch(e.key) {
        case 'ArrowUp':
            newIndex = index - 9;
            e.preventDefault();
            break;
        case 'ArrowDown':
            newIndex = index + 9;
            e.preventDefault();
            break;
        case 'ArrowLeft':
            newIndex = index - 1;
            e.preventDefault();
            break;
        case 'ArrowRight':
            newIndex = index + 1;
            e.preventDefault();
            break;
        case 'Backspace':
        case 'Delete':
            cell.value = '';
            currentGrid[Math.floor(index / 9)][index % 9] = 0;
            break;
        default:
            return;
    }
    
    if (newIndex >= 0 && newIndex < 81) {
        const cells = document.querySelectorAll('.sudoku-cell');
        cells[newIndex].focus();
    }
}

function clearGrid() {
    const cells = document.querySelectorAll('.sudoku-cell');
    cells.forEach(cell => {
        cell.value = '';
        cell.removeAttribute('readonly');
        cell.classList.remove('solved', 'error');
    });
    
    currentGrid = Array(9).fill(null).map(() => Array(9).fill(0));
    isGeneratedPuzzle = Array(9).fill(null).map(() => Array(9).fill(false));
    
    updateStatus('Grid cleared', false);
    timingDisplay.textContent = '';
}

function setGridFromArray(grid, markAsReadonly = false) {
    const cells = document.querySelectorAll('.sudoku-cell');
    
    grid.forEach((row, i) => {
        row.forEach((value, j) => {
            const index = i * 9 + j;
            const cell = cells[index];
            
            if (value !== 0) {
                cell.value = value;
                if (markAsReadonly) {
                    cell.setAttribute('readonly', 'true');
                    isGeneratedPuzzle[i][j] = true;
                }
            } else {
                cell.value = '';
                if (markAsReadonly) {
                    cell.removeAttribute('readonly');
                    isGeneratedPuzzle[i][j] = false;
                }
            }
            
            currentGrid[i][j] = value;
        });
    });
}

function setSolvedGrid(grid, animated = true) {
    const cells = document.querySelectorAll('.sudoku-cell');
    
    grid.forEach((row, i) => {
        row.forEach((value, j) => {
            const index = i * 9 + j;
            const cell = cells[index];
            
            if (value !== 0 && currentGrid[i][j] === 0) {
                if (animated) {
                    setTimeout(() => {
                        cell.value = value;
                        cell.classList.add('solved');
                    }, index * 10); // Stagger animation
                } else {
                    cell.value = value;
                    cell.classList.add('solved');
                }
            }
            
            currentGrid[i][j] = value;
        });
    });
}

// ===== API CALLS =====
async function checkServerHealth() {
    try {
        const response = await fetch(`${API_BASE_URL}/health`);
        if (response.ok) {
            console.log('✅ Server is running');
        }
    } catch (error) {
        updateStatus('⚠️ Server not running. Please start the Rust server first.', true);
        console.error('Server health check failed:', error);
    }
}

async function generatePuzzle() {
    showLoading(true);
    updateStatus('Generating puzzle...', false);
    
    try {
        const response = await fetch(`${API_BASE_URL}/generate`);
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        
        // Clear previous grid
        clearGrid();
        
        // Set the generated puzzle
        setGridFromArray(data.grid, true);
        
        updateStatus('✅ Puzzle generated successfully!', false);
        timingDisplay.textContent = '';
    } catch (error) {
        updateStatus('❌ Failed to generate puzzle. Is the server running?', true);
        console.error('Generate error:', error);
    } finally {
        showLoading(false);
    }
}

async function solvePuzzle() {
    // Check if grid is empty
    const isEmpty = currentGrid.every(row => row.every(cell => cell === 0));
    if (isEmpty) {
        updateStatus('❌ Please enter a puzzle or generate one first', true);
        return;
    }
    
    showLoading(true);
    updateStatus('Solving puzzle...', false);
    timingDisplay.textContent = '';
    
    try {
        const response = await fetch(`${API_BASE_URL}/solve`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                grid: currentGrid
            })
        });
        
        if (!response.ok) {
            throw new Error(`HTTP error! status: ${response.status}`);
        }
        
        const data = await response.json();
        
        if (data.success) {
            setSolvedGrid(data.solved_grid, true);
            
            updateStatus('✅ Puzzle solved successfully!', false);
            
            // Display timing
            if (data.solve_time_micros < 1000) {
                timingDisplay.textContent = `⚡ ${data.solve_time_micros}μs`;
            } else {
                timingDisplay.textContent = `⚡ ${data.solve_time_ms.toFixed(3)}ms`;
            }
        } else {
            updateStatus(`❌ ${data.error || 'Unable to solve puzzle'}`, true);
        }
    } catch (error) {
        updateStatus('❌ Failed to solve puzzle. Is the server running?', true);
        console.error('Solve error:', error);
    } finally {
        showLoading(false);
    }
}

// ===== UI HELPERS =====
function updateStatus(message, isError = false) {
    statusMessage.textContent = message;
    statusMessage.className = 'status-message';
    
    if (isError) {
        statusMessage.classList.add('error');
    } else if (message.includes('✅')) {
        statusMessage.classList.add('success');
    }
}

function showLoading(show) {
    if (show) {
        loadingOverlay.classList.remove('hidden');
        clearBtn.disabled = true;
        generateBtn.disabled = true;
        solveBtn.disabled = true;
    } else {
        loadingOverlay.classList.add('hidden');
        clearBtn.disabled = false;
        generateBtn.disabled = false;
        solveBtn.disabled = false;
    }
}

// ===== START APPLICATION =====
document.addEventListener('DOMContentLoaded', init);
