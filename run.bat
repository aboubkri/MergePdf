echo --- PDF-Merger: Windows Setup ---

:: 1. Check Python
python --version >nul 2>&1
if errorlevel 1 goto :no_python

:: 2. Check Rust
cargo --version >nul 2>&1
if errorlevel 1 goto :no_rust

:: 3. Setup Virtual Environment
if not exist .venv (
    echo [SETUP] Creating virtual environment...
    python -m venv .venv
)

:: 4. Build and Run
echo [BUILD] Installing dependencies...
call .venv\Scripts\activate.bat

python -m pip install --upgrade pip maturin

echo [BUILD] Compiling Rust engine...
:: FIX: Suppress version check for Python 3.14/experimental versions
set PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
maturin develop --release

echo [BUILD] Installing UI dependencies...
python -m pip install -r requirements.txt

echo --- Launching Application ---
python src/ui.py
pause
goto :eof

:no_python
echo [ERROR] Python not found.
echo Please install Python 3.12 from python.org and check "Add to PATH".
pause
exit

:no_rust
echo [ERROR] Rust (cargo) not found.
echo Please install it from https://rustup.rs/
pause
exit