#!/bin/bash
set -e

echo "--- 📄 Starting PDF-Merger Auto-Setup ---"

# PLATFORM DETECTION & SYSTEM DEPENDENCIES
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "OS: Linux detected. Checking distribution..."
    
    # Check for specific package managers
    if command -v apt-get &> /dev/null; then
        echo "Pkg Manager: apt (Debian/Ubuntu)"
        sudo apt-get update
        sudo apt-get install -y python3 python3-venv python3-tk build-essential curl

    elif command -v dnf &> /dev/null; then
        echo "Pkg Manager: dnf (Fedora)"
        sudo dnf install -y python3 python3-tkinter python3-devel gcc gcc-c++ make curl

    elif command -v pacman &> /dev/null; then
        echo "Pkg Manager: pacman (Arch)"
        sudo pacman -S --noconfirm python tk base-devel curl

    else
        echo "⚠️ Unknown distribution. Ensure python3, tk-inter, and a C-compiler are installed."
    fi

elif [[ "$OSTYPE" == "darwin"* ]]; then
    echo "OS: macOS detected."
    # Install Homebrew if missing
    if ! command -v brew &> /dev/null; then
        /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
    fi
    brew install python python-tk
fi

# RUST INSTALLATION (Universal)
if ! command -v cargo &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

# PYTHON VIRTUAL ENVIRONMENT
echo "Configuring Python environment..."
if [ ! -d ".venv" ]; then
    python3 -m venv .venv --without-pip || python3 -m venv .venv
fi

VENV_PYTHON="./.venv/bin/python3"

# Check if Pip actually exists in the venv
if ! $VENV_PYTHON -m pip --version &> /dev/null; then
    echo "Pip missing in venv. Bootstrapping Pip..."
    curl -sS https://bootstrap.pypa.io/get-pip.py | $VENV_PYTHON
fi

$VENV_PYTHON -m pip install --upgrade pip maturin

# Compile the Rust Core
echo "Building Rust engine (this may take a minute)..."
# We use 'maturin' via the venv's bin
./.venv/bin/maturin develop --release

# Install UI requirements
$VENV_PYTHON -m pip install -r requirements.txt

# LAUNCH
echo "--- 🚀 Launching PDF Merger ---"
$VENV_PYTHON src/ui.py