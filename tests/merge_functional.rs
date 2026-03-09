use std::fs;
use mergepdf::pdf::merge::merge_multiple_pdfs;
use lopdf::Document;

#[test]
fn test_pdf_merge_metadata_and_size() {
    // Setup paths
    let inputs = [
        "assets/samples/pdf_text_1.pdf",
        "assets/samples/pdf_text_image_1.pdf",
        "assets/samples/pdf_text_table_1.pdf",
        "assets/samples/pdf_text_chart_1.pdf",
    ];
    let output = "output/test_merged.pdf";

    // Ensure output directory exists
    let _ = fs::create_dir_all("output");

    // Execute Merge
    let result = merge_multiple_pdfs(&inputs, output);
    assert!(result.is_ok(), "Merge function failed: {:?}", result.err());

    // Verify Page Count
    // Calculate the expected sum of pages from inputs
    let mut expected_page_count = 0;
    for path in &inputs {
        let doc = Document::load(path).expect("Failed to load input for verification");
        expected_page_count += doc.get_pages().len();
    }

    let merged_doc = Document::load(output).expect("Failed to load merged output");
    let actual_page_count = merged_doc.get_pages().len();

    assert_eq!(
        actual_page_count, expected_page_count,
        "Merged PDF page count ({}) does not match sum of inputs ({})",
        actual_page_count, expected_page_count
    );

    // Verify File Size
    // Merged PDFs are often shorter than the sum because 
    // fonts and shared resources can be deduplicated or compressed.
    let total_input_size: u64 = inputs
        .iter()
        .map(|p| fs::metadata(p).unwrap().len())
        .sum();
    
    let output_size = fs::metadata(output)
        .expect("Failed to get output metadata")
        .len();

    assert!(
        output_size <= total_input_size,
        "Merged file size ({} bytes) is unexpectedly larger than the sum of inputs ({} bytes)",
        output_size, total_input_size
    );

    // Clean up (leave it to inspect the result of the merge manually)
    fs::remove_file(output).unwrap();
}