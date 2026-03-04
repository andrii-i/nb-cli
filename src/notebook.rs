use anyhow::{Context, Result};
use nbformat::v4::Notebook;
use std::fs;
use std::path::Path;

/// Read a Jupyter notebook from a file
pub fn read_notebook(path: impl AsRef<Path>) -> Result<Notebook> {
    let content = fs::read_to_string(&path)
        .context("Failed to read notebook file")?;

    let notebook = nbformat::parse_notebook(&content)
        .context("Failed to parse notebook")?;

    match notebook {
        nbformat::Notebook::V4(nb) => Ok(nb),
        _ => anyhow::bail!("Only nbformat v4 notebooks are supported"),
    }
}

/// Write a Jupyter notebook to a file
pub fn write_notebook(path: impl AsRef<Path>, notebook: &Notebook) -> Result<()> {
    let notebook_enum = nbformat::Notebook::V4(notebook.clone());
    let content = serialize_notebook(&notebook_enum)?;
    fs::write(path, content)?;
    Ok(())
}

/// Serialize a notebook to JSON string
fn serialize_notebook(notebook: &nbformat::Notebook) -> Result<String> {
    nbformat::serialize_notebook(notebook)
        .context("Failed to serialize notebook")
}
