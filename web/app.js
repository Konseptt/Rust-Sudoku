// ===== CONFIGURATION =====
// Auto-detect if running on GitHub Pages or locally
const isGitHubPages = window.location.hostname.includes('github.io');
const API_BASE_URL = isGitHubPages
    ? 'https://rust-sudoku.onrender.com'  // Update with your backend URL
    : 'http://localhost:8080/api';

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
const mascotDrawing = document.getElementById('mascot-drawing');

// ===== MASCOT EMOTIONS =====
const MASCOT = {
    IDLE: '(•_•)\n<br><span style="font-size: 0.8em;">/||\\</span>',
    THINKING: '(O_O)\n<br><span style="font-size: 0.8em;">/||\\</span>',
    HAPPY: '(^‿^)\n<br><span style="font-size: 0.8em;">\\||/</span>',
    SAD: '(>_<)\n<br><span style="font-size: 0.8em;">/||\\</span>',
    CONFUSED: '(o_O)\n<br><span style="font-size: 0.8em;">/||\\</span>'
};

// ===== INITIALIZATION =====
function init() {
    createGrid();
    attachEventListeners();
    checkServerHealth();
    setMascot(MASCOT.IDLE);
}

function createGrid() {
    gridElement.innerHTML = '';
    for (let i = 0; i < 81; i++) {
        const cell = document.createElement('input');
        cell.type = 'text';
        cell.maxLength = 1;
        cell.className = 'sudoku-cell';
        cell.dataset.index = i;

        cell.addEventListener('input', handleCellInput);
        cell.addEventListener('keydown', handleCellNavigation);
        cell.addEventListener('focus', () => setMascot(MASCOT.IDLE));

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
        setMascot(MASCOT.CONFUSED);
        updateStatus("Numbers only, please!");
        return;
    }

    const index = parseInt(cell.dataset.index);
    const row = Math.floor(index / 9);
    const col = index % 9;

    currentGrid[row][col] = value === '' ? 0 : parseInt(value);

    // Clear any error state
    cell.classList.remove('error');
    cell.classList.remove('solved');

    if (value !== '') {
        playScribbleSound();
    }
}

function handleCellNavigation(e) {
    const cell = e.target;
    const index = parseInt(cell.dataset.index);
    let newIndex = index;

    switch (e.key) {
        case 'ArrowUp': newIndex = index - 9; break;
        case 'ArrowDown': newIndex = index + 9; break;
        case 'ArrowLeft': newIndex = index - 1; break;
        case 'ArrowRight': newIndex = index + 1; break;
        case 'Backspace':
        case 'Delete':
            cell.value = '';
            currentGrid[Math.floor(index / 9)][index % 9] = 0;
            break;
        default: return;
    }

    if (newIndex >= 0 && newIndex < 81) {
        e.preventDefault();
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

    updateStatus('Clean slate!');
    timingDisplay.textContent = '--';
    setMascot(MASCOT.IDLE);
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

function setSolvedGrid(grid) {
    const cells = document.querySelectorAll('.sudoku-cell');

    grid.forEach((row, i) => {
        row.forEach((value, j) => {
            const index = i * 9 + j;
            const cell = cells[index];

            if (value !== 0 && currentGrid[i][j] === 0) {
                setTimeout(() => {
                    cell.value = value;
                    cell.classList.add('solved');
                }, index * 5); // Faster stagger for doodle effect
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
        updateStatus('Server is napping... (Start Rust!)');
        setMascot(MASCOT.SAD);
    }
}

async function generatePuzzle() {
    showLoading(true);
    updateStatus('Drawing a new puzzle...');
    setMascot(MASCOT.THINKING);

    try {
        const response = await fetch(`${API_BASE_URL}/generate`);
        if (!response.ok) throw new Error('Network response was not ok');

        const data = await response.json();
        clearGrid();
        setGridFromArray(data.grid, true);

        updateStatus('Here is a fresh one!');
        setMascot(MASCOT.HAPPY);
        timingDisplay.textContent = '--';
    } catch (error) {
        updateStatus('Oops, I dropped my pencil.');
        setMascot(MASCOT.SAD);
        console.error('Generate error:', error);
    } finally {
        showLoading(false);
    }
}

async function solvePuzzle() {
    const isEmpty = currentGrid.every(row => row.every(cell => cell === 0));
    if (isEmpty) {
        updateStatus('The page is empty!');
        setMascot(MASCOT.CONFUSED);
        return;
    }

    showLoading(true);
    updateStatus('Calculating...');
    setMascot(MASCOT.THINKING);
    timingDisplay.textContent = '--';

    try {
        const response = await fetch(`${API_BASE_URL}/solve`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ grid: currentGrid })
        });

        if (!response.ok) throw new Error('Network response was not ok');

        const data = await response.json();

        if (data.success) {
            setSolvedGrid(data.solved_grid);
            updateStatus('Ta-da! Solved it!');
            setMascot(MASCOT.HAPPY);

            if (data.solve_time_micros < 1000) {
                timingDisplay.textContent = `${data.solve_time_micros}μs`;
            } else {
                timingDisplay.textContent = `${data.solve_time_ms.toFixed(3)}ms`;
            }
        } else {
            updateStatus(data.error || 'I am stumped...');
            setMascot(MASCOT.SAD);
        }
    } catch (error) {
        updateStatus('My brain hurts...');
        setMascot(MASCOT.SAD);
        console.error('Solve error:', error);
    } finally {
        showLoading(false);
    }
}

// ===== UI HELPERS =====
function updateStatus(message) {
    statusMessage.textContent = message;
}

function setMascot(emotion) {
    mascotDrawing.innerHTML = emotion;
}

function showLoading(show) {
    if (show) {
        loadingOverlay.classList.remove('hidden');
    } else {
        loadingOverlay.classList.add('hidden');
    }
}

function playScribbleSound() {
    // Placeholder for sound effect if we wanted to add audio
}

// ===== START APPLICATION =====
document.addEventListener('DOMContentLoaded', init);
