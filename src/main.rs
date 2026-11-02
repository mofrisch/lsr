//! Minimal Unix-style `ls`: list directory entry names.
//!
//! Usage: rust-ls [PATH]
//! Defaults to the current directory when PATH is omitted.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process;

fn list_dir(path: &Path) -> io::Result<()> {
    let mut entries: Vec<_> = fs::read_dir(path)?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<_, _>>()?;

    entries.sort_unstable();

    let stdout = io::stdout();
    let mut out = stdout.lock();
    for name in entries {
        writeln!(out, "{}", name.to_string_lossy())?;
    }
    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().unwrap_or_else(|| ".".to_string());

    if args.next().is_some() {
        eprintln!("usage: rust-ls [PATH]");
        process::exit(2);
    }

    let path = Path::new(&path);
    if let Err(err) = list_dir(path) {
        eprintln!("rust-ls: {}: {}", path.display(), err);
        process::exit(1);
    }
}
