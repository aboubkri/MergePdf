use mergepdf::pdf::merge::merge_multiple_pdfs;

fn main() {
    let pdfs = [
        "assets/samples/pdf_text_1.pdf",
        "assets/samples/pdf_text_image_1.pdf",
        "assets/samples/pdf_text_table_1.pdf",
        "assets/samples/pdf_text_chart_1.pdf",
    ];

    let output = "output/merged_book.pdf";

    match merge_multiple_pdfs(&pdfs, output) {
        Ok(_) => println!("Merged into {}", output),
        Err(e) => eprintln!("Merge failed: {:?}", e),
    }
}
