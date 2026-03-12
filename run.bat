@echo off
echo --- Starting PDF-Merger Auto-Setup ---

:: Check/Install Python
where python >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Python via Winget...
    winget install -e --id Python.Python.3
    echo Please restart this script after Python finishes installing.
    pause && exit
)

:: Check/Install Rust
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo Installing Rust (rustup)...
    winget install -e --id Rustlang.Rustup
    echo Please restart this script after Rust finishes installing.
    pause && exit
)

:: Setup and Run
if not exist .venv ( python -m venv .venv )
call .venv\Scripts\activate
pip install maturin
maturin develop --release
pip install -r requirements.txt

python src/ui.py
pause