use thiserror::Error;

#[derive(Error, Debug)]
pub enum MergeError {
    #[error("No input PDFs provided")]
    NoInputFiles,
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("PDF Processing Error: {0}")]
    Pdf(#[from] lopdf::Error),
    #[error("Missing expected PDF object: {0}")]
    MissingObject(String),
}