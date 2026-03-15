#![allow(non_camel_case_types)]
// File operations — open and save Markdown documents.

use std::path::Path;

/// Read a Markdown file from disk.
pub fn read_file(path: &Path) -> Result<String, FileError_gtw> {
    std::fs::read_to_string(path).map_err(|e| FileError_gtw::ReadFailed {
        path: path.to_string_lossy().into_owned(),
        source: e,
    })
}

/// Write a Markdown string to disk.
pub fn write_file(path: &Path, content: &str) -> Result<(), FileError_gtw> {
    std::fs::write(path, content).map_err(|e| FileError_gtw::WriteFailed {
        path: path.to_string_lossy().into_owned(),
        source: e,
    })
}

/// File IO errors.
#[derive(Debug, thiserror::Error)]
pub enum FileError_gtw {
    /// Failed to read a file.
    #[error("failed to read {path}: {source}")]
    ReadFailed {
        /// File path that failed.
        path: String,
        /// Underlying IO error.
        source: std::io::Error,
    },
    /// Failed to write a file.
    #[error("failed to write {path}: {source}")]
    WriteFailed {
        /// File path that failed.
        path: String,
        /// Underlying IO error.
        source: std::io::Error,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn read_write_roundtrip() {
        let dir = std::env::temp_dir().join("rte_test_gateway");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.md");

        let content = "# Hello\n\nWorld\n";
        write_file(&path, content).unwrap();

        let read_back = read_file(&path).unwrap();
        assert_eq!(read_back, content);

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn read_nonexistent_file() {
        let read_result = read_file(Path::new("/nonexistent/file.md"));
        assert!(read_result.is_err());
    }
}
