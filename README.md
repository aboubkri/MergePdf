# 📄 MergePDF (Rust-Powered)

A simple, transparent, and 100% local tool for merging PDF files. Instead of uploading your sensitive documents to random websites, this tool lets you process them on your own machine using a modern tech stack.

## ✨ Features
* **Drag & Drop:** Just drop your PDFs into the window.
* **High Performance:** Powered by Rust (no file size limits).
* **Cross-Platform:** Works on Windows, macOS, and Linux.
* **Transparent:** Open-source code you can inspect and run locally.
    
## 🛠️ Prerequisites

You must have **Python** and **Rust** installed on your system before running the scripts.

### 1. Python (3.10+)
- **Windows:** [Download from Python.org](https://www.python.org/downloads/windows/).  
  ⚠️ **Important:** Check **"Add Python to PATH"** during installation.
- **macOS:** `brew install python`
- **Linux:** `sudo apt install python3 python3-venv` (Ubuntu/Debian)

### 2. Rust Toolchain (Cargo)
- **All Platforms:** Install via [rustup.rs](https://rustup.rs/) by following the on-screen instructions.

## 🚀 How to Run

### 🪟 Windows
Double-click `run.bat`. The script handles environment setup and compilation automatically.

### 🍎 macOS / 🐧 Linux
1. Open your terminal in the project folder.
2. Make the script executable: `chmod +x run.sh`
3. Run it: `./run.sh`        

## 🏗️ Project Architecture
The app is built as a Hybrid Rust/Python Library:

- Core Engine (src/pdf/): Pure Rust logic for PDF manipulation using lopdf.

- The Bridge (src/lib.rs): Uses PyO3 to expose high-performance Rust functions to Python.

- Frontend (src/ui.py): A Python-based GUI using TkinterDnD for drag-and-drop support.