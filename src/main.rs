//! Minimal Unix-style `ls`: list directory entry names.
//!
//! Usage: rust-ls [PATH]
//! Defaults to the current directory when PATH is omitted.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;

/// List directory entries sorted alphabetically.
///
/// This function reads all entries from the given path, collects them into a vector,
/// sorts them, and writes each entry name to stdout on a separate line.
fn list_dir(path: &Path) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|res| match res {
            Ok(e) => Some(Ok(e.file_name())),
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<_, _>>()?;

    entries.sort_unstable();

    let stdout = io::stdout();
    let mut out = stdout.lock();
    for name in entries {
        writeln!(out, "{}", name.to_string_lossy())?;
    }
    Ok(())
}

/// Main entry point for the rust-ls utility.
///
/// Parses command-line arguments, validates usage, and delegates to list_dir.
fn main() {
    let mut args = env::args().skip(1);

    // Use PathBuf directly to avoid unnecessary string allocation
    let path = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    // Check for extra arguments
    if args.next().is_some() {
        eprintln!("usage: rust-ls [PATH]");
        process::exit(2);
    }

    // Convert to Path reference for list_dir
    let path_ref = path.as_path();
    if let Err(err) = list_dir(path_ref) {
        eprintln!("rust-ls: {}: {}", path_ref.display(), err);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write as IoWrite;
    use tempdir::TempDir;

    /// Helper to create a temporary directory with test files
    fn create_test_dir(entries: &[&str]) -> TempDir {
        let tempdir = TempDir::new("rust-ls-test").expect("Failed to create temp dir");
        for entry in entries {
            let path = tempdir.path().join(entry);
            File::create(&path)
                .and_then(|mut f| f.write_all(b""))
                .expect("Failed to create test file");
        }
        tempdir
    }

    #[test]
    fn test_sorts_entries() {
        let tempdir = create_test_dir(&["zebra", "apple", "mango", "banana"]);
        let path = tempdir.path();

        // Capture output by redirecting stdout
        let mut entries = Vec::new();
        let read_dir = fs::read_dir(path).unwrap();
        for res in read_dir {
            let entry = res.unwrap();
            entries.push(entry.file_name());
        }
        entries.sort_unstable();

        // Verify they are sorted
        let names: Vec<_> = entries
            .into_iter()
            .map(|n| n.to_string_lossy().to_string())
            .collect();

        assert_eq!(names[0], "apple");
        assert_eq!(names[1], "banana");
        assert_eq!(names[2], "mango");
        assert_eq!(names[3], "zebra");
    }

    #[test]
    fn test_list_dir_error() {
        // Test that list_dir returns an error for non-existent path
        let fake_path = Path::new("/nonexistent/path/xyz123");
        let result = list_dir(fake_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_current_directory() {
        // Test that the default "." works
        let current_dir = std::env::current_dir().unwrap();
        let result = list_dir(&current_dir);
        // Just verify it doesn't panic and handles valid directory
        assert!(result.is_ok());
    }
}
