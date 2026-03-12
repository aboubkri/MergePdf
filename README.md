# 📄 MergePDF (Rust-Powered)

A simple, transparent, and 100% local tool for merging PDF files. Instead of uploading your sensitive documents to random websites, this tool lets you process them on your own machine using a modern tech stack.

## ✨ Features
* **Drag & Drop:** Just drop your PDFs into the window.
* **High Performance:** Powered by Rust (no file size limits).
* **Cross-Platform:** Works on Windows, macOS, and Linux.
* **Transparent:** Open-source code you can inspect and run locally.
    
        
## 🚀 One-Click Setup

### For Windows Users
1. Download this folder.
2. Double-click the `run.bat` file.
3. If it's your first time, it will install the necessary tools. You may need to restart the script once after the installation finishes.

### For Linux & macOS Users
1. Open your terminal in this folder.
2. Run the following command:
   ```bash
   chmod +x run.sh && ./run.sh


## 🏗️ Project Architecture
The app is built as a Hybrid Rust/Python Library:

- Core Engine (src/pdf/): Pure Rust logic for PDF manipulation using lopdf.

- The Bridge (src/lib.rs): Uses PyO3 to expose high-performance Rust functions to Python.

- Frontend (src/ui.py): A Python-based GUI using TkinterDnD for drag-and-drop support.

## 🔍 Reproducible Testing via Docker
To ensure the automated setup scripts are robust, a Dockerfile.test is provided. This creates a volatile, "headless" instance of Ubuntu 22.04 LTS to simulate a clean-slate environment with no pre-installed development tools.

To execute the Ubuntu-based integration test::

Ensure Docker is installed and running.

Run:

```Bash
docker build -t pdf-merger-test -f Dockerfile.test .
docker run -it pdf-merger-test
```


Note: The containerized test is designed to validate the installation and build pipeline. Because Docker containers are headless by default, the script will exit with a _tkinter.TclError after a successful build. This confirms the application reached the final execution stage.