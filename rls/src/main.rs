use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    let input = buffer.trim();

    let path = Path::new(input);
    if !path.is_dir() {
        let error_message = format!("'{}' is not a valid directory.", path.display());
        let custom_error = io::Error::new(io::ErrorKind::NotFound, error_message);
        return Err(custom_error);
    }
    let entries = fs::read_dir(path)?;
    println!("Entries in '{}'", path.display());

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
    Ok(())
}
