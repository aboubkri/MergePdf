#!/bin/bash

echo "--- PDF-Merger: Unix Setup ---"

# 1. Check Python
if ! command -v python3 &> /dev/null; then
    echo "[ERROR] Python3 not found. Please install it via your package manager."
    exit 1
fi

# 2. Check Rust
if ! command -v cargo &> /dev/null; then
    echo "[ERROR] Rust (cargo) not found. Please install it via https://rustup.rs/"
    exit 1
fi

# 3. Setup Virtual Environment
if [ ! -d ".venv" ]; then
    echo "[SETUP] Creating virtual environment..."
    python3 -m venv .venv
fi

# 4. Build and Run
echo "[BUILD] Installing dependencies and compiling Rust engine..."
source .venv/bin/activate
python3 -m pip install --upgrade pip maturin
maturin develop --release
python3 -m pip install -r requirements.txt

echo "--- 🚀 Launching Native UI ---"
python3 src/ui.py