use pyo3::prelude::*;

pub mod error;
pub mod pdf;

use crate::pdf::merge::merge_multiple_pdfs;
use std::path::PathBuf;

// This is the function Python will call
#[pyfunction]
fn run_merge(inputs: Vec<String>, output: String) -> PyResult<String> {
    let input_paths: Vec<PathBuf> = inputs.iter().map(PathBuf::from).collect();
    let output_path = PathBuf::from(output);

    merge_multiple_pdfs(&input_paths, output_path)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    Ok("Success".to_string())
}

// This name 'mergepdf' MUST match the [lib] name in Cargo.toml
#[pymodule]
fn mergepdf(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run_merge, m)?)?;
    Ok(())
}