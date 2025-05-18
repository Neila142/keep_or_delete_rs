fn main() {
    println!("Hello, world!");
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
}
