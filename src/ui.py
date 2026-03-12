import tkinter as tk
from tkinter import filedialog, messagebox
from tkinterdnd2 import DND_FILES, TkinterDnD
import mergepdf
import os

class PDFMergerApp:
    def __init__(self, root):
        self.root = root
        self.root.title("MergePDF | Rust Engine")
        self.root.geometry("500x400")
        self.files = []

        # 1. Drag and Drop Zone
        self.drop_label = tk.Label(
            root, 
            text="\n\nDrop PDF files here\nor click to browse\n\n",
            relief="groove",  # Changed from "dashed"
            bd=2,
            bg="#f0f0f0",
            fg="#555",
            pady=20
        )
        self.drop_label.pack(fill="both", expand=True, padx=20, pady=20)

        # Register the label as a drop target
        self.drop_label.drop_target_register(DND_FILES)
        self.drop_label.dnd_bind('<<Drop>>', self.handle_drop)
        self.drop_label.bind("<Button-1>", lambda e: self.pick_files())

        # 2. File List View
        self.listbox = tk.Listbox(root, height=6)
        self.listbox.pack(fill="x", padx=20)

        # 3. Action Buttons
        btn_frame = tk.Frame(root)
        btn_frame.pack(pady=10)

        tk.Button(btn_frame, text="Clear", command=self.clear_files).pack(side="left", padx=5)
        self.merge_btn = tk.Button(
            btn_frame, text="Merge & Save", 
            command=self.do_merge, 
            bg="#2ecc71", fg="white", font=("Arial", 10, "bold")
        )
        self.merge_btn.pack(side="left", padx=5)

    def handle_drop(self, event):
        # Linux often passes file paths inside braces or with specific formatting
        # This helper cleans up the string into a list of paths
        files = self.root.tk.splitlist(event.data)
        pdf_files = [f for f in files if f.lower().endswith('.pdf')]
        
        if pdf_files:
            self.add_files_to_list(pdf_files)
        else:
            messagebox.showwarning("Invalid File", "Please drop PDF files only.")

    def pick_files(self):
        new_files = filedialog.askopenfilenames(filetypes=[("PDF files", "*.pdf")])
        if new_files:
            self.add_files_to_list(new_files)

    def add_files_to_list(self, new_files):
        for f in new_files:
            if f not in self.files:
                self.files.append(f)
                self.listbox.insert(tk.END, os.path.basename(f))

    def clear_files(self):
        self.files = []
        self.listbox.delete(0, tk.END)

    def do_merge(self):
        if not self.files:
            messagebox.showwarning("Empty", "Add some PDFs first.")
            return

        output = filedialog.asksaveasfilename(defaultextension=".pdf")
        if output:
            try:
                # Call Rust Backend
                mergepdf.run_merge(list(self.files), output)
                messagebox.showinfo("Success", "Done!")
                self.clear_files()
            except Exception as e:
                messagebox.showerror("Error", str(e))

if __name__ == "__main__":
    # Use TkinterDnD.Tk instead of tk.Tk
    root = TkinterDnD.Tk()
    app = PDFMergerApp(root)
    root.mainloop()