use std::env::{args, current_dir};
use std::io::{Error, ErrorKind, Result, stdin};
use std::path::{Path, PathBuf};
//Crates
use trash;
use url::Url;
use walkdir::WalkDir;

fn main() {
    cli_interface().expect("Error in the CLI interface");

    println!("Done, Thanks for using KeepOrDelete?");
}

fn detect_directory() -> Result<PathBuf> {
    let mut args: Vec<String> = args().collect();
    args.remove(0);

    // No argument specified use current_dir()
    if args.len() < 1 {
        return Ok(current_dir()?);
    }

    // More than one argument specified, not yet supported
    if args.len() > 1 {
        // TODO Support more than one path
        println!("More than one argument provided, not yet supported");
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "More than one argument provided, not yet supported",
        ));
    }

    // Only one argument find, process
    let path = Path::new(&args[0]);

    if !path.exists() {
        println!("Path doesn't exist");
        return Err(Error::new(ErrorKind::NotFound, "Path doesn't exist"));
    }

    if !path.is_dir() {
        println!("{} is not a directory", path.display());
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "Path is not a directory",
        ));
    }

    Ok(path.to_path_buf())
}

fn format_filename_with_link(path: &Path) -> Result<String> {
    let canon_path = path.canonicalize()?;
    let uri = Url::from_file_path(&canon_path)
        .expect("Failed to create URI")
        .to_string();

    #[cfg(windows)]
    let temp_buf = simplify_windows_filename_for_view(canon_path)?;
    let clean_path: &Path = &PathBuf::as_path(&temp_buf);
    #[cfg(not(windows))]
    let clean_path: &Path = &canon_path;

    let link = format!(
        "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
        uri,
        clean_path.display()
    );

    Ok(link)
}

fn simplify_windows_filename_for_view(path: PathBuf) -> Result<PathBuf> {
    let clean_path: PathBuf = {
        let s = path.to_string_lossy();
        let trimmed = if s.starts_with(r#"\\?\"#) {
            &s[4..]
        } else {
            &s
        };
        PathBuf::from(trimmed)
    };

    Ok(clean_path)
}

fn cli_interface() -> Result<()> {
    let working_dir = detect_directory().expect("Failed to detect directory");

    let mut files_iterator = WalkDir::new(working_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .peekable();

    if files_iterator.peek().is_none() {
        println!("No files found in the directory");
        return Ok(());
    }
    
    for entry in files_iterator {
        println!("Starting reading files");

        cli_process_entry(&entry)?;
    }

    Ok(())
}

fn cli_process_entry(entry: &walkdir::DirEntry) -> Result<()> {
    let link = format_filename_with_link(entry.path())?;

    println!("File: \n{}\n", link);
    println!("Action? [D]elete/[K]eep (default: keep)");

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
       "" | "k" => {
            println!("Kept: {}", entry.path().display())
        },
        "d" => {
            trash::delete(entry.path()).unwrap();
            println!("Deleted: {}", entry.path().display());
        }
        _ => println!("Invalid input"),
    }
    
    Ok(())
}
